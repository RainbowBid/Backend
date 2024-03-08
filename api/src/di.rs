use application::use_cases::login_use_case::LoginUseCase;
use application::use_cases::register_use_case::RegisterUseCase;
use domain::entities::user::User;
use infrastructure::repositories::DatabaseRepositoryImpl;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use std::sync::Arc;

pub struct Modules {
    pub(crate) register_use_case: RegisterUseCase<DatabaseRepositoryImpl<User>>,
    pub(crate) login_use_case: LoginUseCase<DatabaseRepositoryImpl<User>>,
}

impl Modules {
    pub fn new(db: PgPool) -> Self {
        let user_repository = Arc::new(DatabaseRepositoryImpl::new(db));

        let register_use_case = RegisterUseCase::new(user_repository.clone());

        let login_use_case = LoginUseCase::new(user_repository.clone());

        Self {
            register_use_case,
            login_use_case,
        }
    }
}

pub struct Constants {
    pub jwt_key: String,
    pub allowed_origin: String,
}
impl Constants {
    pub fn new(secrets: SecretStore) -> Self {
        let jwt_key = secrets
            .get("JWT_SECRET")
            .expect("You need to set your JWT_KEY secret!");

        let allowed_origin = secrets
            .get("ALLOWED_ORIGIN")
            .expect("You need to set your ALLOWED_ORIGIN secret!");

        Self {
            jwt_key,
            allowed_origin,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub modules: Arc<Modules>,
    pub config: Arc<Constants>,
}

impl AppState {
    pub fn new(db: PgPool, secrets: SecretStore) -> Self {
        let modules = Arc::new(Modules::new(db));
        let config = Arc::new(Constants::new(secrets));

        Self { modules, config }
    }
}
