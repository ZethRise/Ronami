use serde::{Deserialize, Serialize};

use crate::types::User;

/// This object contains information about the bot that was created to be
/// managed by the current bot.
///
/// [The official docs](https://core.telegram.org/bots/api#managedbotcreated).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ManagedBotCreated {
    /// Information about the bot.
    pub bot: User,
}
