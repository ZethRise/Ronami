use serde::{Deserialize, Serialize};

/// Describes the rating of a user based on their Telegram Star spendings.
///
/// [The official docs](https://core.telegram.org/bots/api#userrating).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UserRating {
    /// Current level of the user, indicating their reliability when purchasing
    /// digital goods and services. A higher level suggests a more trustworthy
    /// customer; a negative level is likely reason for concern.
    pub level: i32,

    /// Numerical value of the user's rating; the higher the rating, the better
    pub rating: i32,

    /// The rating value required to get the current level
    pub current_level_rating: i32,

    /// The rating value required to get to the next level; omitted if the
    /// maximum level was reached
    pub next_level_rating: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{"level":2,"rating":150,"current_level_rating":100,"next_level_rating":200}"#;
        let rating: UserRating = serde_json::from_str(json).unwrap();
        assert_eq!(rating.level, 2);
        assert_eq!(rating.rating, 150);
        assert_eq!(rating.current_level_rating, 100);
        assert_eq!(rating.next_level_rating, Some(200));
    }
}
