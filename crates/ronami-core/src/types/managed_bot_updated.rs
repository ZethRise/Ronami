use serde::{Deserialize, Serialize};

use crate::types::User;

/// This object contains information about the creation, token update, or owner
/// update of a bot that is managed by the current bot.
///
/// [The official docs](https://core.telegram.org/bots/api#managedbotupdated).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ManagedBotUpdated {
    /// User that created the bot.
    pub user: User,

    /// Information about the bot. Token of the bot can be fetched using the
    /// method [`get_managed_bot_token`].
    ///
    /// [`get_managed_bot_token`]: crate::requests::Requester::get_managed_bot_token
    pub bot: User,
}
