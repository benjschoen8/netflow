use async_trait::async_trait;
use crate::shared::user_id::UserId;
use crate::iam::user::User;
use crate::iam::iam_error::IamError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), IamError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, IamError>;
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, IamError>;
}
