use crate::domain::iam_error::IamError;
use crate::domain::password::Password;
use crate::domain::password_hash::PasswordHash;

pub trait Hasher: Send + Sync {
    fn hash(&self, password: &Password) -> Result<PasswordHash, IamError>;

    fn verify(&self, password: &Password, hash: &PasswordHash) -> Result<bool, IamError>;
}