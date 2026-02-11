use crate::shared::user_id::UserId;
use crate::shared::currency::Currency;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserPreferences {
    pub user_id: UserId,
    pub preferred_currency: Currency,
    pub theme: String, 
}

impl UserPreferences {
    pub fn create(user_id: UserId) -> Self {
        Self {
            user_id,
            preferred_currency: Currency::TWD,
            theme: "DEFAULT".to_string(),
        }
    }
}
