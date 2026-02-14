
#[derive(Debug)]
pub struct RegisterUserDto {
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub password: String,
    pub confirm_password: String,
    pub terms_accepted: bool,
}