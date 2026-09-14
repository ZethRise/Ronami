use serde::{Deserialize, Serialize};

use crate::types::InputMedia;

/// Describes a media element embedded in an outgoing rich message.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichmessagemedia).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichMessageMedia {
    /// Unique identifier of the media used in a `tg://photo?id=`,
    /// `tg://video?id=`, `tg://document?id=`, or `tg://audio?id=` link. 1-64
    /// characters, only A-Z, a-z, 0-9, _ and - are allowed.
    pub id: String,

    /// The media to be sent. Everything except the media itself and its
    /// properties is ignored.
    pub media: InputMedia,
}

impl InputRichMessageMedia {
    pub fn new<S>(id: S, media: InputMedia) -> Self
    where
        S: Into<String>,
    {
        Self { id: id.into(), media }
    }
}
