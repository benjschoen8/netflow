pub trait DomainEvent: {
    fn event_type(&self) -> &'static str;
    fn event_version(&self) -> &'static str;
}