use chrono::{Datelike, Duration, NaiveDate, Weekday};

/// Represents a week in the quarter
#[derive(Debug, Clone, PartialEq)]
pub struct QuarterWeek {
    /// Start date of the week (Monday)
    pub start_date: NaiveDate,
    /// Week number within the quarter (1-based, e.g., 1-13)
    pub week_number: usize,
    /// Sprint number (1-based, calculated from sprint length)
    pub sprint_number: usize,
    /// Total weeks in the quarter
    pub total_weeks: usize,
    /// Sprint length in weeks
    pub sprint_length_weeks: usize,
}

impl QuarterWeek {
    /// Format for display: "Week 1", "Week 2", etc.
    pub fn format_week_number(&self) -> String {
        format!("Week {}", self.week_number)
    }

    /// Format sprint number: "Sprint 1", "Sprint 2", etc.
    #[allow(dead_code)] // Reserved for future sprint-based UI features
    pub fn format_sprint_number(&self) -> String {
        format!("Sprint {}", self.sprint_number)
    }

    /// Format the date as "Jan 3" or "Jan 3 (W)" if it's a Wednesday
    pub fn format_date(&self, include_weekday: bool) -> String {
        let formatted = self.start_date.format("%b %-d").to_string();
        if include_weekday && self.start_date.weekday() == Weekday::Wed {
            format!("{} (W)", formatted)
        } else {
            formatted
        }
    }

    /// Check if this is the first week of a sprint (for UI separator rendering)
    pub fn is_sprint_start(&self) -> bool {
        (self.week_number - 1).is_multiple_of(self.sprint_length_weeks)
    }
}

/// Generate a list of weeks for a quarter starting from the given date
///
/// # Arguments
/// * `quarter_start` - The first Monday of the quarter
/// * `num_weeks` - Number of weeks in the quarter (typically 13)
/// * `sprint_length_weeks` - Number of weeks in each sprint (typically 2)
///
/// # Returns
/// A vector of `QuarterWeek` structs, one for each week
pub fn generate_quarter_weeks(
    quarter_start: NaiveDate,
    num_weeks: usize,
    sprint_length_weeks: usize,
) -> Vec<QuarterWeek> {
    (0..num_weeks)
        .map(|week_index| {
            let start_date = quarter_start + Duration::weeks(week_index as i64);
            let week_number = week_index + 1;
            // Calculate sprint number based on configurable sprint length
            let sprint_number = (week_index / sprint_length_weeks) + 1;

            QuarterWeek {
                start_date,
                week_number,
                sprint_number,
                total_weeks: num_weeks,
                sprint_length_weeks,
            }
        })
        .collect()
}

/// Find the first Monday on or after the given date
/// Useful for ensuring week starts on Monday
#[allow(dead_code)] // Reserved for future use when normalizing dates to Monday
pub fn find_first_monday(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday();
    match weekday {
        Weekday::Mon => date,
        Weekday::Tue => date + Duration::days(6),
        Weekday::Wed => date + Duration::days(5),
        Weekday::Thu => date + Duration::days(4),
        Weekday::Fri => date + Duration::days(3),
        Weekday::Sat => date + Duration::days(2),
        Weekday::Sun => date + Duration::days(1),
    }
}

/// Get the quarter start date for Q1-Q4 of a given year
/// Returns the actual first day of the quarter:
/// Q1: Jan 1, Q2: Apr 1, Q3: Jul 1, Q4: Oct 1
pub fn get_quarter_start_date(year: i32, quarter: u8) -> Option<NaiveDate> {
    let month = match quarter {
        1 => 1,  // January
        2 => 4,  // April
        3 => 7,  // July
        4 => 10, // October
        _ => return None,
    };

    NaiveDate::from_ymd_opt(year, month, 1)
}

/// Calculate the number of full weeks between two dates
#[allow(dead_code)] // Reserved for future date calculations
pub fn weeks_between(start: NaiveDate, end: NaiveDate) -> f32 {
    let days = (end - start).num_days();
    days as f32 / 7.0
}

/// Check if a date falls within a specific week
#[allow(dead_code)] // Reserved for future date validation
pub fn is_date_in_week(date: NaiveDate, week_start: NaiveDate) -> bool {
    let week_end = week_start + Duration::days(6);
    date >= week_start && date <= week_end
}

/// Get the Monday of the week containing the given date
#[allow(dead_code)] // Reserved for future date normalization
pub fn get_week_start(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday();
    let days_since_monday = match weekday {
        Weekday::Mon => 0,
        Weekday::Tue => 1,
        Weekday::Wed => 2,
        Weekday::Thu => 3,
        Weekday::Fri => 4,
        Weekday::Sat => 5,
        Weekday::Sun => 6,
    };
    date - Duration::days(days_since_monday)
}

/// Get the next quarter start info based on the given date
///
/// Returns the quarter that either:
/// - Contains the current date (if we're in the first week of a quarter), OR
/// - Is the upcoming quarter (if we're past the first week)
///
/// For simplicity, we return the quarter whose start date is >= today
/// If today is past the Q4 start, we return Q1 of next year
///
/// # Returns
/// (year, quarter_number, start_date, quarter_name) e.g., (2025, 1, Jan 6, "Q1 2025")
pub fn get_next_quarter_info(today: NaiveDate) -> (i32, u8, NaiveDate, String) {
    let year = today.year();

    // Check each quarter of the current year
    for quarter in 1..=4 {
        if let Some(start) = get_quarter_start_date(year, quarter) {
            if start >= today {
                let name = format!("Q{} {}", quarter, year);
                return (year, quarter, start, name);
            }
        }
    }

    // If we're past Q4, return Q1 of next year
    let next_year = year + 1;
    let start = get_quarter_start_date(next_year, 1).expect("Q1 should always be valid");
    let name = format!("Q1 {}", next_year);
    (next_year, 1, start, name)
}

