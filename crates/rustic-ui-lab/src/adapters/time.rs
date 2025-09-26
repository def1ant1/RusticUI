//! Adapter powered by the `time` crate.
//!
//! **Unstable:** This adapter is experimental. Feedback from production
//! deployments will shape future revisions.

use super::{DateAdapter, TimeAdapter};

/// Adapter that delegates to the lightweight [`time`] crate which has a small
/// dependency graph and works well in `no_std` environments.
pub struct AdapterTime;

impl DateAdapter for AdapterTime {
    type Date = time::Date;

    fn today(&self) -> Self::Date {
        time::OffsetDateTime::now_utc().date()
    }

    fn add_days(&self, date: &Self::Date, days: i32) -> Self::Date {
        *date + time::Duration::days(days as i64)
    }

    fn format(&self, date: &Self::Date) -> String {
        date.to_string()
    }
}

impl TimeAdapter for AdapterTime {
    type Time = time::Time;

    fn now(&self) -> Self::Time {
        time::OffsetDateTime::now_utc().time()
    }

    fn add_minutes(&self, time: &Self::Time, minutes: i32) -> Self::Time {
        *time + time::Duration::minutes(minutes as i64)
    }

    fn format(&self, time: &Self::Time) -> String {
        time.format(&time::macros::format_description!("%H:%M"))
            .unwrap()
    }
}
