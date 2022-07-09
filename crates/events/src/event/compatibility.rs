use crate::event::Event;
use crate::event::Message;
use cargo_msrv_types::toolchain::OwnedToolchainSpec;

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Compatibility {
    pub toolchain: OwnedToolchainSpec,
    decision: bool,
    pub compatibility_report: CompatibilityReport,
}

impl Compatibility {
    pub fn compatible(toolchain: impl Into<OwnedToolchainSpec>) -> Self {
        Self {
            toolchain: toolchain.into(),
            decision: true,
            compatibility_report: CompatibilityReport::Compatible,
        }
    }

    pub fn incompatible(toolchain: impl Into<OwnedToolchainSpec>, error: Option<String>) -> Self {
        Self {
            toolchain: toolchain.into(),
            decision: false,
            compatibility_report: CompatibilityReport::Incompatible {
                error: error.map(Into::into),
            },
        }
    }
}

impl From<Compatibility> for Event {
    fn from(it: Compatibility) -> Self {
        Message::Compatibility(it).into()
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityReport {
    Compatible,
    Incompatible { error: Option<String> },
}
