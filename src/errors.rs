use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fmt::Formatter;
use std::io;
use std::path::PathBuf;
use std::string::FromUtf8Error;

use crate::fetch::ToolchainSpecifier;

pub type TResult<T> = Result<T, CargoMSRVError>;

#[derive(Debug)]
pub enum CargoMSRVError {
    BareVersionParse(crate::manifest::bare_version::Error),
    CargoMetadata(cargo_metadata::Error),
    DefaultHostTripleNotFound,
    Env(env::VarError),
    GenericMessage(String),
    Io(io::Error, IoErrorSource),
    InvalidConfig(String),
    InvalidRustVersionNumber(std::num::ParseIntError),
    InvalidUTF8(FromUtf8Error),
    NoCrateRootFound,
    NoVersionMatchesManifestMSRV(
        crate::manifest::bare_version::BareVersion,
        Vec<crate::semver::Version>,
    ),
    NoMSRVKeyInCargoToml(PathBuf),
    ParseToml(toml_edit::TomlError),
    RustReleasesSource(rust_releases::RustChangelogError),
    RustReleasesRustDistSource(rust_releases::RustDistError),
    RustReleasesSourceParseError(String),
    RustupInstallFailed(ToolchainSpecifier),
    RustupRunWithCommandFailed,
    SemverError(rust_releases::semver::Error),
    SystemTime(std::time::SystemTimeError),
    ToolchainNotInstalled,
    UnknownTarget,
    UnableToAccessLogFolder,
    UnableToCacheChannelManifest,
    UnableToFindAnyGoodVersion {
        command: String,
    },
    UnableToInitTracing,
    UnableToParseCliArgs,
    UnableToParseRustVersion,
    UnableToRunCheck,
}

impl fmt::Display for CargoMSRVError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            CargoMSRVError::BareVersionParse(err) => write!(f, "Unable to parse minimum rust version: {}", err) ,
            CargoMSRVError::CargoMetadata(err) => err.fmt(f),
            CargoMSRVError::DefaultHostTripleNotFound => write!(f, "The default host triple (target) could not be found."),
            CargoMSRVError::Env(err) => err.fmt(f),
            CargoMSRVError::GenericMessage(msg) => write!(f, "{}", msg.as_str()),
            CargoMSRVError::Io(err, source) => write!(f, "IO error: '{}'. caused by: '{}'.", err, source),
            CargoMSRVError::InvalidConfig(msg) => write!(f, "{}", msg),
            CargoMSRVError::InvalidRustVersionNumber(err) => err.fmt(f),
            CargoMSRVError::InvalidUTF8(err) => err.fmt(f),
            CargoMSRVError::NoCrateRootFound => write!(f, "No crate root found for given crate"),
            CargoMSRVError::NoVersionMatchesManifestMSRV(msrv, versions_available) => write!(f, "The MSRV requirement ({}) in the Cargo manifest did not match any available version, available: {}", msrv, versions_available.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", ")),
            CargoMSRVError::NoMSRVKeyInCargoToml(path) => write!(f, "Unable to find key 'package.metadata.msrv' in '{}'", path.display()),
            CargoMSRVError::ParseToml(err) => f.write_fmt(format_args!("Unable to parse Cargo.toml {:?}", err)),
            CargoMSRVError::RustReleasesSource(err) => err.fmt(f),
            CargoMSRVError::RustReleasesRustDistSource(err) => err.fmt(f),
            CargoMSRVError::RustReleasesSourceParseError(err) => write!(f, "Unable to parse rust-releases source from '{}'", err),
            CargoMSRVError::RustupInstallFailed(toolchain) => f.write_fmt(format_args!("Unable to install toolchain with `rustup install {}`.", toolchain)),
            CargoMSRVError::RustupRunWithCommandFailed => write!(f, "Check toolchain (with `rustup run <toolchain> <command>`) failed."),
            CargoMSRVError::SemverError(err) => write!(f, "{}", err),
            CargoMSRVError::SystemTime(err) => err.fmt(f),
            CargoMSRVError::ToolchainNotInstalled => write!(f, "The given toolchain could not be found. Run `rustup toolchain list` for an overview of installed toolchains."),
            CargoMSRVError::UnknownTarget => write!(f, "The given target could not be found. Run `rustup target list` for an overview of available toolchains."),
            CargoMSRVError::UnableToAccessLogFolder => write!(f, "Unable to access log folder, run with --no-log to try again without logging."),
            CargoMSRVError::UnableToCacheChannelManifest => write!(f, "Unable to get or store the channel manifest on disk."),
            CargoMSRVError::UnableToInitTracing => write!(f, "Unable to init logger, run with --no-log to try again without logging."),
            CargoMSRVError::UnableToFindAnyGoodVersion { command } => write!(f, r#"Unable to find a Minimum Supported Rust Version (MSRV).

If you think this result is erroneous, please run: `{}` manually.

If the above does succeed, or you think cargo-msrv errored in another way, please feel free to
report the issue at: https://github.com/foresterre/cargo-msrv/issues

Thank you in advance!"#, command.as_str()),
            CargoMSRVError::UnableToParseCliArgs => write!(f, "Unable to parse the CLI arguments. Use `cargo msrv help` for more info."),
            CargoMSRVError::UnableToParseRustVersion => write!(f, "The Rust stable version could not be parsed from the stable channel manifest."),
            CargoMSRVError::UnableToRunCheck => write!(f, "Unable to run the checking command. If --check <cmd> is specified, you could try to verify if you can run the cmd manually." )
        }
    }
}

