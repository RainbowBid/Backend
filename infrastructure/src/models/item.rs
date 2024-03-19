use domain::entities::item::Item;
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct ItemModel {
    pub id: Uuid,
    pub brief: String,
    pub description: String,
    pub picture: Vec<u8>,
    pub user_id: Uuid,
    pub category: String,
}

impl TryFrom<ItemModel> for Item {
    type Error = anyhow::Error;

    fn try_from(item_table: ItemModel) -> Result<Self, Self::Error> {
        Ok(Item {
            id: item_table.id.to_string().try_into()?,
            brief: item_table.brief,
            description: item_table.description,
            picture: item_table.picture,
            user_id: item_table.user_id.to_string().try_into()?,
            category: item_table.category.into(),
        })
    }
}

impl TryFrom<Item> for ItemModel {
    type Error = anyhow::Error;

    fn try_from(item: Item) -> Result<Self, Self::Error> {
        Ok(ItemModel {
            id: Uuid::parse_str(&item.id.to_string())?,
            brief: item.brief,
            description: item.description,
            picture: item.picture,
            user_id: Uuid::parse_str(&item.user_id.to_string())?,
            category: item.category.into(),
        })
    }
}
