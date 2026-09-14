//! Some useful utilities.

pub mod command;
pub mod html;
pub mod markdown;
pub mod render;
pub(crate) mod shutdown_token;
pub mod split;

pub use ronami_core::net::client_from_env;
pub use split::{split_message, MAX_MESSAGE_LENGTH};
