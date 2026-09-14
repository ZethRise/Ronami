use serde::{Deserialize, Serialize};

use crate::types::{
    Animation, Audio, Chat, Checklist, Contact, Dice, Document, Game, Giveaway, GiveawayWinners,
    Invoice, LinkPreviewOptions, LivePhoto, Location, MessageId, MessageOrigin, PaidMediaInfo,
    PhotoSize, Poll, Sticker, Story, Venue, Video, VideoNote, Voice,
};

/// This object contains information about a message that is being replied to,
/// which may come from another chat or forum topic.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ExternalReplyInfo {
    /// Origin of the message replied to by the given message.
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a
    /// supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if
    /// the original chat is a supergroup or a channel.
    #[serde(with = "crate::types::option_msg_id_as_int")]
    #[cfg_attr(test, schemars(with = "Option<i32>"))]
    pub message_id: Option<MessageId>,
    /// Options used for link preview generation for the original message, if it
    /// is a text message.
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// _true_, if the message media is covered by a spoiler animation.
    #[serde(default)]
    pub has_media_spoiler: bool,

    #[serde(flatten)]
    pub kind: Option<ExternalReplyInfoKind>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)] // LivePhoto variant is modestly larger
pub enum ExternalReplyInfoKind {
    // Note:
    // - `Venue` must be in front of `Location`
    // - `Animation` must be in front of `Document`
    //
    // This is needed so serde doesn't parse `Venue` as `Location` or `Animation` as `Document`
    // (for backward compatability telegram duplicates some fields).
    //
    // See <https://github.com/teloxide/teloxide/issues/481>
    Animation(Animation),
    Audio(Audio),
    Contact(Contact),
    Dice(Dice),
    Document(Document),
    PaidMedia(PaidMediaInfo),
    Game(Game),
    Venue(Venue),
    Location(Location),
    Photo(Vec<PhotoSize>),
    Poll(Poll),
    Checklist(Checklist),
    Sticker(Sticker),
    Story(Story),
    Giveaway(Giveaway),
    GiveawayWinners(GiveawayWinners),
    Video(Video),
    VideoNote(VideoNote),
    Voice(Voice),
    Invoice(Invoice),
    LivePhoto(LivePhoto),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_external_reply_without_media() {
        let json = r#"{
            "origin": {
                "type": "channel",
                "chat": {
                    "id": -1003134985225,
                    "title": "Channel",
                    "type": "channel"
                },
                "message_id": 1537,
                "date": 1720000000
            },
            "chat": {
                "id": -1003106365660,
                "title": "Group",
                "type": "supergroup"
            },
            "message_id": 1537
        }"#;

        let res: Result<ExternalReplyInfo, _> = serde_json::from_str(json);
        assert!(res.is_ok(), "Failed to deserialize: {:?}", res.err());
        assert_eq!(res.unwrap().kind, None);
    }

    #[test]
    fn test_external_reply_with_photo() {
        let json = r#"{
            "origin": {
                "type": "channel",
                "chat": {
                    "id": -1003134985225,
                    "title": "Channel",
                    "type": "channel"
                },
                "message_id": 1537,
                "date": 1720000000
            },
            "chat": {
                "id": -1003106365660,
                "title": "Group",
                "type": "supergroup"
            },
            "message_id": 1537,
            "photo": [
                {
                    "file_id": "photo_123",
                    "file_unique_id": "unique_123",
                    "width": 100,
                    "height": 100
                }
            ]
        }"#;

        let res: Result<ExternalReplyInfo, _> = serde_json::from_str(json);
        assert!(res.is_ok(), "Failed to deserialize: {:?}", res.err());
        assert!(matches!(res.unwrap().kind, Some(ExternalReplyInfoKind::Photo(_))));
    }
}
