use serde::{Deserialize, Serialize};

use crate::types::{Animation, Audio, Location, PhotoSize, RichText, Video, Voice};

/// This object represents a block in a rich formatted message.
///
/// [The official docs](https://core.telegram.org/bots/api#richblock).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichBlock {
    Paragraph(RichBlockParagraph),
    #[serde(rename = "heading")]
    SectionHeading(RichBlockSectionHeading),
    #[serde(rename = "pre")]
    Preformatted(RichBlockPreformatted),
    Footer(RichBlockFooter),
    Divider(RichBlockDivider),
    MathematicalExpression(RichBlockMathematicalExpression),
    Anchor(RichBlockAnchor),
    List(RichBlockList),
    #[serde(rename = "blockquote")]
    BlockQuotation(RichBlockBlockQuotation),
    #[serde(rename = "pullquote")]
    PullQuotation(RichBlockPullQuotation),
    Collage(RichBlockCollage),
    Slideshow(RichBlockSlideshow),
    Table(RichBlockTable),
    Details(RichBlockDetails),
    Map(RichBlockMap),
    Animation(RichBlockAnimation),
    Audio(RichBlockAudio),
    Photo(RichBlockPhoto),
    Video(RichBlockVideo),
    VoiceNote(RichBlockVoiceNote),
    Thinking(RichBlockThinking),
}

/// Caption of a rich formatted block.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockcaption).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockCaption {
    /// Block caption.
    pub text: RichText,

    /// Block credit which corresponds to the HTML tag `<cite>`.
    pub credit: Option<RichText>,
}

/// Cell in a table.
///
/// [The official docs](https://core.telegram.org/bots/api#richblocktablecell).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockTableCell {
    /// Text in the cell. If omitted, then the cell is invisible.
    pub text: Option<RichText>,

    /// `true`, if the cell is a header cell.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_header: bool,

    /// The number of columns the cell spans if it is bigger than 1.
    pub colspan: Option<u32>,

    /// The number of rows the cell spans if it is bigger than 1.
    pub rowspan: Option<u32>,

    /// Horizontal cell content alignment ("left", "center", or "right").
    pub align: String,

    /// Vertical cell content alignment ("top", "middle", or "bottom").
    pub valign: String,
}

/// An item of a list.
///
/// [The official docs](https://core.telegram.org/bots/api#richblocklistitem).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockListItem {
    /// Label of the item.
    pub label: String,

    /// The content of the item.
    pub blocks: Vec<RichBlock>,

    /// `true`, if the item has a checkbox.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_checkbox: bool,

    /// `true`, if the item has a checked checkbox.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_checked: bool,

    /// For ordered lists, the numeric value of the item label.
    pub value: Option<i64>,

    /// For ordered lists, the type of the item label.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

/// A text paragraph, corresponding to the HTML tag `<p>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockparagraph).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockParagraph {
    /// Text of the block.
    pub text: RichText,
}

/// A section heading, corresponding to the HTML tags `<h1>` - `<h6>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblocksectionheading).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockSectionHeading {
    /// Text of the block.
    pub text: RichText,

    /// Relative size of the text font; 1-6, 1 is the largest, 6 is the
    /// smallest.
    pub size: u8,
}

/// A preformatted text block, corresponding to `<pre>` and `<code>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockpreformatted).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockPreformatted {
    /// Text of the block.
    pub text: RichText,

    /// The programming language of the text.
    pub language: Option<String>,
}

/// A footer, corresponding to the HTML tag `<footer>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockfooter).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockFooter {
    /// Text of the block.
    pub text: RichText,
}

/// A divider, corresponding to the HTML tag `<hr/>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockdivider).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockDivider {}

/// A block with a mathematical expression in LaTeX format.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockmathematicalexpression).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockMathematicalExpression {
    /// The mathematical expression in LaTeX format.
    pub expression: String,
}

/// A block with an anchor, corresponding to `<a name="...">`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockanchor).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockAnchor {
    /// The name of the anchor.
    pub name: String,
}

