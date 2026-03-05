use async_trait::async_trait;
use shared::domain::{UserId, Username, Email};
use crate::domain::iam_user::IamUser;
use crate::domain::iam_error::IamError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_username(&self, username: &Username) -> Result<Option<IamUser>, IamError>;

    async fn find_by_email(&self, email: &Email) -> Result<Option<IamUser>, IamError>;

    async fn find_by_id(&self, id: UserId) -> Result<Option<IamUser>, IamError>;
}