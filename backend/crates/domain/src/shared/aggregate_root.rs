use crate::shared::event::Event;

pub trait AggregateRoot {
    fn events(&self) -> &[Event];

    fn record_event<T: Event>(&mut self, event: T);

    fn clear_events(&mut self);
}