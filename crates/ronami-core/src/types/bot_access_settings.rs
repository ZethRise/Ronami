use serde::{Deserialize, Serialize};

use crate::types::User;

/// Describes the access settings of a managed bot.
///
/// [The official docs](https://core.telegram.org/bots/api#botaccesssettings).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct BotAccessSettings {
    /// `true`, if only selected users can access the bot.
    pub is_access_restricted: bool,

    /// Users that have access to the bot in addition to its owner.
    pub added_users: Option<Vec<User>>,
}

impl BotAccessSettings {
    /// Creates a new `BotAccessSettings`.
    pub const fn new(is_access_restricted: bool) -> Self {
        Self { is_access_restricted, added_users: None }
    }
}
