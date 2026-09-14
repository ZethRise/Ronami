use serde::{Deserialize, Serialize};

use crate::types::User;

/// This object contains information about changes to a user payment
/// subscription toward the current bot.
///
/// [The official docs](https://core.telegram.org/bots/api#botsubscriptionupdated).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct BotSubscriptionUpdated {
    /// User who subscribed for payments toward the bot.
    pub user: User,

    /// Bot-specified invoice payload.
    pub invoice_payload: String,

    /// The new state of the subscription. Currently, it can be one of
    /// "canceled" if the user canceled the subscription, "active" if the user
    /// re-enabled a previously canceled subscription, or "failed" if payment
    /// for the subscription failed.
    pub state: String,
}
