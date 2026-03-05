use shared::domain::{UserId, Username, Phone, Email, AggregateRoot, DomainEvent};
use crate::domain::role::Role;
use crate::domain::password_hash::PasswordHash;
use crate::domain::iam_events::IamEvents;
use crate::domain::events::user_registered::UserRegistered;
use crate::domain::user_status::UserStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IamUser {
    events: Vec<IamEvents>,
    user_id: UserId,
    username: Username,
    password_hash: PasswordHash,
    email: Email,
    phone: Option<Phone>,
    role: Role,
    user_status: UserStatus,
}

impl AggregateRoot for IamUser {
    type Event = IamEvents;
    fn events(&self) -> &[Self::Event] {
        &self.events
    }

    fn record_event(&mut self, event: Self::Event) {
        self.events.push(event);
    }

    fn pull_events(&mut self) -> Vec<Self::Event> {
        std::mem::take(&mut self.events) 
    }
}

impl IamUser {
    pub fn restore(
        user_id: UserId,
        username: Username, 
        password_hash: PasswordHash, 
        email: Email, 
        phone: Option<Phone>,
        role: Role,
        user_status: UserStatus,
    ) -> Self {
        Self {
            events: Vec::new(),
            user_id,
            username,
            password_hash,
            email,
            phone,
            role,
            user_status,
        }
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn phone(&self) -> Option<&Phone> {
        self.phone.as_ref()
    }

    pub fn status(&self) -> UserStatus {
        self.user_status
    }
    
    pub fn register(
        username: Username, 
        password_hash: PasswordHash, 
        email: Email, 
        phone: Option<Phone>,
    ) -> Self {
        let mut user = Self {
            events: Vec::new(),
            user_id: UserId::create(),
            username,
            password_hash,
            email,
            phone,
            role: Role::default(),
            user_status: UserStatus::default(),
        };
        let event_data = UserRegistered::user_registered(user.user_id, user.username.clone(), user.email.clone(), user.phone.clone(), user.role);
        user.record_event(IamEvents::UserRegistered(event_data));

        user
    }
}