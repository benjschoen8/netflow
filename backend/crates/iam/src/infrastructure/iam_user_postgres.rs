use async_trait::async_trait;
use sqlx::postgres::PgPool;
use sqlx::Error as SqlxError;
use serde_json;

use shared::domain::{ UserId, SharedError, CorrelationId, AggregateRoot, Message };
use shared::application::RepoStore;
use crate::domain::iam_user::IamUser;
use crate::domain::iam_events::IamEvents;
use crate::domain::iam_error::IamError;

pub struct PostgresIamUserStore {
    pool: PgPool,
}

impl PostgresIamUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RepoStore<IamUser, IamEvents> for PostgresIamUserStore {
    async fn save(
        &self,
        user: IamUser,
        messages: &Vec<Message<IamEvents>>,
    ) -> Result<(), SharedError> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(
            r#"INSERT INTO users (id, base_email, full_email, password_hash)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (id) DO UPDATE
               SET email = EXCLUDED.email,
                   password_hash = EXCLUDED.password_hash"#,
        )
        .bind(user.user_id().to_string())
        .bind(user.email().base_address())
        .bind(user.email().full_address())
        .bind(user.password_hash().to_string())
        .execute(&mut *tx)
        .await?;

        for message in messages {
            let payload = serde_json::to_string(message.data())
                .map_err(|e| SharedError::Serialization(e.to_string()))?;

            sqlx::query(
                r#"INSERT INTO outbox_events (message_id, correlation_id, payload)
                   VALUES ($1, $2, $3)"#,
            )
            .bind(message.message_id().to_string())
            .bind(message.correlation_id().to_string())
            .bind(payload)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}