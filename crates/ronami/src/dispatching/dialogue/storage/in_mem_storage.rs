use super::Storage;
use futures::future::BoxFuture;
use ronami_core::types::ChatId;
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;
use tokio::sync::Mutex;

/// An error returned from [`InMemStorage`].
#[derive(Debug, Error)]
pub enum InMemStorageError {
    /// Returned from [`InMemStorage::remove_dialogue`].
    #[error("row not found")]
    DialogueNotFound,
}

/// A dialogue storage based on [`std::collections::HashMap`].
///
/// ## Note
/// All your dialogues will be lost after you restart your bot. If you need to
/// store them somewhere on a drive, you should use e.g.
/// [`super::SqliteStorage`] or implement your own.
#[derive(Debug)]
pub struct InMemStorage<D> {
    map: Mutex<HashMap<ChatId, D>>,
}

impl<S> InMemStorage<S> {
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self { map: Mutex::new(HashMap::new()) })
    }

    /// Returns a list of all chat IDs that currently have a dialogue state in
    /// memory.
    pub async fn chat_ids(&self) -> Vec<ChatId> {
        self.map.lock().await.keys().copied().collect()
    }
}

impl<D> Storage<D> for InMemStorage<D>
where
    D: Clone,
    D: Send + 'static,
{
    type Error = InMemStorageError;

    fn remove_dialogue(
        self: Arc<Self>,
        chat_id: ChatId,
    ) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + 'static,
    {
        Box::pin(async move {
            self.map
                .lock()
                .await
                .remove(&chat_id)
                .map_or(Err(InMemStorageError::DialogueNotFound), |_| Ok(()))
        })
    }

    fn update_dialogue(
        self: Arc<Self>,
        chat_id: ChatId,
        dialogue: D,
    ) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + 'static,
    {
        Box::pin(async move {
            self.map.lock().await.insert(chat_id, dialogue);
            Ok(())
        })
    }

    fn get_dialogue(
        self: Arc<Self>,
        chat_id: ChatId,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>> {
        Box::pin(async move { Ok(self.map.lock().await.get(&chat_id).map(ToOwned::to_owned)) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_mem_storage_chat_ids() {
        let storage = InMemStorage::<i32>::new();
        assert!(storage.chat_ids().await.is_empty());

        storage.clone().update_dialogue(ChatId(1), 10).await.unwrap();
        storage.clone().update_dialogue(ChatId(2), 20).await.unwrap();

        let mut ids = storage.chat_ids().await;
        ids.sort_by_key(|id| id.0);
        assert_eq!(ids, vec![ChatId(1), ChatId(2)]);
    }
}
