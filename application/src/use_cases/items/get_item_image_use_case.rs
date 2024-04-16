use anyhow::anyhow;
use domain::app_error::AppError;
use domain::entities::item::Item;
use domain::entities::user::User;
use domain::id::Id;
use domain::interfaces::i_auction_repository::IAuctionRepository;
use domain::interfaces::i_item_repository::IItemRepository;
use std::sync::Arc;
use tracing::{error, info};

pub struct GetItemImageUseCase<R1: IItemRepository, R2: IAuctionRepository> {
    item_repository: Arc<R1>,
    auction_repository: Arc<R2>,
}

impl<R1: IItemRepository, R2: IAuctionRepository> GetItemImageUseCase<R1, R2> {
    pub fn new(item_repository: Arc<R1>, auction_repository: Arc<R2>) -> Self {
        Self {
            item_repository,
            auction_repository,
        }
    }

    pub async fn execute(&self, current_user: User, item_id: String) -> Result<Vec<u8>, AppError> {
        info!("Getting image for items with id: {}", item_id);

        let id = Id::<Item>::try_from(item_id.clone()).map_err(|_| {
            AppError::GetItemImageFailed(anyhow!("Cannot assign invalid item_id to get image"))
        })?;

        match self.item_repository.find(id.clone()).await {
            Ok(Some(item)) => {
                info!("Image found");

                let auction = self
                    .auction_repository
                    .find_ongoing_by_item_id(id)
                    .await
                    .map_err(|_| {
                        AppError::GetItemImageFailed(anyhow!("Failed to get auction if any"))
                    })?;

                match item.user_id == current_user.id || auction.is_some() {
                    true => {
                        info!("Image belongs to user or has an ongoing auction");
                        Ok(item.picture)
                    }
                    _ => {
                        error!(
                            "Image does not belong to user and does not have any ongoing auction"
                        );
                        Err(AppError::ItemDoesNotBelongToUser(
                            item_id,
                            current_user.id.to_string(),
                        ))
                    }
                }
            }
            _ => {
                error!("Failed to get image");
                Err(AppError::GetItemImageFailed(anyhow!("Failed to get image")))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::use_cases::items::get_item_image_use_case::GetItemImageUseCase;
    use domain::app_error::AppError;
    use domain::entities::item::{Category, Item};
    use domain::interfaces::i_auction_repository::MockIAuctionRepository;
    use domain::interfaces::i_item_repository::MockIItemRepository;
    use std::sync::Arc;

    #[tokio::test]
    async fn given_valid_item_id_and_user_id_when_get_item_image_use_case_then_return_image() {
        // Arrange
        let item_id = String::from("00000000-0000-0000-0000-000000000000");
        let user_id = String::from("00000000-0000-0000-0000-000000000001");
        let item = Item::new(
            "brief".to_string(),
            "description".to_string(),
            vec![],
            user_id.clone().try_into().unwrap(),
            Category::Diverse,
        );
        let mut item_repository = MockIItemRepository::new();

        let item_id_clone = item_id.clone();
        item_repository
            .expect_find()
            .withf(move |id| id.value.to_string() == item_id_clone)
            .returning(move |_| Ok(Some(item.clone())));

        let item_repository = Arc::new(item_repository);

        let current_user = domain::entities::user::User {
            id: user_id.try_into().unwrap(),
            name: "username".to_string(),
            email: "email".to_string(),
            password: "password".to_string(),
        };

        let mut auction_repository = MockIAuctionRepository::new();
        auction_repository
            .expect_find_ongoing_by_item_id()
            .returning(|_| Ok(None));
        let auction_repository = Arc::new(auction_repository);

        let get_item_image_use_case = GetItemImageUseCase::new(item_repository, auction_repository);

        // Act
        let result = get_item_image_use_case
            .execute(current_user, item_id.to_string())
            .await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn given_valid_item_id_with_ongoing_auction_and_user_id_other_than_item_owner_when_get_item_image_use_case_then_return_image(
    ) {
        // Arrange
        let item_id = String::from("00000000-0000-0000-0000-000000000000");
        let user_id = String::from("00000000-0000-0000-0000-000000000001");
        let not_owner_user_id = String::from("00000000-0000-0000-0000-000000000002");
        let item = Item::new(
            "brief".to_string(),
            "description".to_string(),
            vec![],
            user_id.clone().try_into().unwrap(),
            Category::Diverse,
        );
        let mut item_repository = MockIItemRepository::new();

        let item_id_clone = item_id.clone();
        item_repository
            .expect_find()
            .withf(move |id| id.value.to_string() == item_id_clone)
            .returning(move |_| Ok(Some(item.clone())));

        let item_repository = Arc::new(item_repository);

        let current_user = domain::entities::user::User {
            id: not_owner_user_id.try_into().unwrap(),
            name: "username".to_string(),
            email: "email".to_string(),
            password: "password".to_string(),
        };

        let mut auction_repository = MockIAuctionRepository::new();

        let auction = domain::entities::auction::Auction::new(
            item_id.clone().try_into().unwrap(),
            100.0,
            chrono::Utc::now() + chrono::Duration::days(1),
            domain::entities::auction::AuctionStrategy::Standard,
        );

        let item_id_clone = item_id.clone();
        auction_repository
            .expect_find_ongoing_by_item_id()
            .withf(move |id| id.value.to_string() == item_id_clone)
            .returning(move |_| Ok(Some(auction.clone())));
        let auction_repository = Arc::new(auction_repository);

        let get_item_image_use_case = GetItemImageUseCase::new(item_repository, auction_repository);

        // Act
        let result = get_item_image_use_case
            .execute(current_user, item_id.to_string())
            .await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn given_invalid_item_id_when_get_item_image_use_case_then_return_error() {
        // Arrange
        let item_id = String::from("00000000-0000-0000-0000-000000000000");
        let mut item_repository = MockIItemRepository::new();

        let item_id_clone = item_id.clone();
        item_repository
            .expect_find()
            .withf(move |id| id.value.to_string() == item_id_clone)
            .returning(|_| Ok(None));
        let item_repository = Arc::new(item_repository);
        let current_user = domain::entities::user::User::new(
            "username".to_string(),
            "email".to_string(),
            "password".to_string(),
        );

        let mut auction_repository = MockIAuctionRepository::new();
        auction_repository
            .expect_find_ongoing_by_item_id()
            .returning(|_| Ok(None));
        let auction_repository = Arc::new(auction_repository);

        let get_item_image_use_case = GetItemImageUseCase::new(item_repository, auction_repository);

        // Act
        let result = get_item_image_use_case
            .execute(current_user, item_id.to_string())
            .await;

        // Assert
        match result {
            Err(AppError::GetItemImageFailed(_)) => assert!(true),
            _ => panic!("Test failed"),
        }
    }

    #[tokio::test]
    async fn given_item_id_not_belonging_to_user_when_get_item_image_use_case_then_return_error() {
        // Arrange
        let item_id = String::from("00000000-0000-0000-0000-000000000000");
        let item = Item::new(
            "brief".to_string(),
            "description".to_string(),
            vec![],
            String::from("00000000-0000-0000-0000-000000000001")
                .try_into()
                .unwrap(),
            Category::Diverse,
        );
        let mut item_repository = MockIItemRepository::new();

        let item_id_clone = item_id.clone();
        item_repository
            .expect_find()
            .withf(move |id| id.value.to_string() == item_id_clone)
            .returning(move |_| Ok(Some(item.clone())));
        let item_repository = Arc::new(item_repository);
        let current_user = domain::entities::user::User::new(
            "username".to_string(),
            "email".to_string(),
            "password".to_string(),
        );

        let mut auction_repository = MockIAuctionRepository::new();
        auction_repository
            .expect_find_ongoing_by_item_id()
            .returning(|_| Ok(None));
        let auction_repository = Arc::new(auction_repository);

        let get_item_image_use_case = GetItemImageUseCase::new(item_repository, auction_repository);

        // Act
        let result = get_item_image_use_case
            .execute(current_user, item_id.to_string())
            .await;

        // Assert
        match result {
            Err(AppError::ItemDoesNotBelongToUser(_, _)) => assert!(true),
            _ => panic!("Test failed"),
        }
    }
}
