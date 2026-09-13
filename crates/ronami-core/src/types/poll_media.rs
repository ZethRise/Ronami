use serde::{Deserialize, Serialize};

use crate::types::{
    Animation, Audio, Document, InputFile, InputMediaAnimation, InputMediaAudio,
    InputMediaDocument, InputMediaPhoto, InputMediaVideo, LivePhoto, Location, MessageEntity,
    ParseMode, PhotoSize, Sticker, Venue, Video,
};
use url::Url;

/// This object represents a link in a poll option.
///
/// [The official docs](https://core.telegram.org/bots/api#link).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Link {
    /// URL of the link.
    pub url: Url,
}

impl Link {
    /// Creates a new `Link`.
    pub const fn new(url: Url) -> Self {
        Self { url }
    }
}

/// This object represents a media in a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#pollmedia).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollMedia {
    /// The media is an animation.
    pub animation: Option<Animation>,

    /// The media is an audio file.
    pub audio: Option<Audio>,

    /// The media is a document.
    pub document: Option<Document>,

    /// The media is a link.
    pub link: Option<Link>,

    /// The media is a live photo.
    pub live_photo: Option<LivePhoto>,

    /// The media is a location.
    pub location: Option<Location>,

    /// The media is a photo.
    pub photo: Option<Vec<PhotoSize>>,

    /// The media is a sticker.
    pub sticker: Option<Sticker>,

    /// The media is a venue.
    pub venue: Option<Venue>,

    /// The media is a video.
    pub video: Option<Video>,
}

/// Represents a live photo to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmedialivephoto).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaLivePhoto {
    /// Live photo video to send.
    pub media: InputFile,

    /// The static photo to send.
    pub photo: InputFile,

    /// Caption of the live photo to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Mode for parsing entities in the caption.
    pub parse_mode: Option<ParseMode>,

    /// List of special entities that appear in the caption, which can be
    /// specified instead of `parse_mode`.
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Pass `true`, if the caption must be shown above the message media.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub show_caption_above_media: bool,

    /// Pass `true` if the live photo needs to be covered with a spoiler
    /// animation.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_spoiler: bool,
}

/// Represents a location to be sent as poll media.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmedialocation).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaLocation {
    /// Latitude of the location in degrees.
    pub latitude: f64,

    /// Longitude of the location in degrees.
    pub longitude: f64,

    /// The radius of uncertainty for the location, measured in meters; 0-1500.
    pub horizontal_accuracy: Option<f64>,
}

/// Represents a venue to be sent as poll media.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediavenue).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaVenue {
    /// Latitude of the venue in degrees.
    pub latitude: f64,

    /// Longitude of the venue in degrees.
    pub longitude: f64,

    /// Name of the venue.
    pub title: String,

    /// Address of the venue.
    pub address: String,

    /// Foursquare identifier of the venue, if known.
    pub foursquare_id: Option<String>,

    /// Foursquare type of the venue, if known.
    pub foursquare_type: Option<String>,

    /// Google Places identifier of the venue, if known.
    pub google_place_id: Option<String>,

    /// Google Places type of the venue, if known.
    pub google_place_type: Option<String>,
}

/// Represents a sticker to be sent as poll option media.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediasticker).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaSticker {
    /// The sticker to send.
    pub media: InputFile,

    /// Emoji associated with the sticker.
    pub emoji: Option<String>,
}

/// Represents a link to be sent as poll option media.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmedialink).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaLink {
    /// URL of the link.
    pub url: Url,
}

/// This object represents the content of a poll description or a quiz
/// explanation to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputpollmedia).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InputPollMedia {
    Animation(InputMediaAnimation),
    Audio(InputMediaAudio),
    Document(InputMediaDocument),
    LivePhoto(InputMediaLivePhoto),
    Location(InputMediaLocation),
    Photo(InputMediaPhoto),
    Venue(InputMediaVenue),
    Video(InputMediaVideo),
}

/// This object represents the content of a poll option to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputpolloptionmedia).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InputPollOptionMedia {
    Animation(InputMediaAnimation),
    Link(InputMediaLink),
    LivePhoto(InputMediaLivePhoto),
    Location(InputMediaLocation),
    Photo(InputMediaPhoto),
    Sticker(InputMediaSticker),
    Venue(InputMediaVenue),
    Video(InputMediaVideo),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_serde() {
        let json = r#"{"url":"https://telegram.org"}"#;
        let link: Link = serde_json::from_str(json).unwrap();
        assert_eq!(link.url.as_str(), "https://telegram.org/");
    }

    #[test]
    fn poll_media_deserialize() {
        let json = r#"{"photo":[{"file_id":"fid","file_unique_id":"uid","width":1,"height":1}]}"#;
        let media: PollMedia = serde_json::from_str(json).unwrap();
        assert!(media.photo.is_some());
    }
}
