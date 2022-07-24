use crate::check::Check;
use crate::check_cmd::CheckCommand;
use crate::command::RustupCommand;
use crate::config::SelectedCheckCommand;
use crate::download::{DownloadToolchain, ToolchainDownloader};
use crate::error::IoErrorSource;
use crate::lockfile::{LockfileHandler, CARGO_LOCK};
use crate::reporter::event::{CheckToolchain, Compatibility, CompatibilityCheckMethod, Method};
use crate::toolchain::ToolchainSpec;
use crate::{CargoMSRVError, Config, Outcome, Reporter, TResult};
use once_cell::unsync::OnceCell;
use std::path::{Path, PathBuf};

pub struct RustupToolchainCheck<'reporter, R: Reporter> {
    reporter: &'reporter R,
    lockfile_path: OnceCell<PathBuf>,
}

impl<'reporter, R: Reporter> Check for RustupToolchainCheck<'reporter, R> {
    fn check(&self, config: &Config, toolchain: &ToolchainSpec) -> TResult<Outcome> {
        self.reporter
            .run_scoped_event(CheckToolchain::new(toolchain.to_owned()), || {
                info!(ignore_lockfile_enabled = config.ignore_lockfile());

                // temporarily move the lockfile if the user opted to ignore it, and it exists
                let cargo_lock = self.lockfile_path(config)?;

                let handle_wrap = if config.ignore_lockfile() && cargo_lock.is_file() {
                    let handle = LockfileHandler::new(cargo_lock).move_lockfile()?;

                    Some(handle)
                } else {
                    None
                };

                self.prepare(toolchain, config)?;

                let outcome = self.run_check_command_via_rustup(
                    toolchain,
                    config.crate_path(),
                    config.check_command(),
                )?;

                // report outcome to UI
                self.report_outcome(&outcome, config.no_check_feedback())?;

                // move the lockfile back
                if let Some(handle) = handle_wrap {
                    handle.move_lockfile_back()?;
                }

                Ok(outcome)
            })
    }
}

impl<'reporter, R: Reporter> RustupToolchainCheck<'reporter, R> {
    pub fn new(reporter: &'reporter R) -> Self {
        Self {
            reporter,
            lockfile_path: OnceCell::new(),
        }
    }

    fn prepare(&self, toolchain: &ToolchainSpec, config: &Config) -> TResult<()> {
        let downloader = ToolchainDownloader::new(self.reporter);
        downloader.download(toolchain)?;

        if config.ignore_lockfile() {
            self.remove_lockfile(config)?;
        }

        Ok(())
    }

    fn run_check_command_via_rustup(
        &self,
        toolchain: &ToolchainSpec,
        dir: Option<&Path>,
        check_cmd: &SelectedCheckCommand,
    ) -> TResult<Outcome> {
        self.reporter.report_event(CompatibilityCheckMethod::new(
            toolchain.to_owned(),
            Method::rustup_run(check_cmd.to_string(), dir),
        ))?;

        let spec = toolchain.spec();
        let cmd = check_cmd.for_version(toolchain.version())?;
        let args = &[spec, cmd].join(" ");
        let rustup_output = RustupCommand::new()
            .with_args(args.split_ascii_whitespace())
            .with_optional_dir(dir)
            .with_stderr()
            .run()
            .map_err(|_| CargoMSRVError::UnableToRunCheck)?;

        let status = rustup_output.exit_status();

        if status.success() {
            Ok(Outcome::new_success(toolchain.to_owned()))
        } else {
            let stderr = rustup_output.stderr();
            let command = check_cmd.to_string();

            info!(
                ?toolchain,
                stderr,
                cmd = command.as_str(),
                "try_building run failed"
            );

            Ok(Outcome::new_failure(
                toolchain.to_owned(),
                stderr.to_string(),
            ))
        }
    }

    fn report_outcome(&self, outcome: &Outcome, no_error_report: bool) -> TResult<()> {
        match outcome {
            Outcome::Success(outcome) => {
                // report compatibility with this toolchain
                self.reporter
                    .report_event(Compatibility::compatible(outcome.toolchain_spec.to_owned()))?
            }
            Outcome::Failure(outcome) if no_error_report => {
                // report incompatibility with this toolchain
                self.reporter.report_event(Compatibility::incompatible(
                    outcome.toolchain_spec.to_owned(),
                    None,
                ))?
            }
            Outcome::Failure(outcome) => {
                // report incompatibility with this toolchain
                self.reporter.report_event(Compatibility::incompatible(
                    outcome.toolchain_spec.to_owned(),
                    Some(outcome.error_message.clone()),
                ))?
            }
        };

        Ok(())
    }

    fn lockfile_path(&self, config: &Config) -> TResult<&Path> {
        let path = self.lockfile_path.get_or_try_init(|| {
            config
                .context()
                .crate_root_path()
                .map(|path| path.join(CARGO_LOCK))
        })?;

        Ok(path)
    }

    fn remove_lockfile(&self, config: &Config) -> TResult<()> {
        let lock_file = self.lockfile_path(config)?;

        if lock_file.is_file() {
            std::fs::remove_file(&lock_file).map_err(|error| CargoMSRVError::Io {
                error,
                source: IoErrorSource::RemoveFile(lock_file.to_path_buf()),
            })?;
        }

        Ok(())
    }
}
