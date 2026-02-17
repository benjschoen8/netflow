#[derive(Clone, Default)]
pub struct InMemoryEventBus {
    subscribers: Arc<RwLock<HashMap<TypeId, Vec<Box<dyn ErasedEventHandler>>>>>,
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
        let mut subscribers = self.subscribers.write().await;
        let type_id = TypeId::of::<E>();
        
        let wrapper = EventHandlerWrapper {
            handler,
            _marker: std::marker::PhantomData,
        };

        subscribers
            .entry(type_id)
            .or_insert_with(Vec::new)
            .push(Box::new(wrapper));
    }
}

#[async_trait]
impl EventBus for InMemoryEventBus {
    async fn publish(
        &self, 
        event: &(dyn DomainEvent + Send + Sync), 
        correlation_id: CorrelationId
    ) -> Result<(), SharedError> {
        let subscribers = self.subscribers.read().await;
        
        if let Some(handlers) = subscribers.get(&type_id) {
            for handler in handlers {
                if let Err(e) = handler.handle(event.as_any(), correlation_id.clone()).await {
                    eprintln!("Error handling event: {:?}", e); 
                }
            }
        }
        
        Ok(())
    }
}