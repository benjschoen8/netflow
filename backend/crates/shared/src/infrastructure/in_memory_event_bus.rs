use tokio::sync::broadcast;
use crate::domain::domain_event::DomainEvent;
use crate::domain::shared_error::SharedError;
use crate::domain::message::Message;
use crate::application::event_bus::EventBus;

#[derive(Clone)]
pub struct InMemoryEventBus<E>
where
    E: DomainEvent + Clone + Send + Sync + 'static,
{
    sender: broadcast::Sender<Message<E>>,
}

impl<E> EventBus for InMemoryEventBus<E>
where E: DomainEvent + Clone + Send + Sync + 'static {
    type Message = Message<E>;

    async fn publish(&self, messages: Vec<Self::Message>) -> Result<(), SharedError> {
        for message in messages {
            self.sender.send(message)
                .map_err(|_| SharedError::EventPublishFailed(format!("Failed to publish message: {}", message.message_id.to_string())))?;
        }
        Ok(())
    }
}


impl<E> InMemoryEventBus<E>
where E: DomainEvent + Clone + Send + Sync + 'static {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Message<E>> {
        self.sender.subscribe()
    }
}