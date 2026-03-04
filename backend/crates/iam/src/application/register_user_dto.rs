use serde::{Deserialize};

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterUserDto {
    username: String,
    email: String,
    phone: Option<String>,
    password: String,
    confirm_password: String,
    terms_accepted: bool,
}

impl RegisterUserDto {
    pub fn username(&self) -> &str { &self.username }
    pub fn email(&self) -> &str { &self.email }
    pub fn phone(&self) -> Option<&str> { self.phone.as_deref() }
    pub fn password(&self) -> &str { &self.password }
    pub fn confirm_password(&self) -> &str { &self.confirm_password }
    pub fn terms_accepted(&self) -> bool { self.terms_accepted }
}