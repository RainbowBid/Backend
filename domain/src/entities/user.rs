use crate::id::Id;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Id<User>,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        Self {
            id: Id::gen(),
            name,
            email,
            password,
        }
    }
}
