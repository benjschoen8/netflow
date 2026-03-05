use std::sync::Arc;

use shared::domain::{AggregateRoot, CorrelationId, Username, Email, Phone, Message};
use shared::application::{EventBus, RepoStore, MessageMapper};
use crate::domain::iam_error::IamError;
use crate::domain::iam_events::IamEvents;
use crate::domain::iam_user::IamUser;
use crate::domain::user_repository::UserRepository;
use crate::domain::hasher::Hasher;
use crate::domain::password::Password;
use crate::domain::user_register_service::UserRegisterService;
use crate::application::register_user_dto::RegisterUserDto;

pub struct UserRegisterUsecase {
    event_bus: Arc<dyn EventBus<Message = Message<IamEvents>>>,
    user_repo: Arc<dyn RepoStore<IamUser, IamEvents>>,
    register_service: UserRegisterService,
}

impl UserRegisterUsecase {
    pub fn new(
        event_bus: Arc<dyn EventBus<Message = Message<IamEvents>>>, 
        user_repo: Arc<dyn RepoStore<IamUser, IamEvents>>, 
        user_query: Arc<dyn UserRepository>, 
        hasher: Arc<dyn Hasher>, 
    ) -> Self {
        let register_service = UserRegisterService::new(user_query, hasher);
        Self { event_bus, user_repo, register_service }
    }

    pub async fn register(
        &self,
        input: RegisterUserDto,
        correlation_id: &CorrelationId,
    ) -> Result<(), IamError> {
        
        if !input.terms_accepted() {
            return Err(IamError::TermsNotAccepted);
        }

        if input.password() != input.confirm_password() {
            return Err(IamError::PasswordMismatch); 
        }
        
        let username = Username::new(input.username().to_string())?;
        let password = Password::new(input.password().to_string())?;
        let email = Email::parse(input.email().to_string())?;
        
        let phone = input.phone()
                        .map(|p| Phone::new(p.to_string()))
                        .transpose()?;

        let mut user = self.register_service.register(
            username,
            password,
            email,
            phone
        ).await?;

        let messages = MessageMapper::map(user.pull_events(), &correlation_id);

        self.user_repo.save(user, &messages).await?;
        
        self.event_bus.publish(messages).await?;
        
        Ok(())
    }
}