pub trait AggregateRoot {
    type Event;

    fn events(&self) -> &[Self::Event];

    fn record_event(&mut self, event: Self::Event);

    fn clear_events(&mut self);
}