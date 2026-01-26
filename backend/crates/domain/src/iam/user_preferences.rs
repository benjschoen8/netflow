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


#[test]
fn test_preferences_defaults() {
    // 1. Arrange
    let user_id = Uuid::new_v4();

    // 2. Act
    let prefs = UserPreferences::default_for_user(user_id);

    // 3. Assert
    assert_eq!(prefs.user_id, user_id, "Preferences must link to the correct user");
    assert_eq!(prefs.preferred_currency, "TWD", "Default currency should be TWD");
    assert_eq!(prefs.theme, "SYSTEM", "Default theme should be SYSTEM");
}

#[test]
fn test_dashboard_layout_structure() {
    let user_id = Uuid::new_v4();
    let prefs = UserPreferences::default_for_user(user_id);

    // Verify the JSON blob structure
    let layout = &prefs.dashboard_layout;
    
    // Check if "widgets" exists and is an array
    assert!(layout.get("widgets").is_some(), "Layout must have widgets");
    assert!(layout["widgets"].is_array(), "Widgets must be a list");
    
    // Optional: Check specific default widgets
    let widgets = layout["widgets"].as_array().unwrap();
    assert!(widgets.contains(&JsonValue::String("spending_summary".to_string())));
}