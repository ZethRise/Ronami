use serde::{Deserialize, Serialize};

/// Describes a keyboard button to be used by a user of a Mini App.
///
/// [The official docs](https://core.telegram.org/bots/api#preparedkeyboardbutton).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PreparedKeyboardButton {
    /// Unique identifier of the keyboard button.
    pub id: String,
}

impl PreparedKeyboardButton {
    /// Creates a new `PreparedKeyboardButton`.
    pub fn new<T>(id: T) -> Self
    where
        T: Into<String>,
    {
        Self { id: id.into() }
    }
}
