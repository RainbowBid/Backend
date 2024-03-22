use anyhow::anyhow;
use domain::app_error::AppError;
use domain::entities::auction::Auction;
use domain::entities::user::User;
use domain::interfaces::i_auction_repository::IAuctionRepository;
use domain::interfaces::i_item_repository::IItemRepository;
use std::sync::Arc;
use tracing::info;

pub mod dtos {
    use anyhow::anyhow;
    use chrono::{DateTime, Utc};
    use domain::app_error::AppError;
    use domain::entities::auction::Auction;
    use domain::id::Id;
    use serde::Deserialize;
    use std::time::SystemTime;
    use validator::{Validate, ValidationError};

    fn validate_end_date(end_date: &i64) -> Result<(), ValidationError> {
        if let Some(end_date) = DateTime::<Utc>::from_timestamp_millis(*end_date) {
            if end_date > Utc::now() + chrono::Duration::minutes(1) {
                Ok(())
            } else {
                Err(ValidationError::new(
                    "End date must be at least 1 minute in the future",
                ))
            }
        } else {
            Err(ValidationError::new("Invalid end date"))
        }
    }

    #[derive(Deserialize, Debug, Validate)]
    pub struct CreateAuctionRequest {
        pub item_id: String,
        #[validate(range(min = 0.0, message = "Starting price must be greater than 0"))]
        pub starting_price: f32,
        #[validate(custom(
            function = "validate_end_date",
            message = "End date must be at least 1 minute in the future"
        ))]
        pub end_date: i64,
    }

    impl TryFrom<CreateAuctionRequest> for Auction {
        type Error = AppError;

        fn try_from(dto: CreateAuctionRequest) -> Result<Auction, AppError> {
            Ok(Auction::new(
                Id::try_from(dto.item_id).map_err(|_| {
                    AppError::CreateAuctionFailed(anyhow!(
                        "Cannot assign invalid item_id to newly created auction"
                    ))
                })?,
                dto.starting_price,
                DateTime::<Utc>::from_timestamp_millis(dto.end_date)
                    .unwrap_or(DateTime::<Utc>::from(SystemTime::now())),
            ))
        }
    }
}

pub struct CreateAuctionUseCase<R1: IAuctionRepository, R2: IItemRepository> {
    auction_repository: Arc<R1>,
    item_repository: Arc<R2>,
}

impl<R1: IAuctionRepository, R2: IItemRepository> CreateAuctionUseCase<R1, R2> {
    pub fn new(auction_repository: Arc<R1>, item_repository: Arc<R2>) -> Self {
        Self {
            auction_repository,
            item_repository,
        }
    }

