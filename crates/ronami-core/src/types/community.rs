use serde::{Deserialize, Serialize};

/// Represents a community (a group of chats).
///
/// [The official docs](https://core.telegram.org/bots/api#community).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Community {
    /// Unique identifier for this community.
    pub id: i64,

    /// Name of the community.
    pub name: String,
}

/// Describes a service message about a chat or a bot being added to a
/// community.
///
/// [The official docs](https://core.telegram.org/bots/api#communitychatadded).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct CommunityChatAdded {
    /// The new community to which the chat or the bot belongs.
    pub community: Community,
}

/// Describes a service message about a chat or a bot being removed from a
/// community.
///
/// [The official docs](https://core.telegram.org/bots/api#communitychatremoved).
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct CommunityChatRemoved {}

/// Describes a service message about a chat being joined by a user from a
/// community.
///
/// [The official docs](https://core.telegram.org/bots/api#communitychatjoined).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct CommunityChatJoined {
    /// The community from which the chat was joined.
    pub community: Community,
}
