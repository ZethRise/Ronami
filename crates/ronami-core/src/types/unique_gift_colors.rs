use serde::{Deserialize, Serialize};

use crate::types::{CustomEmojiId, Rgb};

/// Color scheme for a user's name, message replies and link previews based on a
/// unique gift.
///
/// [The official docs](https://core.telegram.org/bots/api#uniquegiftcolors).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UniqueGiftColors {
    /// Custom emoji identifier of the unique gift's model
    pub model_custom_emoji_id: CustomEmojiId,

    /// Custom emoji identifier of the unique gift's symbol
    pub symbol_custom_emoji_id: CustomEmojiId,

    /// Main color used in light themes; RGB format
    pub light_theme_main_color: Rgb,

    /// List of 1-3 additional colors used in light themes; RGB format
    pub light_theme_other_colors: Vec<Rgb>,

    /// Main color used in dark themes; RGB format
    pub dark_theme_main_color: Rgb,

    /// List of 1-3 additional colors used in dark themes; RGB format
    pub dark_theme_other_colors: Vec<Rgb>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{
            "model_custom_emoji_id": "id1",
            "symbol_custom_emoji_id": "id2",
            "light_theme_main_color": 16711680,
            "light_theme_other_colors": [255],
            "dark_theme_main_color": 0,
            "dark_theme_other_colors": [1, 2]
        }"#;
        let colors: UniqueGiftColors = serde_json::from_str(json).unwrap();
        assert_eq!(colors.model_custom_emoji_id, CustomEmojiId("id1".into()));
        assert_eq!(colors.light_theme_main_color, Rgb { r: 255, g: 0, b: 0 });
        assert_eq!(colors.light_theme_other_colors.len(), 1);
    }
}
