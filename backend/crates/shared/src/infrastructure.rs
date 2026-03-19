#[cfg(feature = "sqlx")]
pub mod conversion;

pub mod in_memory_event_bus;

pub use crate::infrastructure::in_memory_event_bus::InMemoryEventBus;

// Placeholder stubs — not compiled into any active feature path.
mod outbox;
mod integration_envelope_factory;
mod integration_event_metadata;
