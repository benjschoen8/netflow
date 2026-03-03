use tokio::sync::broadcast;
use crate::shared::domain::DomainEvent;

#[derive(Clone)]
pub struct InMemoryEventBus<E> {
    sender: broadcast::Sender<Message<E>>,
}

impl<E> InMemoryEventBus<E>
where
    E: DomainEvent + Clone + Send + Sync + 'static, 
{
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn publish(&self, event: Message<E>) {
        let _ = self.sender.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Message<E>> {
        self.sender.subscribe()
    }
}