use serde::{Deserialize, Serialize};

use crate::types::{InputRichBlock, InputRichMessageMedia};

/// Describes a rich message to be sent.
///
/// Exactly one of the fields `html`, `markdown`, or `blocks` must be used.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichmessage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichMessage {
    /// Content of the rich message to send described using HTML formatting.
    pub html: Option<String>,

    /// Content of the rich message to send described using Markdown formatting.
    pub markdown: Option<String>,

    /// Content of the rich message to send described as a list of blocks.
    pub blocks: Option<Vec<InputRichBlock>>,

    /// List of media that are specified in the markdown or html fields using
    /// `tg://photo?id=`, `tg://video?id=`, `tg://document?id=`, and
    /// `tg://audio?id=` links.
    pub media: Option<Vec<InputRichMessageMedia>>,

    /// Pass `true` if the rich message must be shown right-to-left.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_rtl: bool,

    /// Pass `true` to skip automatic detection of entities (e.g., URLs, email
    /// addresses, username mentions, hashtags, cashtags, bot commands, or phone
    /// numbers) in the text.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub skip_entity_detection: bool,
}

impl InputRichMessage {
    pub fn html(html: impl Into<String>) -> Self {
        Self {
            html: Some(html.into()),
            markdown: None,
            blocks: None,
            media: None,
            is_rtl: false,
            skip_entity_detection: false,
        }
    }

    pub fn markdown(markdown: impl Into<String>) -> Self {
        Self {
            html: None,
            markdown: Some(markdown.into()),
            blocks: None,
            media: None,
            is_rtl: false,
            skip_entity_detection: false,
        }
    }

    pub fn blocks(blocks: impl IntoIterator<Item = InputRichBlock>) -> Self {
        Self {
            html: None,
            markdown: None,
            blocks: Some(blocks.into_iter().collect()),
            media: None,
            is_rtl: false,
            skip_entity_detection: false,
        }
    }

    #[must_use]
    pub fn media(mut self, media: impl IntoIterator<Item = InputRichMessageMedia>) -> Self {
        self.media = Some(media.into_iter().collect());
        self
    }

    #[must_use]
    pub fn is_rtl(mut self, val: bool) -> Self {
        self.is_rtl = val;
        self
    }

    #[must_use]
    pub fn skip_entity_detection(mut self, val: bool) -> Self {
        self.skip_entity_detection = val;
        self
    }
}
