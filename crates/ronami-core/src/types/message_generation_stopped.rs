use serde::{Deserialize, Serialize};

use crate::types::{Chat, DraftId, ThreadId};

/// This object describes an update about a user stopping message generation.
///
/// [The official docs](https://core.telegram.org/bots/api#messagegenerationstopped).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct MessageGenerationStopped {
    /// Chat in which the message is generated.
    pub chat: Chat,

    /// Unique identifier of the message thread in which the message is
    /// generated.
    pub message_thread_id: Option<ThreadId>,

    /// Unique identifier of the message draft which was stopped.
    pub draft_id: DraftId,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"{
            "chat": {"id": 42, "type": "private", "first_name": "Ann"},
            "message_thread_id": 7,
            "draft_id": 5
        }"#;

        let stopped: MessageGenerationStopped = serde_json::from_str(data).unwrap();

        assert_eq!(stopped.chat.id.0, 42);
        assert_eq!(stopped.message_thread_id, Some(ThreadId(crate::types::MessageId(7))));
        assert_eq!(stopped.draft_id, DraftId(5));
    }

    #[test]
    fn deserialize_without_thread() {
        let data = r#"{"chat": {"id": 42, "type": "private", "first_name": "Ann"}, "draft_id": 5}"#;

        let stopped: MessageGenerationStopped = serde_json::from_str(data).unwrap();

        assert_eq!(stopped.message_thread_id, None);
    }
}
