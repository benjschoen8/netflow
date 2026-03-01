use crate::infrastructure::handler_wrapper::HandlerWrapper;
use crate::domain::shared_error::SharedError

struct ConcreteHandlerWrapper<E, H> {
    handler: H,
    _marker: std::marker::PhantomData<E>,
}

#[async_trait]
impl<E, H> HandlerWrapper for ConcreteHandlerWrapper<E, H>
where
    E: DomainEvent + 'static,
    H: EventHandler<E> + Send + Sync + 'static,
{
    async fn handle_event(&self, event: &dyn Any) -> Result<(), SharedError> {
        if let Some(concrete_event) = event.downcast_ref::<E>() {
            self.handler.handle(concrete_event).await
        } else {
            Err(SharedError::UnexpectedError("Event type mismatch".into()))
        }
    }
}