use crate::event::{Event, Message};
use cargo_msrv_types::release_source::ReleaseSource;

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub struct FetchIndex {
    #[serde(rename = "source")]
    from_source: ReleaseSource,
}

impl FetchIndex {
    pub fn new(source: ReleaseSource) -> Self {
        Self {
            from_source: source,
        }
    }
}

impl From<FetchIndex> for Event {
    fn from(it: FetchIndex) -> Self {
        Message::FetchIndex(it).into()
    }
}
