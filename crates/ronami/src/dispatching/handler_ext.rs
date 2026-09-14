use crate::{
    dispatching::{
        dialogue::{GetChatId, Storage},
        DpHandlerDescription,
    },
    types::{Me, Message},
    utils::command::BotCommands,
};
use dptree::Handler;

use std::fmt::Debug;

/// Extension methods for working with `dptree` handlers.
pub trait HandlerExt<Output> {
    /// Returns a handler that accepts a parsed command `C`.
    ///
    /// ## Dependency requirements
    ///
    ///  - [`crate::types::Message`]
    ///  - [`crate::types::Me`]
    #[must_use]
    fn filter_command<C>(self) -> Self
    where
        C: BotCommands + Send + Sync + 'static;

    /// Returns a handler that accepts a parsed command `C` if the command
    /// contains a bot mention, for example `/start@my_bot`.
    ///
    /// ## Dependency requirements
    ///
    ///  - [`crate::types::Message`]
    ///  - [`crate::types::Me`]
    #[must_use]
    fn filter_mention_command<C>(self) -> Self
    where
        C: BotCommands + Send + Sync + 'static;

    /// Returns a handler that extracts a [`RepliedMessage`] from an incoming
    /// [`Message`], allowing access to both the original message and the
    /// replied-to message.
    ///
    /// ## Dependency requirements
    ///
    ///  - [`crate::types::Message`]
    ///
    /// [`RepliedMessage`]: crate::types::RepliedMessage
    #[must_use]
    fn filter_replied_message(self) -> Self;

    /// Passes [`Dialogue<D, S>`] and `D` as handler dependencies.
    ///
    /// It does so by the following steps:
    ///
    ///  1. If an incoming update has no chat ID ([`GetChatId::chat_id`] returns
    ///     `None`), the rest of the chain will not be executed. Otherwise,
    ///     passes `Dialogue::new(storage, chat_id)` forwards.
    ///  2. If [`Dialogue::get_or_default`] on the passed dialogue returns `Ok`,
    ///     passes the dialogue state forwards. Otherwise, logs an error and the
    ///     rest of the chain is not executed.
    ///
    /// If `RONAMI_DIALOGUE_BEHAVIOUR` environmental variable exists and is
    /// equal to "default", this function will not panic if it can't get the
    /// dialogue (if, for example, the state enum was updated). Setting the
    /// value to "panic" will return the initial behaviour.
    ///
    /// ## Dependency requirements
    ///
    ///  - `Arc<S>`
    ///  - `Upd`
    ///
    /// [`Dialogue<D, S>`]: super::dialogue::Dialogue
    /// [`Dialogue::get_or_default`]: super::dialogue::Dialogue::get_or_default
    #[must_use]
    fn enter_dialogue<Upd, S, D>(self) -> Self
    where
        S: Storage<D> + ?Sized + Send + Sync + 'static,
        <S as Storage<D>>::Error: Debug + Send,
        D: Default + Clone + Send + Sync + 'static,
        Upd: GetChatId + Clone + Send + Sync + 'static;
}

impl<Output> HandlerExt<Output> for Handler<'static, Output, DpHandlerDescription>
where
    Output: Send + Sync + 'static,
{
    fn filter_command<C>(self) -> Self
    where
        C: BotCommands + Send + Sync + 'static,
    {
        self.chain(filter_command::<C, Output>())
    }

    fn filter_mention_command<C>(self) -> Self
    where
        C: BotCommands + Send + Sync + 'static,
    {
        self.chain(filter_mention_command::<C, Output>())
    }

    fn filter_replied_message(self) -> Self {
        self.chain(filter_replied_message::<Output>())
    }

    fn enter_dialogue<Upd, S, D>(self) -> Self
    where
        S: Storage<D> + ?Sized + Send + Sync + 'static,
        <S as Storage<D>>::Error: Debug + Send,
        D: Default + Clone + Send + Sync + 'static,
        Upd: GetChatId + Clone + Send + Sync + 'static,
    {
        self.chain(super::dialogue::enter::<Upd, S, D, Output>())
    }
}

