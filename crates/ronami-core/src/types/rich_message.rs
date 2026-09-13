use serde::{Deserialize, Serialize};

use crate::types::RichBlock;

/// Rich formatted message.
///
/// [The official docs](https://core.telegram.org/bots/api#richmessage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichMessage {
    /// Content of the message.
    pub blocks: Vec<RichBlock>,

    /// `true`, if the rich message must be shown right-to-left.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_rtl: bool,
}

impl RichMessage {
    pub fn new(blocks: impl Into<Vec<RichBlock>>) -> Self {
        Self { blocks: blocks.into(), is_rtl: false }
    }

    #[must_use]
    pub fn is_rtl(mut self, val: bool) -> Self {
        self.is_rtl = val;
        self
    }
}
