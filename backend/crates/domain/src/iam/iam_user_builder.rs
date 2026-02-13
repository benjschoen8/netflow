use crate::iam::iam_user::IamUser;
use crate::iam::user_id::UserId;
use crate::iam::username::Username;
use crate::iam::password_hash::PasswordHash;
use crate::shared::email::Email;
use crate::shared::phone::Phone;
use crate::iam::role::Role;




#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IamUserBuilder {
        id: Option<UserId>,
        username: Option<Username>,
        password_hash: PasswordHash,
        email: Email,
        phone: Option<Phone>,
        role: Role,
}

pub fn new() -> Self {
    Self { 
        id: None,
        username: None,
        password_hash: None,
        email: None,
        phone: None,
        role: Default::default(),
    }
}

pub fn with_id(&mut self, id: String) -> self {
    self.id = UserId::new(id);
    self
}

pub fn with_username(&mut self, username: String) -> self {
    self.username = Username::new(username);
    self
}

pub fn with_password_hash(&mut self, password_hash: String) -> self {
    self.password_hash = PasswordHash::new(password_hash);
    self
}

pub fn with_email(&mut self, email: String) -> self {
    self.email = Email::parse(email);
    self
}

pub fn with_phone(&mut self, ) -> {

}

pub fn 

pub fn register(
    &self,
    username: Username, 
    password: Password, 
    email: Email, 
    phone: Option<Phone>, 
    role: Role
) -> IamUser {
    Iam
} 
