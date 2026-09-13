use serde::{Deserialize, Serialize};

/// This object represents a photo with a short video.
///
/// [The official docs](https://core.telegram.org/bots/api#livephoto).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct LivePhoto {
    /// Available sizes of the static photo.
    pub photo: Option<Vec<PhotoSize>>,

    /// Identifier of the live photo video file.
    pub file_id: FileId,

    /// Unique identifier of the live photo video file.
    pub file_unique_id: FileUniqueId,

    /// Live photo video width as defined by the sender.
    pub width: u32,

    /// Live photo video height as defined by the sender.
    pub height: u32,

    /// Duration of the live photo video in seconds as defined by the sender.
    pub duration: Seconds,

    /// MIME type of the live photo video file.
    pub mime_type: Option<String>,

    /// File size of the live photo video in bytes.
    pub file_size: Option<u32>,
}

use crate::types::{FileId, FileUniqueId, PhotoSize, Seconds};

impl LivePhoto {
    /// Creates a new `LivePhoto`.
    pub const fn new(
        file_id: FileId,
        file_unique_id: FileUniqueId,
        width: u32,
        height: u32,
        duration: Seconds,
    ) -> Self {
        Self {
            photo: None,
            file_id,
            file_unique_id,
            width,
            height,
            duration,
            mime_type: None,
            file_size: None,
        }
    }
}
