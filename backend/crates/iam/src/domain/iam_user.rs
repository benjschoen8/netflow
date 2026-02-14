use shared::{UserId, Username, Phone, Email, AggregateRoot};
use crate::domain::role::Role;
use crate::domain::password_hash::PasswordHash;
use crate::domain::iam_event::IamEvent;
use crate::domain::user_status::UserStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IamUser {
    events: Vec<IamEvent>,
    id: UserId,
    username: Username,
    password_hash: PasswordHash,
    email: Email,
    phone: Option<Phone>,
    role: Role,
    user_status: UserStatus,
}

impl AggregateRoot for IamUser {
    type Event = IamEvent;
    fn events(&self) -> &[Self::Event] {
        &self.events
    }

    fn record_event(&mut self, event: Self::Event) {
        self.events.push(event);
    }

    fn clear_events(&mut self) {
        self.events.clear();
    }
}

impl IamUser {
    pub fn restore(
        id: UserId,
        username: Username, 
        password_hash: PasswordHash, 
        email: Email, 
        phone: Option<Phone>,
        role: Role,
        user_status: UserStatus,
    ) -> Self {
        Self {
            events: Vec::new(),
            id,
            username,
            password_hash,
            email,
            phone,
            role,
            user_status,
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn username(&self) -> &Username {
        &self.username
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

    pub fn status(&self) -> &UserStatus {
        &self.user_status
    }
    
    pub fn register(
        username: Username, 
        password_hash: PasswordHash, 
        email: Email, 
        phone: Option<Phone>,
    ) -> Self {
        let mut user = Self {
            events: Vec::new(),
            id: UserId::create(),
            username,
            password_hash,
            email,
            phone,
            role: Role::default(),
            user_status: UserStatus::default(),
        };

        user.record_event(IamEvent::user_registered(user.id, user.username.clone(), user.email.clone(), user.role));

        user
    }
}