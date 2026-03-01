use crate::domain::shared_error::SharedError;

#[async_trait]
trait HandlerWrapper: Send + Sync {
    async fn handle_event(&self, event: &dyn Any) -> Result<(), SharedError>;
}j