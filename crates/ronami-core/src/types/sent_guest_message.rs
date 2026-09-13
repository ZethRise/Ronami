use derive_more::derive::From;
use serde::{Deserialize, Serialize};

/// This object represents a message sent by the bot as a reply to a guest
/// message.
///
/// [The official docs](https://core.telegram.org/bots/api#sentguestmessage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct SentGuestMessage {
    /// Identifier of the sent ephemeral message.
    pub inline_message_id: String,
}

impl SentGuestMessage {
    /// Creates a new `SentGuestMessage`.
    pub const fn new(inline_message_id: String) -> Self {
        Self { inline_message_id }
    }
}

/// Unique query identifier for a guest message.
#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, From)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(transparent)]
#[from(&'static str, String)]
pub struct GuestQueryId(pub String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sent_guest_message_de() {
        let json = r#"{"inline_message_id":"msgid1"}"#;
        let m: SentGuestMessage = serde_json::from_str(json).unwrap();
        assert_eq!(m.inline_message_id, "msgid1");
    }
}
