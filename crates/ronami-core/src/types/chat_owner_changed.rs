use serde::{Deserialize, Serialize};

use crate::types::User;

/// Describes a service message about an ownership change in the chat.
///
/// [The official docs](https://core.telegram.org/bots/api#chatownerchanged).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ChatOwnerChanged {
    /// The new owner of the chat
    pub new_owner: User,
}
