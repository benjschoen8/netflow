use domain::iam::preferences::UserPreferences;
use uuid::Uuid;
use serde_json::Value;

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
    assert!(widgets.contains(&Value::String("spending_summary".to_string())));
}