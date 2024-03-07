use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct Db(pub(crate) Arc<PgPool>);

impl Db {
    pub fn new(pool: PgPool) -> Self {
        Self(Arc::new(pool))
    }
}
