use async_trait::async_trait;
use crate::shared::user_id::UserId;
use crate::iam::refresh_token::RefreshToken;
use crate::iam::iam_error::IamError;

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn save(&self, token: &RefreshToken) -> Result<(), IamError>;
    async fn find_by_hash(&self, token_hash: &str) -> Result<Option<RefreshToken>, IamError>;
    async fn revoke_all_for_user(&self, user_id: UserId) -> Result<(), IamError>;
    async fn delete_expired(&self) -> Result<(), IamError>;
}
