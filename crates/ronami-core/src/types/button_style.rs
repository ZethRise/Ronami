use serde::{Deserialize, Serialize};

/// Style of the button.
///
/// [The official docs](https://core.telegram.org/bots/api#keyboardbutton).
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ButtonStyle {
    Danger,
    Success,
    Primary,
}
