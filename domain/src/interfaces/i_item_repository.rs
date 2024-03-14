use crate::entities::item::Item;
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait IItemRepository {
    async fn insert(&self, item: Item) -> anyhow::Result<Option<Item>>;
}
