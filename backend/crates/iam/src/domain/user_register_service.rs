use std::sync::Arc;

use shared::domain::{Username, Email, Phone};
use crate::domain::iam_error::IamError;
use crate::domain::iam_user::IamUser;
use crate::domain::user_repository::UserRepository;
use crate::domain::hasher::Hasher;
use crate::domain::password::Password;

pub struct UserRegisterService {
    user_repo: Arc<dyn UserRepository>,
    hasher: Arc<dyn Hasher>,
}

impl UserRegisterService {
    pub fn new(user_repo: Arc<dyn UserRepository>, hasher: Arc<dyn Hasher>) -> Self {
        Self { user_repo, hasher }
    }

    pub async fn register(
        &self,
        username: Username,
        password: Password,
        email: Email,
        phone: Option<Phone>,
    ) -> Result<IamUser, IamError> {

        if self.user_repo.find_by_username(&username).await?.is_some() {
            return Err(IamError::UsernameExists);
        }

        if self.user_repo.find_by_email(&email).await?.is_some() {
            return Err(IamError::EmailExists);
        }

        let hashed_password = self.hasher.hash(&password)?;


        let user = IamUser::register(
            username,
            hashed_password,
            email,
            phone
        );

        Ok(user)
    }
}