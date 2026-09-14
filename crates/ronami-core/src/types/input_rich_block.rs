use serde::{Deserialize, Serialize};

use crate::types::{
    InputMediaAnimation, InputMediaAudio, InputMediaPhoto, InputMediaVideo, InputMediaVoiceNote,
    Location, RichBlockCaption, RichBlockTableCell, RichText,
};

/// This object represents a block in a rich formatted message to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblock).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputRichBlock {
    Paragraph(InputRichBlockParagraph),
    #[serde(rename = "heading")]
    SectionHeading(InputRichBlockSectionHeading),
    #[serde(rename = "pre")]
    Preformatted(InputRichBlockPreformatted),
    Footer(InputRichBlockFooter),
    Divider(InputRichBlockDivider),
    MathematicalExpression(InputRichBlockMathematicalExpression),
    Anchor(InputRichBlockAnchor),
    List(InputRichBlockList),
    #[serde(rename = "blockquote")]
    BlockQuotation(InputRichBlockBlockQuotation),
    #[serde(rename = "pullquote")]
    PullQuotation(InputRichBlockPullQuotation),
    Collage(InputRichBlockCollage),
    Slideshow(InputRichBlockSlideshow),
    Table(InputRichBlockTable),
    Details(InputRichBlockDetails),
    Map(InputRichBlockMap),
    Animation(InputRichBlockAnimation),
    Audio(InputRichBlockAudio),
    Photo(InputRichBlockPhoto),
    Video(InputRichBlockVideo),
    VoiceNote(InputRichBlockVoiceNote),
    Thinking(InputRichBlockThinking),
}

/// An item of a list to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblocklistitem).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockListItem {
    /// Content of the item.
    pub blocks: Vec<InputRichBlock>,

    /// Pass `true` if the item has a checkbox.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_checkbox: bool,

    /// Pass `true` if the item has a checked checkbox.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_checked: bool,

    /// For ordered lists, the numeric value of the item label.
    pub value: Option<i64>,

    /// For ordered lists, the type of the item label; must be one of "a" for
    /// lowercase letters, "A" for uppercase letters, "i" for lowercase Roman
    /// numerals, "I" for uppercase Roman numerals, or "1" for decimal numbers.
    pub r#type: Option<String>,
}

/// A text paragraph to be sent, corresponding to the HTML tag `<p>`.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockparagraph).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockParagraph {
    /// Text of the block.
    pub text: RichText,
}

/// A section heading to be sent, corresponding to the HTML tags `<h1>`-`<h6>`.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblocksectionheading).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockSectionHeading {
    /// Text of the block.
    pub text: RichText,

    /// Relative size of the text font; 1-6, 1 is the largest, 6 is the
    /// smallest.
    pub size: u8,
}

/// A preformatted text block to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockpreformatted).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockPreformatted {
    /// Text of the block.
    pub text: RichText,

    /// Programming language of the text.
    pub language: Option<String>,
}

/// A footer to be sent, corresponding to the HTML tag `<footer>`.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockfooter).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockFooter {
    /// Text of the block.
    pub text: RichText,
}

/// A divider to be sent, corresponding to the HTML tag `<hr/>`.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockdivider).
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockDivider {}

/// A block with a mathematical expression in LaTeX format to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockmathematicalexpression).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockMathematicalExpression {
    /// Mathematical expression in LaTeX format.
    pub expression: String,
}

/// A block with an anchor to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockanchor).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockAnchor {
    /// Name of the anchor.
    pub name: String,
}

/// A list of blocks to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblocklist).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockList {
    /// Items of the list.
    pub items: Vec<InputRichBlockListItem>,
}

/// A block quotation to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockblockquotation).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockBlockQuotation {
    /// Content of the block.
    pub blocks: Vec<InputRichBlock>,

    /// Credit of the block.
    pub credit: Option<RichText>,
}

/// A quotation with centered text to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockpullquotation).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockPullQuotation {
    /// Text of the block.
    pub text: RichText,

    /// Credit of the block.
    pub credit: Option<RichText>,
}

/// A collage to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockcollage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockCollage {
    /// Elements of the collage.
    pub blocks: Vec<InputRichBlock>,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// A slideshow to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockslideshow).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockSlideshow {
    /// Elements of the slideshow.
    pub blocks: Vec<InputRichBlock>,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// A table to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblocktable).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockTable {
    /// Cells of the table.
    pub cells: Vec<Vec<RichBlockTableCell>>,

    /// Pass `true` if the table has borders.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_bordered: bool,

    /// Pass `true` if the table is striped.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_striped: bool,

    /// Pass `true` if table cells must have smaller indents.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_compact: bool,

    /// Caption of the table.
    pub caption: Option<RichText>,
}

/// An expandable block for details disclosure to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockdetails).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockDetails {
    /// Always shown summary of the block.
    pub summary: RichText,

    /// Content of the block.
    pub blocks: Vec<InputRichBlock>,

    /// Pass `true` if the content of the block is visible by default.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_open: bool,
}

/// A block with a map to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockmap).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockMap {
    /// Location of the center of the map.
    pub location: Location,

    /// Map zoom level; 0-24.
    pub zoom: Option<u8>,

    /// Map width; 0-10000.
    pub width: Option<u32>,

    /// Map height; 0-10000.
    pub height: Option<u32>,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// A block with an animation to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockanimation).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockAnimation {
    /// The animation. Caption is ignored.
    pub animation: InputMediaAnimation,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// A block with a music file to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockaudio).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockAudio {
    /// The audio. Caption is ignored.
    pub audio: InputMediaAudio,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// A block with a photo to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockphoto).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockPhoto {
    /// The photo. Caption is ignored.
    pub photo: InputMediaPhoto,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// A block with a video to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockvideo).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockVideo {
    /// The video. Caption is ignored.
    pub video: InputMediaVideo,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// A block with a voice note to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockvoicenote).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockVoiceNote {
    /// The voice note. Caption is ignored.
    pub voice_note: InputMediaVoiceNote,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// A block with a "Thinking..." placeholder to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockthinking).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockThinking {
    /// Text of the block.
    pub text: RichText,
}
