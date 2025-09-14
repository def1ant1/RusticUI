//! Trait-based date API so widgets can remain agnostic about the
//! underlying date/time library.  This mirrors the adapter pattern used
//! in the upstream MUI project and enables the community to plug in their
//! preferred chrono or time implementation without code changes.

/// Abstraction over a date library.
pub trait DateAdapter {
    /// Concrete date type used by the adapter.
    type Date: Clone + PartialEq + core::fmt::Debug;

    /// Returns today's date according to the adapter.
    fn today(&self) -> Self::Date;

    /// Adds the specified number of days to `date`.
    fn add_days(&self, date: &Self::Date, days: i32) -> Self::Date;

    /// Formats the date into a user visible string using the adapter's
    /// default locale.
    fn format(&self, date: &Self::Date) -> String;
}

/// Adapter powered by the `chrono` crate.
#[cfg(feature = "chrono")]
pub struct AdapterChrono;

#[cfg(feature = "chrono")]
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

/// Adapter powered by the `time` crate.
#[cfg(feature = "time")]
pub struct AdapterTime;

#[cfg(feature = "time")]
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

