use serde::{Deserialize, Serialize};

use crate::types::{MaybeInaccessibleMessage, MessageEntity};

/// Describes a service message about an option deleted from a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#polloptiondeleted).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollOptionDeleted {
    /// Message containing the poll from which the option was deleted, if known.
    pub poll_message: Option<MaybeInaccessibleMessage>,

    /// Unique identifier of the deleted option.
    pub option_persistent_id: String,

    /// Option text.
    pub option_text: String,

    /// Special entities that appear in the option_text.
    pub option_text_entities: Option<Vec<MessageEntity>>,
}
