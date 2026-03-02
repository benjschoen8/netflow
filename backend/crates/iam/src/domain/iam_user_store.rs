use async_trait::async_trait;
use crate::shared::shared_error::SharedError;
use crate::shared::user_id::UserId;
use crate::iam::iam_user::IamUser;
use crate::correlation_id::CorrelationId; 

#[async_trait]
pub trait IamUserStore: Send + Sync {
    async fn save(&self, user: &mut IamUser, correlation_id: &CorrelationId) -> Result<(), SharedError>;
}