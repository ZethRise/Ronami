use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::types::{
    CopyTextButton, DisabledButton, LoginUrl, RichText, SwitchInlineQueryChosenChat, WebAppInfo,
};

/// This object represents a button in a [`RichMessage`].
///
/// Exactly one of the fields other than `text` and `style` must be used to
/// specify the type of the button, see [`RichMessageButtonKind`].
///
/// [The official docs](https://core.telegram.org/bots/api#richmessagebutton).
///
/// [`RichMessage`]: crate::types::RichMessage
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichMessageButton {
    /// Text of the button.
    ///
    /// May contain only plain text, [`RichTextCustomEmoji`] and
    /// [`RichTextDateTime`] entities.
    ///
    /// [`RichTextCustomEmoji`]: crate::types::RichTextCustomEmoji
    /// [`RichTextDateTime`]: crate::types::RichTextDateTime
    pub text: RichText,

    /// Style of the button.
    pub style: Option<RichMessageButtonStyle>,

    #[serde(flatten)]
    pub kind: RichMessageButtonKind,
}

/// Style of a [`RichMessageButton`].
///
/// [The official docs](https://core.telegram.org/bots/api#richmessagebutton).
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum RichMessageButtonStyle {
    Danger,
    Success,
    Primary,

    /// The button is shown as a regular link without borders.
    ///
    /// Allowed only for callback buttons.
    Link,
}

/// The type of a [`RichMessageButton`].
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum RichMessageButtonKind {
    /// HTTP or `tg://` url to be opened when button is pressed.
    ///
    /// Links in the form of `tg://user?id=<user_id>` can be used to mention a
    /// user by their ID without using a username, if this is allowed by
    /// their privacy settings.
    Url(Url),

    /// Data to be sent in a [`CallbackQuery`] to the bot when button is
    /// pressed, 1-64 bytes.
    ///
    /// [`CallbackQuery`]: crate::types::CallbackQuery
    CallbackData(String),

    /// Description of the [Web App] that will be launched when the user presses
    /// the button. Available only in private chats between a user and the bot.
    /// Not supported for messages sent on behalf of a Telegram Business
    /// account.
    ///
    /// [Web App]: https://core.telegram.org/bots/webapps
    WebApp(WebAppInfo),

    /// An HTTPS URL used to automatically authorize the user. Not supported for
    /// ephemeral messages.
    LoginUrl(LoginUrl),

    /// If set, pressing the button will prompt the user to select one of their
    /// chats, open that chat and insert the bot's username and the specified
    /// inline query in the input field. Can be empty, in which case just the
    /// bot's username will be inserted.
    SwitchInlineQuery(String),

    /// If set, pressing the button will insert the bot's username and the
    /// specified inline query in the current chat's input field. Can be empty,
    /// in which case only the bot's username will be inserted.
    SwitchInlineQueryCurrentChat(String),

    /// If set, pressing the button will prompt the user to select one of their
    /// chats of the specified type, open that chat and insert the bot's
    /// username and the specified inline query in the input field.
    SwitchInlineQueryChosenChat(SwitchInlineQueryChosenChat),

    /// A button that copies the specified text to the clipboard.
    CopyText(CopyTextButton),

    /// If set, then the button is disabled and does nothing.
    Disabled(DisabledButton),
}

impl RichMessageButton {
    /// Creates a new button of the specified kind.
    pub fn new<T>(text: T, kind: RichMessageButtonKind) -> Self
    where
        T: Into<RichText>,
    {
        Self { text: text.into(), style: None, kind }
    }

    /// Sets the style of the button.
    #[must_use]
    pub fn style(mut self, val: RichMessageButtonStyle) -> Self {
        self.style = Some(val);
        self
    }

    /// Constructor for a button with [`Url`] kind.
    pub fn url<T>(text: T, url: Url) -> Self
    where
        T: Into<RichText>,
    {
        Self::new(text, RichMessageButtonKind::Url(url))
    }

