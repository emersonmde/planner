use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

use super::TeamMember;
use crate::utils::get_next_quarter_info;

/// Current schema version for Preferences
pub const PREFERENCES_SCHEMA_VERSION: &str = "1.0";

/// Default schema version for deserialization (handles pre-versioned data)
fn default_schema_version() -> String {
    PREFERENCES_SCHEMA_VERSION.to_string()
}

/// Team preferences - persisted to localStorage
/// These settings are long-term and shared across all quarters
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Preferences {
    /// Schema version for migration support
    /// Defaults to "1.0" when loading data without this field (pre-1.0 or 1.0 data)
    #[serde(default = "default_schema_version")]
    pub schema_version: String,

    /// Team name (e.g., "Backend Team", "Data Science")
    /// Used for identifying exports in multi-team aggregation (v2.0)
    pub team_name: String,

    /// Team roster (engineers and scientists)
    pub team_members: Vec<TeamMember>,

    /// Global sprint anchor date
    /// All sprints are calculated relative to this date
    /// Typically the first Monday of the company's sprint cycle
    pub sprint_anchor_date: NaiveDate,

    /// Sprint length in weeks (typically 2)
    pub sprint_length_weeks: usize,

    /// Default capacity per team member in weeks
    /// Used when creating new team members
    pub default_capacity: f32,
}

#[allow(dead_code)] // Validation used in M14
impl Preferences {
    /// Create new preferences with default values
    /// Sprint anchor defaults to the start of the next quarter
    pub fn new(team_name: String) -> Self {
        let today = Local::now().date_naive();
        let (_, _, quarter_start, _) = get_next_quarter_info(today);

        Self {
            schema_version: PREFERENCES_SCHEMA_VERSION.to_string(),
            team_name,
            team_members: Vec::new(),
            sprint_anchor_date: quarter_start,
            sprint_length_weeks: 2,
            default_capacity: 12.0,
        }
    }

    /// Validate preferences
    pub fn validate(&self) -> Result<(), PreferencesValidationError> {
        if self.team_name.trim().is_empty() {
            return Err(PreferencesValidationError::EmptyTeamName);
        }

        if self.sprint_length_weeks == 0 || self.sprint_length_weeks > 4 {
            return Err(PreferencesValidationError::InvalidSprintLength(
                self.sprint_length_weeks,
            ));
        }

        if self.default_capacity <= 0.0 {
            return Err(PreferencesValidationError::InvalidDefaultCapacity(
                self.default_capacity,
            ));
        }

        Ok(())
    }
}

impl Default for Preferences {
    fn default() -> Self {
        Self::new("My Team".to_string())
    }
}

/// Validation errors for preferences
#[allow(dead_code)] // Used in M14 for validation
#[derive(Debug, Clone, PartialEq)]
pub enum PreferencesValidationError {
    EmptyTeamName,
    InvalidSprintLength(usize),
    InvalidDefaultCapacity(f32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preferences_creation() {
        let prefs = Preferences::new("Backend Team".to_string());
        assert_eq!(prefs.schema_version, "1.0");
        assert_eq!(prefs.team_name, "Backend Team");
        assert_eq!(prefs.team_members.len(), 0);
        assert_eq!(prefs.sprint_length_weeks, 2);
        assert_eq!(prefs.default_capacity, 12.0);
    }

    #[test]
    fn test_preferences_default() {
        let prefs = Preferences::default();
        assert_eq!(prefs.schema_version, "1.0");
        assert_eq!(prefs.team_name, "My Team");
        assert!(prefs.validate().is_ok());
    }

    #[test]
    fn test_schema_version_default_on_legacy_data() {
        // Simulate loading data saved before schema_version was added
        let legacy_json = r#"{
            "team_name": "Legacy Team",
            "team_members": [],
            "sprint_anchor_date": "2024-01-01",
            "sprint_length_weeks": 2,
            "default_capacity": 12.0
        }"#;

        let prefs: Preferences = serde_json::from_str(legacy_json).unwrap();
        assert_eq!(prefs.schema_version, "1.0"); // Should default to 1.0
        assert_eq!(prefs.team_name, "Legacy Team");
    }

    #[test]
    fn test_schema_version_preserved_on_load() {
        // Ensure explicit schema_version is preserved (for future versions)
        let json_with_version = r#"{
            "schema_version": "2.0",
            "team_name": "Future Team",
            "team_members": [],
            "sprint_anchor_date": "2024-01-01",
            "sprint_length_weeks": 2,
            "default_capacity": 12.0
        }"#;

        let prefs: Preferences = serde_json::from_str(json_with_version).unwrap();
        assert_eq!(prefs.schema_version, "2.0"); // Should preserve the version
    }

    #[test]
    fn test_validation_empty_team_name() {
        let prefs = Preferences {
            team_name: "".to_string(),
            ..Default::default()
        };
        assert_eq!(
            prefs.validate(),
            Err(PreferencesValidationError::EmptyTeamName)
        );
    }

    #[test]
    fn test_validation_invalid_sprint_length() {
        let prefs_zero_weeks = Preferences {
            sprint_length_weeks: 0,
            ..Default::default()
        };
        assert!(matches!(
            prefs_zero_weeks.validate(),
            Err(PreferencesValidationError::InvalidSprintLength(0))
        ));

        let prefs_too_long = Preferences {
            sprint_length_weeks: 5,
            ..Default::default()
        };
        assert!(matches!(
            prefs_too_long.validate(),
            Err(PreferencesValidationError::InvalidSprintLength(5))
        ));
    }

    #[test]
    fn test_validation_invalid_capacity() {
        let prefs_zero_capacity = Preferences {
            default_capacity: 0.0,
            ..Default::default()
        };
        assert!(matches!(
            prefs_zero_capacity.validate(),
            Err(PreferencesValidationError::InvalidDefaultCapacity(_))
        ));

        let prefs_negative_capacity = Preferences {
            default_capacity: -1.0,
            ..Default::default()
        };
        assert!(matches!(
            prefs_negative_capacity.validate(),
            Err(PreferencesValidationError::InvalidDefaultCapacity(_))
        ));
    }
}
