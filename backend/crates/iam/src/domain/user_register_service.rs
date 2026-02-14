use crate::iam_user::IamUser;
use crate::hasher::Hasher;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserRegisterService {
    hasher: Hasher,
    builder: IamUserBuilder,
}

pub fn new(hasher: Hasher) -> Self {
    Self { hasher }
}

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