    /// Constructor for a button with [`CallbackData`] kind.
    ///
    /// [`CallbackData`]: RichMessageButtonKind::CallbackData
    pub fn callback<T, C>(text: T, callback_data: C) -> Self
    where
        T: Into<RichText>,
        C: Into<String>,
    {
        Self::new(text, RichMessageButtonKind::CallbackData(callback_data.into()))
    }

    /// Constructor for a button with [`WebApp`] kind.
    ///
    /// [`WebApp`]: RichMessageButtonKind::WebApp
    pub fn web_app<T>(text: T, info: WebAppInfo) -> Self
    where
        T: Into<RichText>,
    {
        Self::new(text, RichMessageButtonKind::WebApp(info))
    }

    /// Constructor for a button with [`LoginUrl`] kind.
    ///
    /// [`LoginUrl`]: RichMessageButtonKind::LoginUrl
    pub fn login<T>(text: T, url: LoginUrl) -> Self
    where
        T: Into<RichText>,
    {
        Self::new(text, RichMessageButtonKind::LoginUrl(url))
    }

    /// Constructor for a button with [`SwitchInlineQuery`] kind.
    ///
    /// [`SwitchInlineQuery`]: RichMessageButtonKind::SwitchInlineQuery
    pub fn switch_inline_query<T, Q>(text: T, switch_inline_query: Q) -> Self
    where
        T: Into<RichText>,
        Q: Into<String>,
    {
        Self::new(text, RichMessageButtonKind::SwitchInlineQuery(switch_inline_query.into()))
    }

    /// Constructor for a button with [`SwitchInlineQueryCurrentChat`] kind.
    ///
    /// [`SwitchInlineQueryCurrentChat`]: RichMessageButtonKind::SwitchInlineQueryCurrentChat
    pub fn switch_inline_query_current_chat<T, Q>(text: T, switch_inline_query: Q) -> Self
    where
        T: Into<RichText>,
        Q: Into<String>,
    {
        Self::new(
            text,
            RichMessageButtonKind::SwitchInlineQueryCurrentChat(switch_inline_query.into()),
        )
    }

    /// Constructor for a button with [`CopyText`] kind.
    ///
    /// [`CopyText`]: RichMessageButtonKind::CopyText
    pub fn copy_text_button<T>(text: T, copy_text_button: CopyTextButton) -> Self
    where
        T: Into<RichText>,
    {
        Self::new(text, RichMessageButtonKind::CopyText(copy_text_button))
    }

    /// Constructor for a button with [`Disabled`] kind.
    ///
    /// [`Disabled`]: RichMessageButtonKind::Disabled
    pub fn disabled<T>(text: T) -> Self
    where
        T: Into<RichText>,
    {
        Self::new(text, RichMessageButtonKind::Disabled(DisabledButton {}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_callback_button() {
        let button =
            RichMessageButton::callback("Click", "data").style(RichMessageButtonStyle::Link);

        assert_eq!(
            serde_json::to_string(&button).unwrap(),
            r#"{"text":"Click","style":"link","callback_data":"data"}"#
        );
    }

    #[test]
    fn serialize_url_button_without_style() {
        let button = RichMessageButton::url("Open", Url::parse("https://example.com/").unwrap());

        assert_eq!(
            serde_json::to_string(&button).unwrap(),
            r#"{"text":"Open","url":"https://example.com/"}"#
        );
    }

    #[test]
    fn serialize_disabled_button() {
        let button = RichMessageButton::disabled("Nope");

        assert_eq!(serde_json::to_string(&button).unwrap(), r#"{"text":"Nope","disabled":{}}"#);
    }

    #[test]
    fn deserialize_button() {
        let button: RichMessageButton =
            serde_json::from_str(r#"{"text":"Copy","style":"primary","copy_text":{"text":"hi"}}"#)
                .unwrap();

        assert_eq!(
            button,
            RichMessageButton::copy_text_button("Copy", CopyTextButton { text: "hi".to_owned() })
                .style(RichMessageButtonStyle::Primary)
        );
    }
}