    pub async fn execute(
        &self,
        current_user: User,
        dto: dtos::CreateAuctionRequest,
    ) -> Result<(), AppError> {
        info!("Creating auction with item_id: {}", dto.item_id);

        let auction: Auction = dto.try_into()?;

        let item = match self.item_repository.find(auction.item_id.clone()).await {
            Ok(Some(item)) => item,
            Ok(None) => {
                return Err(AppError::CannotCreateAuctionForNonExistingItem(
                    auction.item_id.clone().to_string(),
                ))
            }
            Err(_) => {
                return Err(AppError::CreateAuctionFailed(anyhow!(
                    "Failed to create auction"
                )))
            }
        };

        if item.user_id != current_user.id {
            return Err(
                AppError::CannotCreateAuctionForItemThatDoesNotBelongToCurrentUser(
                    current_user.id.to_string(),
                    item.user_id.to_string(),
                ),
            );
        }

        let ongoing_auction = match self
            .auction_repository
            .find_ongoing_by_item_id(auction.item_id.clone())
            .await
        {
            Ok(auction) => auction,
            Err(_) => {
                return Err(AppError::CreateAuctionFailed(anyhow!(
                    "Failed to create auction"
                )))
            }
        };

        if ongoing_auction.is_some() {
            return Err(AppError::CannotCreateAuctionForItemWithOngoingAuction(
                auction.item_id.clone().to_string(),
            ));
        }

        match self.auction_repository.insert(auction).await {
            Ok(Some(_)) => {
                info!("Auction created successfully");
                Ok(())
            }
            _ => Err(AppError::CreateAuctionFailed(anyhow!(
                "Failed to create auction"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::use_cases::auctions::create_auction_use_case::{dtos, CreateAuctionUseCase};
    use anyhow::anyhow;
    use chrono::Utc;
    use domain::entities::auction::Auction;
    use domain::entities::item::{Category, Item};
    use domain::entities::user::User;
    use domain::id::Id;
    use domain::interfaces::i_auction_repository::MockIAuctionRepository;
    use domain::interfaces::i_item_repository::MockIItemRepository;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn given_valid_input_when_executing_then_auction_is_created() {
        // Arrange
        let current_user = User::new(
            "username".to_string(),
            "email".to_string(),
            "hashed_password".to_string(),
        );

        let item_id = Uuid::new_v4();
        let user_id = current_user.id.value;

        let mut item_repository = MockIItemRepository::new();

        let item_id_clone = item_id;
        let user_id_clone = user_id;
        item_repository
            .expect_find()
            .withf(move |id| id.value == item_id_clone)
            .returning(move |_| {
                Ok(Some(Item::new(
                    "brief".to_string(),
                    "description".to_string(),
                    vec![0],
                    Id::try_from(user_id_clone.to_string()).unwrap(),
                    Category::Electronics,
                )))
            });

        let mut auction_repository = MockIAuctionRepository::new();

        let item_id_clone = item_id;
        auction_repository
            .expect_find_ongoing_by_item_id()
            .withf(move |id| id.value == item_id_clone)
            .returning(|_| Ok(None));

        let item_id_clone1 = item_id;
        let item_id_clone2 = item_id;
        auction_repository
            .expect_insert()
            .withf(move |auction| auction.item_id.value == item_id_clone1)
            .returning(move |_| {
                Ok(Some(Auction::new(
                    Id::try_from(item_id_clone2.to_string()).unwrap(),
                    0.0,
                    Utc::now() + chrono::Duration::minutes(10),
                )))
            });

        let use_case =
            CreateAuctionUseCase::new(Arc::new(auction_repository), Arc::new(item_repository));

        let dto = dtos::CreateAuctionRequest {
            item_id: item_id.to_string(),
            starting_price: 0.0,
            end_date: (Utc::now() + chrono::Duration::minutes(10)).timestamp(),
        };

        // Act
        let result = use_case.execute(current_user, dto).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn given_invalid_dto_for_non_existing_item_when_executing_then_cannot_create_auction_for_non_existing_item_is_returned(
    ) {
        // Arrange
        let current_user = User::new(
            "username".to_string(),
            "email".to_string(),
            "hashed_password".to_string(),
        );

        let item_id = Uuid::new_v4();

        let mut item_repository = MockIItemRepository::new();

        item_repository.expect_find().returning(|_| Ok(None));

        let use_case = CreateAuctionUseCase::new(
            Arc::new(MockIAuctionRepository::new()),
            Arc::new(item_repository),
        );

        let dto = dtos::CreateAuctionRequest {
            item_id: item_id.to_string(),
            starting_price: 0.0,
            end_date: (Utc::now() + chrono::Duration::minutes(10)).timestamp(),
        };

        // Act
        let result = use_case.execute(current_user, dto).await;

        // Assert
        match result {
            Err(domain::app_error::AppError::CannotCreateAuctionForNonExistingItem(message)) => {
                assert_eq!(message, item_id.to_string());
            }
            _ => panic!("Test failed"),
        }
    }

    #[tokio::test]
    async fn given_invalid_dto_for_item_that_does_not_belong_to_current_user_when_executing_then_cannot_create_auction_for_item_that_does_not_belong_to_current_user_is_returned(
    ) {
        // Arrange
        let current_user = User::new(
            "username".to_string(),
            "email".to_string(),
            "hashed_password".to_string(),
        );

        let item_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let mut item_repository = MockIItemRepository::new();

        item_repository.expect_find().returning(move |_| {
            Ok(Some(Item::new(
                "brief".to_string(),
                "description".to_string(),
                vec![0],
                Id::try_from(user_id.to_string()).unwrap(),
                Category::Electronics,
            )))
        });

        let use_case = CreateAuctionUseCase::new(
            Arc::new(MockIAuctionRepository::new()),
            Arc::new(item_repository),
        );

        let dto = dtos::CreateAuctionRequest {
            item_id: item_id.to_string(),
            starting_price: 0.0,
            end_date: (Utc::now() + chrono::Duration::minutes(10)).timestamp(),
        };

        // Act
        let result = use_case.execute(current_user.clone(), dto).await;

        // Assert
        match result {
            Err(domain::app_error::AppError::CannotCreateAuctionForItemThatDoesNotBelongToCurrentUser(user_id, item_id)) => {
                assert_eq!(user_id, current_user.id.to_string());
                assert_eq!(item_id, item_id.to_string());
            }
            _ => panic!("Test failed"),
        }
    }

    #[tokio::test]
    async fn given_invalid_dto_for_item_with_ongoing_auction_when_executing_then_cannot_create_auction_for_item_with_ongoing_auction_is_returned(
    ) {
        // Arrange
        let current_user = User::new(
            "username".to_string(),
            "email".to_string(),
            "hashed_password".to_string(),
        );

        let item_id = Uuid::new_v4();
        let user_id = current_user.id.value;

        let mut item_repository = MockIItemRepository::new();

        let item_id_clone = item_id;
        let user_id_clone = user_id;
        item_repository
            .expect_find()
            .withf(move |id| id.value == item_id_clone)
            .returning(move |_| {
                Ok(Some(Item::new(
                    "brief".to_string(),
                    "description".to_string(),
                    vec![0],
                    Id::try_from(user_id_clone.to_string()).unwrap(),
                    Category::Electronics,
                )))
            });

        let mut auction_repository = MockIAuctionRepository::new();

        let item_id_clone = item_id;
        auction_repository
            .expect_find_ongoing_by_item_id()
            .withf(move |id| id.value == item_id_clone)
            .returning(move |_| {
                Ok(Some(Auction::new(
                    Id::try_from(item_id_clone.to_string()).unwrap(),
                    0.0,
                    Utc::now() + chrono::Duration::minutes(10),
                )))
            });

        let use_case =
            CreateAuctionUseCase::new(Arc::new(auction_repository), Arc::new(item_repository));

        let dto = dtos::CreateAuctionRequest {
            item_id: item_id.to_string(),
            starting_price: 0.0,
            end_date: (Utc::now() + chrono::Duration::minutes(10)).timestamp(),
        };

        // Act
        let result = use_case.execute(current_user, dto).await;

        // Assert
        match result {
            Err(domain::app_error::AppError::CannotCreateAuctionForItemWithOngoingAuction(
                item_id,
            )) => {
                assert_eq!(item_id, item_id.to_string());
            }
            _ => panic!("Test failed"),
        }
    }

    #[tokio::test]
    async fn given_invalid_dto_when_executing_then_create_auction_failed_error_is_returned() {
        // Arrange
        let current_user = User::new(
            "username".to_string(),
            "email".to_string(),
            "hashed_password".to_string(),
        );

        let mut item_repository = MockIItemRepository::new();

        item_repository
            .expect_find()
            .returning(|_| Err(anyhow!("")));

        let use_case = CreateAuctionUseCase::new(
            Arc::new(MockIAuctionRepository::new()),
            Arc::new(item_repository),
        );

        let dto = dtos::CreateAuctionRequest {
            item_id: "invalid_id".to_string(),
            starting_price: 0.0,
            end_date: (Utc::now() + chrono::Duration::minutes(10)).timestamp(),
        };

        // Act
        let result = use_case.execute(current_user, dto).await;

        // Assert
        match result {
            Err(domain::app_error::AppError::CreateAuctionFailed(_)) => assert!(true),
            _ => panic!("Test failed"),
        }
    }
}
