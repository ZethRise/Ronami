use serde::{Deserialize, Serialize};

use crate::types::Audio;

/// This object represents the audios displayed on a user's profile.
///
/// [The official docs](https://core.telegram.org/bots/api#userprofileaudios).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UserProfileAudios {
    /// Total number of profile audios for the target user
    pub total_count: u32,

    /// Requested profile audios
    pub audios: Vec<Audio>,
}
