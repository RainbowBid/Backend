use crate::entities::user::User;
use crate::id::Id;

#[derive(Debug, Clone)]
pub struct Item {
    pub id: Id<Item>,
    pub brief: String,
    pub description: String,
    pub picture: Vec<u8>,
    pub user_id: Id<User>,
    pub category: Category,
}

impl Item {
    pub fn new(
        brief: String,
        description: String,
        picture: Vec<u8>,
        user_id: Id<User>,
        category: Category,
    ) -> Self {
        let id = Id::gen();

        Self {
            id,
            brief,
            description,
            picture,
            user_id,
            category,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Category {
    Art,
    Sport,
    Electronics,
    Services,
    Diverse,
}

impl From<Category> for String {
    fn from(category: Category) -> Self {
        match category {
            Category::Art => "art".to_string(),
            Category::Sport => "sport".to_string(),
            Category::Electronics => "electronics".to_string(),
            Category::Services => "services".to_string(),
            Category::Diverse => "diverse".to_string(),
        }
    }
}

impl From<String> for Category {
    fn from(category: String) -> Self {
        match category.as_str() {
            "art" => Category::Art,
            "sport" => Category::Sport,
            "electronics" => Category::Electronics,
            "services" => Category::Services,
            _ => Category::Diverse,
        }
    }
}
