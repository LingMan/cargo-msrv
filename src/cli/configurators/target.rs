use crate::cli::configurators::Configure;
use crate::cli::CargoMsrvOpts;
use crate::config::ConfigBuilder;
use crate::TResult;

pub(in crate::cli) struct Target;

impl Configure for Target {
    fn configure(builder: ConfigBuilder, opts: &CargoMsrvOpts) -> TResult<ConfigBuilder> {
        // TODO{foresterre}: maybe also for `verify`, not just `find`?
        if let Some(target) = &opts.find_opts.toolchain_opts.target {
            Ok(builder.target(target.as_str()))
        } else {
            Ok(builder)
        }
    }
}
