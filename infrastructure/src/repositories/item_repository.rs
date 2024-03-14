use async_trait::async_trait;
use domain::entities::item::Item;
use domain::entities::user::User;
use domain::id::Id;
use crate::models::item::ItemModel;
use crate::repositories::DatabaseRepositoryImpl;
use domain::interfaces::i_item_repository::IItemRepository;

#[async_trait]
impl IItemRepository for DatabaseRepositoryImpl<ItemModel>{
    async fn get_all_by_user_id(&self, user_id: String) -> anyhow::Result<Option<Vec<Item>>> {
        todo!()
    }
}