use std::sync::Arc;

use shared::domain::{CorrelationId, Username, Email, Phone};
use shared::application::RepoStore;
use crate::domain::iam_error::IamError;
use crate::domain::iam_user::IamUser;
use crate::domain::user_repository::UserRepository;
use crate::domain::hasher::Hasher;
use crate::domain::password::Password;
use crate::domain::user_register_service::UserRegisterService;
use crate::application::register_user_dto::RegisterUserDto;

pub struct UserRegisterUsecase {
    user_repo: Arc<dyn RepoStore<IamUser>>,
    register_service: UserRegisterService,
}

impl UserRegisterUsecase {
    pub fn new(user_repo: Arc<dyn RepoStore<IamUser>>, user_query: Arc<dyn UserRepository>, hasher: Arc<dyn Hasher>) -> Self {
        let register_service = UserRegisterService::new(user_query.clone(), hasher);
        Self { user_repo, register_service }
    }

    pub async fn register(
        &self,
        input: RegisterUserDto,
        correlation_id: CorrelationId,
    ) -> Result<IamUser, IamError> {
        if input.password != input.confirm_password {
            return Err(IamError::PasswordMismatch); 
        }
        
        let username = Username::new(input.username)?;
        let password = Password::new(input.password)?;
        let email = Email::parse(input.email)?;
        
        let phone = match input.phone {
            Some(p) => Some(Phone::new(p)?),
            None => None,
        };

        let user = self.register_service.register(
            username,
            password,
            email,
            phone
        ).await?;

        self.user_repo.save(user.clone(), correlation_id).await?;

        Ok(user)
    }
}