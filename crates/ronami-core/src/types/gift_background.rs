use serde::{Deserialize, Serialize};

use crate::types::Rgb;

/// Background of a gift.
///
/// [The official docs](https://core.telegram.org/bots/api#giftbackground).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct GiftBackground {
    /// Center color of the background in RGB format
    pub center_color: Rgb,

    /// Edge color of the background in RGB format
    pub edge_color: Rgb,

    /// Text color of the background in RGB format
    pub text_color: Rgb,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{"center_color":16711680,"edge_color":255,"text_color":0}"#;
        let bg: GiftBackground = serde_json::from_str(json).unwrap();
        assert_eq!(bg.center_color, Rgb { r: 255, g: 0, b: 0 });
        assert_eq!(bg.edge_color, Rgb { r: 0, g: 0, b: 255 });
        assert_eq!(bg.text_color, Rgb { r: 0, g: 0, b: 0 });
    }
}
