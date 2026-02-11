pub trait AggregateRoot {
    fn events(&self) -> &[DomainEvent];
    fn clear_events(&mut self);
}