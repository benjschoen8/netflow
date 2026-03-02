use async_trait::async_trait;
use sqlx::PgPool;
use serde_json;

use domain::iam::iam_user_store::IamUserStore;
use domain::iam::iam_user::IamUser;
use domain::shared::user_id::UserId;
use domain::shared::shared_error::SharedError;
use domain::correlation_id::CorrelationId;
use domain::shared::aggregate_root::AggregateRoot;
use crate::message::Message;

pub struct PostgresIamUserStore {
    pool: PgPool,
}

impl PostgresIamUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IamUserStore for PostgresIamUserStore {
    async fn save(&self, user: &mut IamUser, correlation_id: &CorrelationId) -> Result<(), SharedError> {
        let mut tx = self.pool.begin().await.map_err(|e| SharedError::Database(e.to_string()))?;

        sqlx::query!(
            r#"
            INSERT INTO users (id, email, password_hash)
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO UPDATE 
            SET email = EXCLUDED.email, password_hash = EXCLUDED.password_hash
            "#,
            user.id().into_inner(),
            user.email().as_str(),
            user.password_hash().as_str()
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| SharedError::Database(e.to_string()))?;

        let events = user.clear_events(); 

        for event in events {
            let message = Message::new(event, correlation_id.clone());
            
            let payload = serde_json::to_string(&message).map_err(|_| {
                SharedError::Serialization("Failed to serialize Outbox Message".into())
            })?;

            sqlx::query!(
                r#"
                INSERT INTO outbox_events (message_id, correlation_id, payload)
                VALUES ($1, $2, $3)
                "#,
                message.message_id.into_inner(), 
                correlation_id.as_str(),         
                payload
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| SharedError::Database(e.to_string()))?;
        }

        tx.commit().await.map_err(|e| SharedError::Database(e.to_string()))?;

        Ok(())
    }
}