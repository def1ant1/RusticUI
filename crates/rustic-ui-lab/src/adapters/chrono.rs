//! Adapter powered by the `chrono` crate.
//!
//! **Unstable:** Adapter implementations may evolve as real world use cases
//! surface.  This module is behind the `chrono` feature flag.

use super::{DateAdapter, TimeAdapter};

/// Adapter that delegates to the [`chrono`] crate for all date and time
/// computations.  Chrono is widely used and battle tested which makes it a
/// sensible default for server side applications.
pub struct AdapterChrono;

impl DateAdapter for AdapterChrono {
    type Date = chrono::NaiveDate;

    fn today(&self) -> Self::Date {
        chrono::Local::now().date_naive()
    }

    fn add_days(&self, date: &Self::Date, days: i32) -> Self::Date {
        *date + chrono::Duration::days(days as i64)
    }

    fn format(&self, date: &Self::Date) -> String {
        date.to_string()
    }
}

impl TimeAdapter for AdapterChrono {
    type Time = chrono::NaiveTime;

    fn now(&self) -> Self::Time {
        chrono::Local::now().time()
    }

    fn add_minutes(&self, time: &Self::Time, minutes: i32) -> Self::Time {
        *time + chrono::Duration::minutes(minutes as i64)
    }

    fn format(&self, time: &Self::Time) -> String {
        time.format("%H:%M").to_string()
    }
}
