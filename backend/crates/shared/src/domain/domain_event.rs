use enum_dispatch::enum_dispatch;
use crate::domain::event_id::EventId;
use crate::domain::timestamp::Timestamp;

#[enum_dispatch]
pub trait DomainEvent: Send + Sync {
    fn event_id(&self)-> EventId;
    fn occurred_on(&self)-> Timestamp;
    fn event_type(&self) -> &'static str;
    fn event_version(&self) -> &'static str;
    fn domain(&self) -> &'static str;
    fn service(&self) -> &'static str;
}