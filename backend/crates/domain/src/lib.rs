pub mod shared {
    pub mod event_id;
    pub mod user_id;
    pub mod account_id;
    pub mod currency;
    pub mod money;
    pub mod role;
}

pub use crate::shared::event_id::EventId;
pub use crate::shared::user_id::UserId;
pub use crate::shared::account_id::AccountId;
pub use crate::shared::currency::Currency;
pub use crate::shared::money::Money;
pub use crate::shared::role::Role;

pub mod iam {
    pub mod user;
    pub mod user_preferences;
    pub mod iam_error;
}
