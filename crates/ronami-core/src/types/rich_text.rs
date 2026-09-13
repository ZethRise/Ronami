use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::types::User;

/// This object represents a rich formatted text.
///
/// It can be either a plain string, an array of `RichText`, or a formatted rich
/// text entity.
///
/// [The official docs](https://core.telegram.org/bots/api#richtext).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum RichText {
    Plain(String),
    Array(Vec<RichText>),
    Entity(Box<RichTextEntity>),
}

impl RichText {
    pub fn plain(s: impl Into<String>) -> Self {
        Self::Plain(s.into())
    }

    pub fn bold(text: impl Into<RichText>) -> Self {
        Self::Entity(Box::new(RichTextEntity::Bold(RichTextBold { text: text.into() })))
    }

    pub fn italic(text: impl Into<RichText>) -> Self {
        Self::Entity(Box::new(RichTextEntity::Italic(RichTextItalic { text: text.into() })))
    }

    pub fn underline(text: impl Into<RichText>) -> Self {
        Self::Entity(Box::new(RichTextEntity::Underline(RichTextUnderline { text: text.into() })))
    }

    pub fn strikethrough(text: impl Into<RichText>) -> Self {
        Self::Entity(Box::new(RichTextEntity::Strikethrough(RichTextStrikethrough {
            text: text.into(),
        })))
    }

    pub fn spoiler(text: impl Into<RichText>) -> Self {
        Self::Entity(Box::new(RichTextEntity::Spoiler(RichTextSpoiler { text: text.into() })))
    }

    pub fn code(text: impl Into<RichText>) -> Self {
        Self::Entity(Box::new(RichTextEntity::Code(RichTextCode { text: text.into() })))
    }
}

impl From<String> for RichText {
    fn from(s: String) -> Self {
        Self::Plain(s)
    }
}

impl From<&str> for RichText {
    fn from(s: &str) -> Self {
        Self::Plain(s.to_owned())
    }
}

impl<T: Into<RichText>> From<Vec<T>> for RichText {
    fn from(v: Vec<T>) -> Self {
        Self::Array(v.into_iter().map(Into::into).collect())
    }
}

/// Formatted rich text entities.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichTextEntity {
    Bold(RichTextBold),
    Italic(RichTextItalic),
    Underline(RichTextUnderline),
    Strikethrough(RichTextStrikethrough),
    Spoiler(RichTextSpoiler),
    DateTime(RichTextDateTime),
    TextMention(RichTextTextMention),
    Subscript(RichTextSubscript),
    Superscript(RichTextSuperscript),
    Marked(RichTextMarked),
    Code(RichTextCode),
    CustomEmoji(RichTextCustomEmoji),
    MathematicalExpression(RichTextMathematicalExpression),
    Url(RichTextUrl),
    EmailAddress(RichTextEmailAddress),
    PhoneNumber(RichTextPhoneNumber),
    BankCardNumber(RichTextBankCardNumber),
    Mention(RichTextMention),
    Hashtag(RichTextHashtag),
    Cashtag(RichTextCashtag),
    BotCommand(RichTextBotCommand),
    Anchor(RichTextAnchor),
    AnchorLink(RichTextAnchorLink),
    Reference(RichTextReference),
    ReferenceLink(RichTextReferenceLink),
}

/// A bold text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextbold).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextBold {
    pub text: RichText,
}

/// An italicized text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextitalic).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextItalic {
    pub text: RichText,
}

/// An underlined text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextunderline).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextUnderline {
    pub text: RichText,
}

/// A strikethrough text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextstrikethrough).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextStrikethrough {
    pub text: RichText,
}

/// A text covered by a spoiler.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextspoiler).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextSpoiler {
    pub text: RichText,
}

/// Formatted date and time.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextdatetime).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextDateTime {
    pub text: RichText,
    pub unix_time: i64,
    pub date_time_format: String,
}

/// A mention of a Telegram user by their identifier.
///
/// [The official docs](https://core.telegram.org/bots/api#richtexttextmention).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextTextMention {
    pub text: RichText,
    pub user: User,
}

/// A subscript text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextsubscript).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextSubscript {
    pub text: RichText,
}

/// A superscript text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextsuperscript).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextSuperscript {
    pub text: RichText,
}

/// A marked text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextmarked).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextMarked {
    pub text: RichText,
}

/// A monowidth text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextcode).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextCode {
    pub text: RichText,
}

/// A custom emoji.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextcustomemoji).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextCustomEmoji {
    pub custom_emoji_id: String,
    pub alternative_text: String,
}

/// A mathematical expression.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextmathematicalexpression).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextMathematicalExpression {
    pub expression: String,
}

/// A text with a link.
///
/// [The official docs](https://core.telegram.org/bots/api#richtexturl).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextUrl {
    pub text: RichText,
    pub url: Url,
}

/// A text with an email address.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextemailaddress).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextEmailAddress {
    pub text: RichText,
    pub email_address: String,
}

/// A text with a phone number.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextphonenumber).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextPhoneNumber {
    pub text: RichText,
    pub phone_number: String,
}

/// A text with a bank card number.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextbankcardnumber).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextBankCardNumber {
    pub text: RichText,
    pub bank_card_number: String,
}

/// A mention by a username.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextmention).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextMention {
    pub text: RichText,
    pub username: String,
}

/// A hashtag.
///
/// [The official docs](https://core.telegram.org/bots/api#richtexthashtag).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextHashtag {
    pub text: RichText,
    pub hashtag: String,
}

/// A cashtag.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextcashtag).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextCashtag {
    pub text: RichText,
    pub cashtag: String,
}

/// A bot command.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextbotcommand).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextBotCommand {
    pub text: RichText,
    pub bot_command: String,
}

/// An anchor.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextanchor).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextAnchor {
    pub name: String,
}

/// A link to an anchor.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextanchorlink).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextAnchorLink {
    pub text: RichText,
    pub anchor_name: String,
}

/// A reference.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextreference).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextReference {
    pub text: RichText,
    pub name: String,
}

/// A link to a reference.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextreferencelink).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextReferenceLink {
    pub text: RichText,
    pub reference_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_serde() {
        let text = RichText::from("hello world");
        let json = serde_json::to_string(&text).unwrap();
        assert_eq!(json, r#""hello world""#);

        let de: RichText = serde_json::from_str(&json).unwrap();
        assert_eq!(de, text);
    }

    #[test]
    fn bold_text_serde() {
        let text = RichText::bold("bold text");
        let json = serde_json::to_string(&text).unwrap();
        assert_eq!(json, r#"{"type":"bold","text":"bold text"}"#);

        let de: RichText = serde_json::from_str(&json).unwrap();
        assert_eq!(de, text);
    }

    #[test]
    fn array_rich_text_serde() {
        let text = RichText::Array(vec![RichText::from("Hello, "), RichText::bold("world!")]);
        let json = serde_json::to_string(&text).unwrap();
        assert_eq!(json, r#"["Hello, ",{"type":"bold","text":"world!"}]"#);

        let de: RichText = serde_json::from_str(&json).unwrap();
        assert_eq!(de, text);
    }
}
