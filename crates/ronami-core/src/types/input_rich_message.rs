use serde::{Deserialize, Serialize};

/// Describes a rich message to be sent.
///
/// Exactly one of the fields `html` or `markdown` must be used.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichmessage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichMessage {
    /// Content of the rich message to send described using HTML formatting.
    pub html: Option<String>,

    /// Content of the rich message to send described using Markdown formatting.
    pub markdown: Option<String>,

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
            is_rtl: false,
            skip_entity_detection: false,
        }
    }

    pub fn markdown(markdown: impl Into<String>) -> Self {
        Self {
            html: None,
            markdown: Some(markdown.into()),
            is_rtl: false,
            skip_entity_detection: false,
        }
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
