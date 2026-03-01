use std::sync::{Arc, RwLock};

use crate::domain::shared_error::SharedError;
use crate::doamin::domain_event::DomainEvent;
use crate::infrastructure::concrete_handler_wrapper::ConcreteHandlerWrapper;
use crate::infrastructure::handler_wrapper::HandlerWrapper;

#[derive(Default, Clone)]
pub struct InMemoryEventBus {
    subscribers: Arc<RwLock<HashMap<TypeId, Vec<Box<dyn HandlerWrapper>>>>>,
}

impl InMemoryEventBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn subscribe<E, H>(&self, handler: H)
    where
        E: DomainEvent + 'static,
        H: EventHandler<E> + Send + Sync + 'static,
    {
        let mut sub_lock = self.subscribers.write().await;
        let type_id = TypeId::of::<E>();
        let wrapper = Box::new(ConcreteHandlerWrapper {
            handler,
            _marker: std::marker::PhantomData,
        });

        sub_lock.entry(type_id).or_insert_with(Vec::new).push(wrapper);
    }

    pub async fn publish<E>(&self, event: &E) -> Result<(), SharedError>
    where
        E: DomainEvent + 'static,
    {
        let sub_lock = self.subscribers.read().await;
        let type_id = TypeId::of::<E>();

        if let Some(handlers) = sub_lock.get(&type_id) {
            for handler in handlers {
                handler.handle_event(event as &dyn Any).await?;
            }
        }
        Ok(())
    }
}