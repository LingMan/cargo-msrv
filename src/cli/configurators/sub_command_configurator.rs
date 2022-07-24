use crate::cli::configurators::Configure;
use crate::cli::{CargoMsrvOpts, ListOpts, SetOpts, SubCommand, VerifyOpts};
use crate::config::list::ListCmdConfig;
use crate::config::set::SetCmdConfig;
use crate::config::verify::VerifyCmdConfig;
use crate::config::{ConfigBuilder, SubCommandConfig};
use crate::TResult;

pub(in crate::cli) struct SubCommandConfigurator;

impl Configure for SubCommandConfigurator {
    fn configure(builder: ConfigBuilder, opts: &CargoMsrvOpts) -> TResult<ConfigBuilder> {
        if let Some(cmd) = &opts.subcommand {
            match cmd {
                SubCommand::List(opts) => {
                    return configure_list(builder, opts);
                }
                SubCommand::Set(opts) => {
                    return configure_set(builder, opts);
                }
                SubCommand::Verify(opts) => {
                    return configure_verify(builder, opts);
                }
                _ => {}
            }
        }

        if opts.verify {
            return configure_deprecated_verify_flag(builder);
        }

        Ok(builder)
    }
}

fn configure_list(builder: ConfigBuilder, opts: &ListOpts) -> TResult<ConfigBuilder> {
    let config = ListCmdConfig {
        variant: opts.variant,
    };

    let config = SubCommandConfig::ListConfig(config);
    Ok(builder.sub_command_config(config))
}

fn configure_set<'c>(builder: ConfigBuilder, opts: &SetOpts) -> TResult<ConfigBuilder> {
    let config = SetCmdConfig {
        msrv: opts.msrv.clone(),
    };

    let config = SubCommandConfig::SetConfig(config);
    Ok(builder.sub_command_config(config))
}

fn configure_verify(builder: ConfigBuilder, opts: &VerifyOpts) -> TResult<ConfigBuilder> {
    let config = VerifyCmdConfig {
        rust_version: opts.rust_version.clone(),
    };

    let config = SubCommandConfig::VerifyConfig(config);
    Ok(builder.sub_command_config(config))
}

fn configure_deprecated_verify_flag(builder: ConfigBuilder) -> TResult<ConfigBuilder> {
    let config = VerifyCmdConfig { rust_version: None };

    let config = SubCommandConfig::VerifyConfig(config);
    Ok(builder.sub_command_config(config))
}
