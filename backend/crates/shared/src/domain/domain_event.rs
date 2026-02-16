use enum_dispatch::enum_dispatch;
use crate::domain::event_id::EventId;

#[enum_dispatch]
pub trait DomainEvent: Send + Sync {
    fn event_id(&self) -> EventId;
    fn event_type(&self) -> &'static str;
    fn event_version(&self) -> &'static str;
    fn doamin(&self) -> &'static str;
    fn service(&self) -> &'static str;
}