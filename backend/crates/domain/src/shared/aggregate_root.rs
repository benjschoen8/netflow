use crate::shared::domain_event::DomainEvent;

pub trait AggregateRoot {
    fn events(&self) -> &[DomainEvent];
    
    fn record_event(&mut self, event: DomainEvent);
    
    fn clear_events(&mut self);
}