/// Returns a handler that accepts a parsed command `C`.
///
/// A call to this function is the same as `dptree::entry().filter_command()`.
///
/// See [`HandlerExt::filter_command`].
///
/// ## Dependency requirements
///
///  - [`crate::types::Message`]
///  - [`crate::types::Me`]
#[must_use]
pub fn filter_command<C, Output>() -> Handler<'static, Output, DpHandlerDescription>
where
    C: BotCommands + Send + Sync + 'static,
    Output: Send + Sync + 'static,
{
    dptree::filter_map(move |message: Message, me: Me| {
        let bot_name = me.user.username.expect("Bots must have a username");
        message.text().or_else(|| message.caption()).and_then(|text| C::parse(text, &bot_name).ok())
    })
}

/// Returns a handler that accepts a parsed command `C` if the command
/// contains a bot mention, for example `/start@my_bot`.
///
/// A call to this function is the same as
/// `dptree::entry().filter_mention_command()`.
///
/// See [`HandlerExt::filter_mention_command`].
///
/// ## Dependency requirements
///
///  - [`crate::types::Message`]
///  - [`crate::types::Me`]
#[must_use]
pub fn filter_mention_command<C, Output>() -> Handler<'static, Output, DpHandlerDescription>
where
    C: BotCommands + Send + Sync + 'static,
    Output: Send + Sync + 'static,
{
    dptree::filter_map(move |message: Message, me: Me| {
        let bot_name = me.user.username.expect("Bots must have a username");

        let text_or_caption = message.text().or_else(|| message.caption());
        let command = text_or_caption.and_then(|text| C::parse(text, &bot_name).ok());
        // If the parsing succeeds with a bot_name,
        // but fails without - there is a mention
        let is_username_required =
            text_or_caption.and_then(|text| C::parse(text, "").ok()).is_none();

        if !is_username_required {
            return None;
        }
        command
    })
}

/// A call to this function is the same as
/// `dptree::entry().filter_replied_message()`.
///
/// See [`HandlerExt::filter_replied_message`].
#[must_use]
pub fn filter_replied_message<Output>() -> Handler<'static, Output, DpHandlerDescription>
where
    Output: Send + Sync + 'static,
{
    dptree::filter_map(|message: Message| message.replied_message())
}

#[cfg(test)]
#[cfg(feature = "macros")]
mod tests {
    use crate::{self as ronami, dispatching::UpdateFilterExt, utils::command::BotCommands};
    use chrono::DateTime;
    use dptree::deps;
    use ronami_core::types::{
        Chat, ChatId, ChatKind, ChatPrivate, LinkPreviewOptions, Me, MediaKind, MediaText, Message,
        MessageCommon, MessageId, MessageKind, Update, UpdateId, UpdateKind, User, UserId,
    };

    use super::HandlerExt;

    #[derive(BotCommands, Clone)]
    #[command(rename_rule = "lowercase")]
    enum Cmd {
        Test,
    }

    fn make_update(text: String) -> Update {
        let timestamp = 1_569_518_829;
        let date = DateTime::from_timestamp(timestamp, 0).unwrap();
        Update {
            id: UpdateId(326_170_274),
            kind: UpdateKind::Message(Message {
                via_bot: None,
                id: MessageId(5042),
                thread_id: None,
                from: Some(User {
                    id: UserId(109_998_024),
                    is_bot: false,
                    first_name: String::from("Laster"),
                    last_name: None,
                    username: Some(String::from("laster_alex")),
                    language_code: Some(String::from("en")),
                    is_premium: false,
                    added_to_attachment_menu: false,
                    can_manage_bots: false,
                    supports_guest_queries: false,
                    supports_join_request_queries: false,
                }),
                sender_chat: None,
                is_topic_message: false,
                is_paid_post: false,
                suggested_post_info: None,
                sender_business_bot: None,
                date,
                chat: Chat {
                    id: ChatId(109_998_024),
                    kind: ChatKind::Private(ChatPrivate {
                        username: Some(String::from("Laster")),
                        first_name: Some(String::from("laster_alex")),
                        last_name: None,
                    }),
                },
                direct_messages_topic: None,
                kind: MessageKind::Common(MessageCommon {
                    reply_to_message: None,
                    forward_origin: None,
                    external_reply: None,
                    quote: None,
                    edit_date: None,
                    media_kind: MediaKind::Text(MediaText {
                        text,
                        entities: vec![],
                        link_preview_options: Some(LinkPreviewOptions {
                            is_disabled: true,
                            url: None,
                            prefer_small_media: false,
                            prefer_large_media: false,
                            show_above_text: false,
                        }),
                    }),
                    reply_markup: None,
                    author_signature: None,
                    paid_star_count: None,
                    effect_id: None,
                    is_automatic_forward: false,
                    has_protected_content: false,
                    reply_to_checklist_task_id: None,
                    reply_to_story: None,
                    reply_to_poll_option_id: None,
                    sender_tag: None,
                    guest_query_id: None,
                    guest_bot_caller_user: None,
                    guest_bot_caller_chat: None,
                    live_photo: None,
                    rich_message: None,
                    sender_boost_count: None,
                    is_from_offline: false,
                    business_connection_id: None,
                }),
            }),
        }
    }

