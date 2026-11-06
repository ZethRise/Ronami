use serde::{Deserialize, Serialize};

use crate::types::{CallbackQueryId, UserId};

/// Describes the parameters of an ephemeral message to be sent.
///
/// An ephemeral message is visible only to a specific user and the bot.
///
/// [The official docs](https://core.telegram.org/bots/api#ephemeralmessageparameters).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct EphemeralMessageParameters {
    /// Identifier of the user who will receive the message.
    ///
    /// It is not guaranteed that the user will receive the message, especially
    /// if they are offline.
    pub receiver_user_id: UserId,

    /// Identifier of the callback query which triggered the message, if any.
    pub callback_query_id: Option<CallbackQueryId>,

    /// `true`, if the ephemeral message must be shown in place of the original
    /// message.
    ///
    /// Must be `false` for callback queries from ephemeral messages, which must
    /// be edited using regular `editEphemeralMessage…` methods.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub replace_callback_query_message: bool,
}

impl EphemeralMessageParameters {
    /// Creates parameters for an ephemeral message visible to the specified
    /// user.
    #[must_use]
    pub const fn new(receiver_user_id: UserId) -> Self {
        Self { receiver_user_id, callback_query_id: None, replace_callback_query_message: false }
    }

    /// Sets the callback query which triggered the message.
    #[must_use]
    pub fn callback_query_id<C>(mut self, val: C) -> Self
    where
        C: Into<CallbackQueryId>,
    {
        self.callback_query_id = Some(val.into());
        self
    }

    /// Makes the ephemeral message to be shown in place of the original
    /// message.
    #[must_use]
    pub fn replace_callback_query_message(self) -> Self {
        Self { replace_callback_query_message: true, ..self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_minimal() {
        let params = EphemeralMessageParameters::new(UserId(42));

        assert_eq!(serde_json::to_string(&params).unwrap(), r#"{"receiver_user_id":42}"#);
    }

    #[test]
    fn serialize_full() {
        let params = EphemeralMessageParameters::new(UserId(42))
            .callback_query_id(CallbackQueryId("query".to_owned()))
            .replace_callback_query_message();

        assert_eq!(
            serde_json::to_string(&params).unwrap(),
            r#"{"receiver_user_id":42,"callback_query_id":"query","replace_callback_query_message":true}"#
        );
    }

    #[test]
    fn deserialize() {
        let params: EphemeralMessageParameters =
            serde_json::from_str(r#"{"receiver_user_id":42,"callback_query_id":"query"}"#).unwrap();

        assert_eq!(
            params,
            EphemeralMessageParameters::new(UserId(42))
                .callback_query_id(CallbackQueryId("query".to_owned()))
        );
    }

    #[test]
    fn send_message_payload_uses_ephemeral_message_parameters() {
        use crate::{
            payloads::{SendMessage, SendMessageSetters as _},
            types::ChatId,
        };

        let payload = SendMessage::new(ChatId(1), "hi")
            .ephemeral_message_parameters(EphemeralMessageParameters::new(UserId(42)));

        assert_eq!(
            serde_json::to_value(&payload).unwrap(),
            serde_json::json!({
                "chat_id": 1,
                "text": "hi",
                "ephemeral_message_parameters": {"receiver_user_id": 42}
            })
        );
    }

    #[test]
    fn draft_payloads_serialize_stop_options() {
        use crate::{
            payloads::{SendMessageDraft, SendMessageDraftSetters as _},
            types::{ChatId, DraftId},
        };

        let payload = SendMessageDraft::new(ChatId(1), DraftId(3), "partial")
            .can_stop(true)
            .keep_on_stop(true);

        assert_eq!(
            serde_json::to_value(&payload).unwrap(),
            serde_json::json!({
                "chat_id": 1,
                "draft_id": 3,
                "text": "partial",
                "can_stop": true,
                "keep_on_stop": true
            })
        );
    }
}
