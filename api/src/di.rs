use std::sync::Arc;

use application::use_cases::auctions::confirm_auction_use_case::ConfirmAuctionUseCase;
use application::use_cases::auctions::create_auction_use_case::CreateAuctionUseCase;
use application::use_cases::auctions::get_ongoing_auction_for_item_use_case::GetAuctionByItemIdUseCase;
use application::use_cases::auctions::get_ongoing_auctions_use_case::GetAuctionsUseCase;
use application::use_cases::auctions::handle_expired_auction_use_case::HandleExpiredAuctionUseCase;
use application::use_cases::auctions::handle_expired_auctions_use_case::HandleExpiredAuctionsUseCase;
use application::use_cases::bids::create_bid_use_case::CreateBidUseCase;
use application::use_cases::bids::get_bids_use_case::GetBidsUseCase;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;

use application::use_cases::items::create_item_use_case::CreateItemUseCase;
use application::use_cases::items::get_item_image_use_case::GetItemImageUseCase;
use application::use_cases::items::get_item_use_case::GetItemUseCase;
use application::use_cases::items::get_items_use_case::GetItemsUseCase;
use application::use_cases::user::get_user_use_case::GetUserUseCase;
use application::use_cases::user::login_use_case::LoginUseCase;
use application::use_cases::user::register_use_case::RegisterUseCase;
use domain::entities::auction::Auction;
use domain::entities::item::Item;
use domain::entities::user::User;
use infrastructure::repositories::DatabaseRepositoryImpl;

pub struct Modules {
    pub(crate) register_use_case: RegisterUseCase<DatabaseRepositoryImpl<User>>,
    pub(crate) login_use_case: LoginUseCase<DatabaseRepositoryImpl<User>>,
    pub(crate) get_user_use_case: GetUserUseCase<DatabaseRepositoryImpl<User>>,
    pub(crate) create_item_use_case: CreateItemUseCase<DatabaseRepositoryImpl<Item>>,
    pub(crate) get_item_image_use_case:
        GetItemImageUseCase<DatabaseRepositoryImpl<Item>, DatabaseRepositoryImpl<Auction>>,
    pub(crate) get_items_use_case: GetItemsUseCase<DatabaseRepositoryImpl<Item>>,
    pub(crate) get_item_use_case:
        GetItemUseCase<DatabaseRepositoryImpl<Item>, DatabaseRepositoryImpl<Auction>>,
    pub(crate) create_auction_use_case:
        CreateAuctionUseCase<DatabaseRepositoryImpl<Auction>, DatabaseRepositoryImpl<Item>>,
    pub(crate) get_by_item_id: GetAuctionByItemIdUseCase<DatabaseRepositoryImpl<Auction>>,
    pub(crate) get_auctions_use_case: GetAuctionsUseCase<DatabaseRepositoryImpl<Auction>>,
    pub(crate) get_bids_use_case: GetBidsUseCase<DatabaseRepositoryImpl<Auction>>,
    pub(crate) create_bid_use_case: CreateBidUseCase<DatabaseRepositoryImpl<Auction>>,
    pub(crate) handle_expired_auctions_use_case:
        HandleExpiredAuctionsUseCase<DatabaseRepositoryImpl<Auction>, DatabaseRepositoryImpl<Item>>,
    pub(crate) confirm_auction_use_case:
        ConfirmAuctionUseCase<DatabaseRepositoryImpl<Auction>, DatabaseRepositoryImpl<Item>>,
}

impl Modules {
    pub fn new(db: PgPool) -> Self {
        let user_repository = Arc::new(DatabaseRepositoryImpl::new(db.clone()));

        let item_repository = Arc::new(DatabaseRepositoryImpl::new(db.clone()));

        let auction_repository = Arc::new(DatabaseRepositoryImpl::new(db.clone()));

        let register_use_case = RegisterUseCase::new(user_repository.clone());

        let login_use_case = LoginUseCase::new(user_repository.clone());

        let get_user_use_case = GetUserUseCase::new(user_repository.clone());

        let get_items_use_case = GetItemsUseCase::new(item_repository.clone());

        let create_item_use_case = CreateItemUseCase::new(item_repository.clone());

        let get_item_image_use_case =
            GetItemImageUseCase::new(item_repository.clone(), auction_repository.clone());

        let get_item_use_case =
            GetItemUseCase::new(item_repository.clone(), auction_repository.clone());

        let create_auction_use_case =
            CreateAuctionUseCase::new(auction_repository.clone(), item_repository.clone());

        let get_by_item_id = GetAuctionByItemIdUseCase::new(auction_repository.clone());

        let get_auctions_use_case = GetAuctionsUseCase::new(auction_repository.clone());

        let get_bids_use_case = GetBidsUseCase::new(auction_repository.clone());

        let create_bid_use_case = CreateBidUseCase::new(auction_repository.clone());

        let handle_expired_auction_use_case = Arc::new(HandleExpiredAuctionUseCase::new(
            auction_repository.clone(),
            item_repository.clone(),
        ));

        let handle_expired_auctions_use_case = HandleExpiredAuctionsUseCase::new(
            auction_repository.clone(),
            handle_expired_auction_use_case.clone(),
        );

        let confirm_auction_use_case =
            ConfirmAuctionUseCase::new(auction_repository.clone(), item_repository.clone());

        Self {
            register_use_case,
            login_use_case,
            get_user_use_case,
            get_items_use_case,
            create_item_use_case,
            get_item_image_use_case,
            get_item_use_case,
            create_auction_use_case,
            get_by_item_id,
            get_auctions_use_case,
            get_bids_use_case,
            create_bid_use_case,
            handle_expired_auctions_use_case,
            confirm_auction_use_case,
        }
    }
}

pub struct Constants {
    pub jwt_key: String,
    pub allowed_origin: String,
    pub jwt_duration: String,
    pub finalize_auctions_cron: String,
}

impl Constants {
    pub fn new(secrets: SecretStore) -> Self {
        let jwt_key = secrets
            .get("JWT_KEY")
            .expect("You need to set your JWT_KEY secret!");

        let jwt_duration = secrets
            .get("JWT_DURATION_MINUTES")
            .expect("You need to set you JWT_DURATION_MINUTES secret!");

        let allowed_origin = secrets
            .get("ALLOWED_ORIGIN")
            .expect("You need to set your ALLOWED_ORIGIN secret!");

        let finalize_auctions_cron = secrets
            .get("FINALIZE_AUCTIONS_CRON")
            .expect("You need to set your FINALIZE_AUCTIONS_CRON secret!");

        Self {
            jwt_key,
            allowed_origin,
            jwt_duration,
            finalize_auctions_cron,
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
