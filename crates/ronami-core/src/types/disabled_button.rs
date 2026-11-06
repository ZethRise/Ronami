use serde::{Deserialize, Serialize};

/// This object represents a disabled button which does nothing.
///
/// Currently holds no information.
///
/// [The official docs](https://core.telegram.org/bots/api#disabledbutton).
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct DisabledButton {}