/// Calculate sprint boundaries (start and end dates) for a given week
///
/// # Arguments
/// * `week_start` - The Monday start date of a week
/// * `quarter_start` - The first Monday of the quarter
/// * `sprint_length_weeks` - Number of weeks in each sprint (typically 2)
///
/// # Returns
/// (sprint_start_date, sprint_end_date) where end date is the last day (Sunday) of the sprint
pub fn get_sprint_boundaries(
    week_start: NaiveDate,
    quarter_start: NaiveDate,
    sprint_length_weeks: usize,
) -> (NaiveDate, NaiveDate) {
    // Calculate which week of the quarter this is (0-indexed)
    let days_since_quarter_start = (week_start - quarter_start).num_days();
    let week_index = (days_since_quarter_start / 7) as usize;

    // Calculate which sprint this week belongs to (0-indexed)
    let sprint_index = week_index / sprint_length_weeks;

    // Calculate the start of this sprint (first Monday)
    let sprint_start = quarter_start + Duration::weeks((sprint_index * sprint_length_weeks) as i64);

    // Calculate the end of this sprint (last Sunday)
    let sprint_end = sprint_start + Duration::weeks(sprint_length_weeks as i64) - Duration::days(1);

    (sprint_start, sprint_end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_monday() {
        // Jan 1, 2025 is a Wednesday
        let jan_1 = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let first_monday = find_first_monday(jan_1);
        // Should be Jan 6, 2025 (Monday)
        assert_eq!(first_monday, NaiveDate::from_ymd_opt(2025, 1, 6).unwrap());
        assert_eq!(first_monday.weekday(), Weekday::Mon);
    }

    #[test]
    fn test_get_quarter_start_date() {
        // Q1 2025 - Jan 1, 2025
        let q1 = get_quarter_start_date(2025, 1).unwrap();
        assert_eq!(q1.month(), 1);
        assert_eq!(q1.day(), 1);

        // Q2 2025 - Apr 1, 2025
        let q2 = get_quarter_start_date(2025, 2).unwrap();
        assert_eq!(q2.month(), 4);
        assert_eq!(q2.day(), 1);

        // Q3 2025 - Jul 1, 2025
        let q3 = get_quarter_start_date(2025, 3).unwrap();
        assert_eq!(q3.month(), 7);
        assert_eq!(q3.day(), 1);

        // Q4 2025 - Oct 1, 2025
        let q4 = get_quarter_start_date(2025, 4).unwrap();
        assert_eq!(q4.month(), 10);
        assert_eq!(q4.day(), 1);
    }

    #[test]
    fn test_generate_quarter_weeks() {
        let start = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap(); // Monday
        let weeks = generate_quarter_weeks(start, 13, 2); // 2-week sprints

        assert_eq!(weeks.len(), 13);
        assert_eq!(weeks[0].week_number, 1);
        assert_eq!(weeks[0].sprint_number, 1);
        assert_eq!(weeks[1].sprint_number, 1); // Still sprint 1
        assert_eq!(weeks[2].sprint_number, 2); // Sprint 2 starts at week 3
        assert_eq!(weeks[12].week_number, 13);
        assert_eq!(weeks[0].sprint_length_weeks, 2);
    }

    #[test]
    fn test_weeks_between() {
        let start = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();
        let end = NaiveDate::from_ymd_opt(2025, 1, 20).unwrap();
        let weeks = weeks_between(start, end);
        assert_eq!(weeks, 2.0);
    }

    #[test]
    fn test_get_week_start() {
        // Wednesday, Jan 8, 2025
        let wed = NaiveDate::from_ymd_opt(2025, 1, 8).unwrap();
        let monday = get_week_start(wed);
        assert_eq!(monday, NaiveDate::from_ymd_opt(2025, 1, 6).unwrap());
        assert_eq!(monday.weekday(), Weekday::Mon);
    }

    #[test]
    fn test_get_sprint_boundaries() {
        let quarter_start = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap(); // Monday, Jan 6
        let sprint_length = 2; // 2-week sprints

        // Week 1 (Jan 6) - should be in Sprint 1 (Jan 6 - Jan 19)
        let week1 = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();
        let (start, end) = get_sprint_boundaries(week1, quarter_start, sprint_length);
        assert_eq!(start, NaiveDate::from_ymd_opt(2025, 1, 6).unwrap()); // Monday
        assert_eq!(end, NaiveDate::from_ymd_opt(2025, 1, 19).unwrap()); // Sunday

        // Week 2 (Jan 13) - still in Sprint 1
        let week2 = NaiveDate::from_ymd_opt(2025, 1, 13).unwrap();
        let (start, end) = get_sprint_boundaries(week2, quarter_start, sprint_length);
        assert_eq!(start, NaiveDate::from_ymd_opt(2025, 1, 6).unwrap());
        assert_eq!(end, NaiveDate::from_ymd_opt(2025, 1, 19).unwrap());

        // Week 3 (Jan 20) - should be in Sprint 2 (Jan 20 - Feb 2)
        let week3 = NaiveDate::from_ymd_opt(2025, 1, 20).unwrap();
        let (start, end) = get_sprint_boundaries(week3, quarter_start, sprint_length);
        assert_eq!(start, NaiveDate::from_ymd_opt(2025, 1, 20).unwrap());
        assert_eq!(end, NaiveDate::from_ymd_opt(2025, 2, 2).unwrap());
    }
}
