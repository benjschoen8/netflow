use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// --- INTEGRATION: Import types from Shared Kernel ---
use crate::shared::user_id::UserId;
use crate::shared::currency::Currency; // Use the Enum, never String!

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserPreferences {
    pub user_id: UserId,
    
    // 1. Strict Typing:
    // This guarantees the preference matches a real currency the system supports.
    pub preferred_currency: Currency,
    
    // 2. UI Theme:
    // Keeping this as String is acceptable for now, but an Enum is safer long-term.
    pub theme: String, 
    
    // 3. Flexible Layout:
    // Perfect use case for JSONB.
    pub dashboard_layout: JsonValue,
}

impl UserPreferences {
    pub fn default_for_user(user_id: UserId) -> Self {
        Self {
            user_id,
            
            // Strict Default
            preferred_currency: Currency::TWD,
            
            theme: "SYSTEM".to_string(),
            
            // Standard Default Widgets
            dashboard_layout: serde_json::json!({
                "widgets": ["spending_summary", "quick_add", "net_worth_card"],
                "hidden": []
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::currency::Currency;
    use crate::shared::user_id::UserId;

    #[test]
    fn test_preferences_defaults_strictly_typed() {
        // 1. Arrange
        let user_id = UserId::new();

        // 2. Act
        let prefs = UserPreferences::default_for_user(user_id);

        // 3. Assert
        assert_eq!(prefs.user_id, user_id);
        
        // Asserting against the Enum, not a String
        assert_eq!(prefs.preferred_currency, Currency::TWD, "Default must be TWD enum variant");
        assert_eq!(prefs.theme, "SYSTEM");
    }

    #[test]
    fn test_dashboard_layout_structure() {
        let user_id = UserId::new();
        let prefs = UserPreferences::default_for_user(user_id);

        let layout = &prefs.dashboard_layout;
        
        // Check structure
        assert!(layout.get("widgets").is_some());
        
        let widgets = layout["widgets"].as_array().unwrap();
        
        // Verify defaults exist
        assert!(widgets.contains(&serde_json::json!("spending_summary")));
        assert!(widgets.contains(&serde_json::json!("net_worth_card")));
    }
}