#[derive(Debug)]
pub enum IoErrorSource {
    CurrentDir,
    ReadFile { path: PathBuf },
    WriteFile { path: PathBuf },
    RemoveFile { path: PathBuf },
    RenameFile { path: PathBuf },
    SpawnProcess { name: OsString },
    WaitForProcessAndCollectOutput { name: OsString },
}

impl fmt::Display for IoErrorSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::CurrentDir => write!(f, "Unable to determine current working directory"),
            Self::ReadFile { path } => write!(f, "Unable to read file '{}'", path.display()),
            Self::WriteFile { path } => write!(f, "Unable to write file '{}'", path.display()),
            Self::RemoveFile { path } => write!(f, "Unable to remove file '{}'", path.display()),
            Self::RenameFile { path } => write!(f, "Unable to rename file '{}'", path.display()),
            Self::SpawnProcess { name } => write!(f, "Unable to spawn process '{:?}'", name),
            Self::WaitForProcessAndCollectOutput { name } => {
                write!(
                    f,
                    "Unable to collect output from '{:?}', or process did not terminate properly",
                    name
                )
            }
        }
    }
}

impl Error for CargoMSRVError {}

impl From<String> for CargoMSRVError {
    fn from(msg: String) -> Self {
        CargoMSRVError::GenericMessage(msg)
    }
}

impl From<cargo_metadata::Error> for CargoMSRVError {
    fn from(err: cargo_metadata::Error) -> Self {
        CargoMSRVError::CargoMetadata(err)
    }
}

impl From<env::VarError> for CargoMSRVError {
    fn from(err: env::VarError) -> Self {
        CargoMSRVError::Env(err)
    }
}

impl From<FromUtf8Error> for CargoMSRVError {
    fn from(err: FromUtf8Error) -> Self {
        CargoMSRVError::InvalidUTF8(err)
    }
}

impl From<std::num::ParseIntError> for CargoMSRVError {
    fn from(err: std::num::ParseIntError) -> Self {
        CargoMSRVError::InvalidRustVersionNumber(err)
    }
}

impl From<toml_edit::TomlError> for CargoMSRVError {
    fn from(err: toml_edit::TomlError) -> Self {
        CargoMSRVError::ParseToml(err)
    }
}

impl From<rust_releases::semver::Error> for CargoMSRVError {
    fn from(err: rust_releases::semver::Error) -> Self {
        CargoMSRVError::SemverError(err)
    }
}

impl From<std::time::SystemTimeError> for CargoMSRVError {
    fn from(err: std::time::SystemTimeError) -> Self {
        CargoMSRVError::SystemTime(err)
    }
}

impl From<rust_releases::RustChangelogError> for CargoMSRVError {
    fn from(err: rust_releases::RustChangelogError) -> Self {
        CargoMSRVError::RustReleasesSource(err)
    }
}

impl From<rust_releases::RustDistError> for CargoMSRVError {
    fn from(err: rust_releases::RustDistError) -> Self {
        CargoMSRVError::RustReleasesRustDistSource(err)
    }
}

impl From<crate::manifest::bare_version::Error> for CargoMSRVError {
    fn from(err: crate::manifest::bare_version::Error) -> Self {
        CargoMSRVError::BareVersionParse(err)
    }
}