    fn make_me() -> Me {
        Me {
            user: User {
                id: UserId(42),
                is_bot: true,
                first_name: "First".to_owned(),
                last_name: None,
                username: Some("SomethingSomethingBot".to_owned()),
                language_code: None,
                is_premium: false,
                added_to_attachment_menu: false,
                can_manage_bots: false,
                supports_guest_queries: false,
                supports_join_request_queries: false,
            },
            can_join_groups: false,
            allows_users_to_create_topics: false,
            can_read_all_group_messages: false,
            supports_inline_queries: false,
            can_connect_to_business: false,
            has_main_web_app: false,
            has_topics_enabled: false,
        }
    }

    #[tokio::test]
    async fn test_filter_command() {
        let h = dptree::entry()
            .branch(Update::filter_message().filter_command::<Cmd>().endpoint(|| async {}));
        let me = make_me();

        let update = make_update("/test@".to_owned() + me.username());
        let result = h.dispatch(deps![update, me.clone()]).await;
        assert!(result.is_break());

        let update = make_update("/test@".to_owned() + "SomeOtherBot");
        let result = h.dispatch(deps![update, me.clone()]).await;
        assert!(result.is_continue());

        let update = make_update("/test".to_owned());
        let result = h.dispatch(deps![update, me.clone()]).await;
        assert!(result.is_break());
    }

    #[tokio::test]
    async fn test_filter_mention_command() {
        let h = dptree::entry()
            .branch(Update::filter_message().filter_mention_command::<Cmd>().endpoint(|| async {}));
        let me = make_me();

        let update = make_update("/test@".to_owned() + me.username());
        let result = h.dispatch(deps![update, me.clone()]).await;
        assert!(result.is_break());

        let update = make_update("/test@".to_owned() + "SomeOtherBot");
        let result = h.dispatch(deps![update, me.clone()]).await;
        assert!(result.is_continue());

        let update = make_update("/test".to_owned());
        let result = h.dispatch(deps![update, me.clone()]).await;
        assert!(result.is_continue());
    }

    #[tokio::test]
    async fn test_filter_replied_message() {
        use ronami_core::types::{MessageKind, RepliedMessage, UpdateKind};

        let h = dptree::entry().branch(Update::filter_message().filter_replied_message().endpoint(
            |msg: Message, reply: RepliedMessage| async move {
                assert_eq!(msg.text(), Some("reply"));
                assert_eq!(reply.text(), Some("original"));
            },
        ));

        let original = serde_json::from_str::<Message>(
            r#"{
            "message_id": 1,
            "date": 100,
            "chat": {"id": 1, "type": "private", "first_name": "fn"},
            "text": "original"
        }"#,
        )
        .unwrap();

        let mut reply_msg = serde_json::from_str::<Message>(
            r#"{
            "message_id": 2,
            "date": 200,
            "chat": {"id": 1, "type": "private", "first_name": "fn"},
            "text": "reply"
        }"#,
        )
        .unwrap();

        if let MessageKind::Common(ref mut common) = reply_msg.kind {
            common.reply_to_message = Some(Box::new(original));
        }

        let update =
            Update { id: ronami_core::types::UpdateId(1), kind: UpdateKind::Message(reply_msg) };

        let result = h.dispatch(deps![update]).await;
        assert!(result.is_break());
    }
}
