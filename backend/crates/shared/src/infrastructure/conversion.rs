use sqlx::Error as SqlxError;
use crate::domain::shared_error::SharedError;

impl From<SqlxError> for SharedError {
    fn from(e: SqlxError) -> Self {
        SharedError::Database(e.to_string())
    }
}