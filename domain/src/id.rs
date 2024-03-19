use derive_new::new;
use serde::Serialize;
use uuid::Uuid;

#[derive(new, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Id<T> {
    pub value: Uuid,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Id<T> {
    pub fn gen() -> Self {
        Id::new(Uuid::new_v4())
    }
}

impl<T> std::fmt::Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> TryFrom<String> for Id<T> {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Uuid::parse_str(&value)
            .map(|value| Id::new(value))
            .map_err(|err| anyhow::anyhow!("{:?}", err))
    }
}
