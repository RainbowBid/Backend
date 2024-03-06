use crate::interfaces::i_user_repository::IUserRepository;

pub trait IRepositoriesModule {
    fn user_repository(&self) -> &impl IUserRepository;
}
