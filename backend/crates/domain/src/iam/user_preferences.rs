use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::shared::user_id::UserId;
use crate::shared::currency::Currency;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserPreferences {
    pub user_id: UserId,
    pub preferred_currency: Currency,
    pub theme: String, 
}

impl UserPreferences {
    pub fn default_for_user(user_id: UserId) -> Self {
        Self {
            user_id,
            preferred_currency: Currency::TWD,
            theme: "SYSTEM".to_string(),
        }
    }
}
