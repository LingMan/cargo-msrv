use crate::{Config, Output, TResult};

pub(crate) mod doctor;
pub(crate) mod find;
pub(crate) mod list;
pub(crate) mod set;
pub(crate) mod show;
pub(crate) mod verify;

/// Allows users to check whether the MSRV of a crate is proper.
///
/// Use case:
///
/// * Run `cargo msrv verify` on the CI, to verify the crates MSRV is acceptable.
pub use {find::Find, list::List, set::Set, show::Show, verify::Verify};

/// A sub-command of `cargo-msrv`.
///
/// It takes a set of inputs, from the `config`, and reports it's results via the `reporter`.
pub trait SubCommand {
    /// Run the sub-command
    fn run<R: Output>(&self, config: &Config, reporter: &R) -> TResult<()>;
}
