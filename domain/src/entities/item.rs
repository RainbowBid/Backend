use crate::entities::user::User;
use crate::id::Id;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Item {
    pub id: Id<Item>,
    pub brief: String,
    pub description: String,
    pub picture: Vec<u8>,
    pub user_id: Id<User>,
}

impl Item {
    pub fn new(brief: String, description: String, picture: Vec<u8>, user_id: Id<User>) -> Self {
        Self {
            id: Id::gen(),
            brief,
            description,
            picture,
            user_id,
        }
    }
}
