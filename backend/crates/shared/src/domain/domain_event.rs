use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait DomainEvent: Send + Sync {
    fn aggregate_root_id(&self) -> String;
    fn event_type(&self) -> &'static str;
    fn event_version(&self) -> &'static str;
}