/// A list of blocks, corresponding to `<ul>` or `<ol>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblocklist).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockList {
    /// Items of the list.
    pub items: Vec<RichBlockListItem>,
}

/// A block quotation, corresponding to `<blockquote>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockblockquotation).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockBlockQuotation {
    /// Content of the block.
    pub blocks: Vec<RichBlock>,

    /// Credit of the block.
    pub credit: Option<RichText>,
}

/// A quotation with centered text, loosely corresponding to `<aside>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockpullquotation).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockPullQuotation {
    /// Text of the block.
    pub text: RichText,

    /// Credit of the block.
    pub credit: Option<RichText>,
}

/// A collage, corresponding to custom HTML tag `<tg-collage>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockcollage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockCollage {
    /// Elements of the collage.
    pub blocks: Vec<RichBlock>,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// A slideshow, corresponding to custom HTML tag `<tg-slideshow>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockslideshow).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockSlideshow {
    /// Elements of the slideshow.
    pub blocks: Vec<RichBlock>,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// A table, corresponding to `<table>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblocktable).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockTable {
    /// Cells of the table.
    pub cells: Vec<Vec<RichBlockTableCell>>,

    /// `true`, if the table has borders.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_bordered: bool,

    /// `true`, if the table is striped.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_striped: bool,

    /// Caption of the table.
    pub caption: Option<RichText>,
}

/// An expandable block for details disclosure, corresponding to `<details>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockdetails).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockDetails {
    /// Always shown summary of the block.
    pub summary: RichText,

    /// Content of the block.
    pub blocks: Vec<RichBlock>,

    /// `true`, if the content of the block is visible by default.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_open: bool,
}

/// A block with a map, corresponding to `<tg-map>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockmap).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockMap {
    /// Location of the center of the map.
    pub location: Location,

    /// Map zoom level; 13-20.
    pub zoom: u8,

    /// Expected width of the map.
    pub width: u32,

    /// Expected height of the map.
    pub height: u32,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// A block with an animation, corresponding to `<video>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockanimation).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockAnimation {
    /// The animation.
    pub animation: Animation,

    /// `true`, if the media preview is covered by a spoiler animation.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_spoiler: bool,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// A block with a music file, corresponding to `<audio>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockaudio).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockAudio {
    /// The audio.
    pub audio: Audio,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// A block with a photo, corresponding to `<photo>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockphoto).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockPhoto {
    /// Available sizes of the photo.
    pub photo: Vec<PhotoSize>,

    /// `true`, if the media preview is covered by a spoiler animation.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_spoiler: bool,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// A block with a video, corresponding to `<video>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockvideo).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockVideo {
    /// The video.
    pub video: Video,

    /// `true`, if the media preview is covered by a spoiler animation.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_spoiler: bool,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// A block with a voice note, corresponding to `<audio>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockvoicenote).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockVoiceNote {
    /// The voice note.
    pub voice_note: Voice,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// A block with a "Thinking..." placeholder.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockthinking).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockThinking {
    /// Text of the block.
    pub text: RichText,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paragraph_serde() {
        let block =
            RichBlock::Paragraph(RichBlockParagraph { text: RichText::from("test paragraph") });
        let json = serde_json::to_string(&block).unwrap();
        assert_eq!(json, r#"{"type":"paragraph","text":"test paragraph"}"#);

        let de: RichBlock = serde_json::from_str(&json).unwrap();
        assert_eq!(de, block);
    }

    #[test]
    fn section_heading_serde() {
        let block = RichBlock::SectionHeading(RichBlockSectionHeading {
            text: RichText::from("heading"),
            size: 2,
        });
        let json = serde_json::to_string(&block).unwrap();
        assert_eq!(json, r#"{"type":"heading","text":"heading","size":2}"#);

        let de: RichBlock = serde_json::from_str(&json).unwrap();
        assert_eq!(de, block);
    }
}
