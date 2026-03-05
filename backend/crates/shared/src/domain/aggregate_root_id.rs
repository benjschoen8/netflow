use uuid::Uuid;

pub trait AggregateRootId: Send + Sync {
    fn uuid(&self) -> Uuid;
}