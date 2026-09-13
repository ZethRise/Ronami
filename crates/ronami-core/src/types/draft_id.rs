use derive_more::From;
use serde::{Deserialize, Serialize};

/// Unique identifier of a message draft.
#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize, From)]
#[derive(derive_more::Display)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct DraftId(pub i64);

impl DraftId {
    /// Checks if the draft identifier is valid (non-zero per Telegram
    /// documentation).
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 != 0
    }
}
