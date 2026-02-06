use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterUserDto {
    pub username: String,

    pub email: String,
    
    pub phone: String,

    pub password: String,

    pub confirm_password: String,

    pub terms_accepted: bool,
}