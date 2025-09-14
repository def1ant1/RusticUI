//! Adapters bridging external date/time libraries.
//!
//! **Unstable:** The adapter API is pre-production and may change without
//! warning.  Downstream applications should track the changelog closely.
//!
//! The goal is to decouple widgets from specific date/time crates so that
//! large organizations can standardize on a single time library while still
//! sharing UI components.  Each adapter is feature gated and lives in its own
//! module to keep compile times lean and optional dependencies isolated.

/// Abstraction over a date library.  Widgets use this trait for all
/// calendar math and formatting so that end users can plug in their
/// preferred implementation.
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

/// Abstraction over a time library.  The interface mirrors [`DateAdapter`]
/// but operates on clock times.  A separate trait keeps responsibilities
/// focused and allows projects to only implement what they need.
pub trait TimeAdapter {
    /// Concrete time type used by the adapter.
    type Time: Clone + PartialEq + core::fmt::Debug;

    /// Returns the current time according to the adapter.
    fn now(&self) -> Self::Time;

    /// Adds the specified number of minutes to `time`.
    fn add_minutes(&self, time: &Self::Time, minutes: i32) -> Self::Time;

    /// Formats the time into a user visible string using the adapter's
    /// default locale.
    fn format(&self, time: &Self::Time) -> String;
}

// Re-export feature gated adapters so consumers can simply import from the
// `adapters` module without knowing which backend is enabled.
#[cfg(feature = "chrono")]
pub mod chrono;
#[cfg(feature = "chrono")]
pub use chrono::AdapterChrono;

#[cfg(feature = "time")]
pub mod time;
#[cfg(feature = "time")]
pub use self::time::AdapterTime;
