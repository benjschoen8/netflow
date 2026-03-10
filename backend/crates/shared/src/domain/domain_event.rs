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

#[macro_export]
macro_rules! impl_domain_event {
    ($type:ty, $event_type:literal, $domain:literal, $service:literal) => {
        impl shared::domain::DomainEvent for $type {
            fn event_id(&self) -> shared::domain::EventId {
                self.metadata.event_id()
            }
            fn occurred_on(&self) -> shared::domain::Timestamp {
                self.metadata.occurred_on()
            }
            fn event_type(&self) -> &'static str { $event_type }
            fn event_version(&self) -> &'static str { "1.0" }
            fn domain(&self) -> &'static str { $domain }
            fn service(&self) -> &'static str { $service }
        }
    };
}