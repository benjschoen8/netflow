use crate::shared::user_id::UserId;
use crate::shared::phone::Phone;
use crate::shared::email::Email;
use crate::iam::role::Role;
use crate::shared::username::Username;
use crate::iam::password_hash::PasswordHash;
use crate::iam::iam_event::IamEvent;
use crate::shared::event::Event;
use crate::shared::aggregate_root::AggregateRoot;
use crate::iam::iam_error::IamError;
use crate::shared::shared_error::SharedError;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IamUser {
    events: Vec<IamEvent>,
    id: UserId,
    username: Username,
    password_hash: PasswordHash,
    email: Email,
    phone: Option<Phone>,
    role: Role,
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
    ) -> Self {
        Self {
            events: Vec::new(),
            id,
            username,
            password_hash,
            email,
            phone,
            role,
        }
    }

    pub fn register(
        username: Username, 
        password_hash: PasswordHash, 
        email: Email, 
        phone: Option<Phone>,
        role: Role,
    ) -> Self {
        let user = Self {
            events: Vec::new(),
            id: UserId::create(),
            username,
            password_hash,
            email,
            phone,
            role,
        };

        user.record_event(IamEvent::UserRegistered {
            user_id: user.id,
            role: user.role,
        });

        user
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
}