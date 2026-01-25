use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserPreferences {
    pub user_id: Uuid,
    
    // Default: "TWD"
    pub preferred_currency: String,
    
    // Default: "SYSTEM", "LIGHT", or "DARK"
    pub theme: String,
    
    // Flexible JSON blob for widget positions
    pub dashboard_layout: JsonValue,
}

impl UserPreferences {
    pub fn default_for_user(user_id: Uuid) -> Self {
        Self {
            user_id,
            preferred_currency: "TWD".to_string(),
            theme: "SYSTEM".to_string(),
            dashboard_layout: serde_json::json!({
                "widgets": ["spending_summary", "quick_add"],
                "hidden": []
            }),
        }
    }
}