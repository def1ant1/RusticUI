//! Time management primitives shared by advanced state machines.
//!
//! Enterprise applications frequently require deterministic time control so
//! that complex state charts can be validated in isolation.  Instead of
//! sprinkling ad-hoc `Instant::now()` calls throughout each component we expose
//! a tiny abstraction layer which can be backed by real wall clock time in
//! production and mocked clocks inside integration tests or automation suites.
//! This keeps business logic completely deterministic and dramatically reduces
//! the amount of manual QA required before releasing builds.

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::time::{Duration, Instant};

/// Minimal trait capturing the behaviour required by the timer helpers.
///
/// The trait keeps the API intentionally small so we can use it across
/// state machines without complicating their generics.  Implementations should
/// guarantee monotonically increasing instants which makes the schedulers
/// robust even when reused by long running services.
pub trait Clock: Clone {
    /// Concrete instant representation used by the clock implementation.
    type Instant: Copy + PartialOrd + fmt::Debug;

    /// Returns the current moment according to the clock.
    fn now(&self) -> Self::Instant;

    /// Adds the provided duration to an instant.
    fn add(&self, instant: Self::Instant, duration: Duration) -> Self::Instant;

    /// Returns the duration between two instants.
    fn duration_between(&self, earlier: Self::Instant, later: Self::Instant) -> Duration;
}

/// Real wall clock backed [`Clock`] implementation.
#[derive(Debug, Default, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    type Instant = Instant;

    #[inline]
    fn now(&self) -> Self::Instant {
        Instant::now()
    }

    #[inline]
    fn add(&self, instant: Self::Instant, duration: Duration) -> Self::Instant {
        instant
            .checked_add(duration)
            .expect("system clock overflowed while scheduling timer")
    }

    #[inline]
    fn duration_between(&self, earlier: Self::Instant, later: Self::Instant) -> Duration {
        later.saturating_duration_since(earlier)
    }
}

/// Deterministic clock used by unit tests and automation fixtures.
///
/// The clock shares its offset across clones which allows state machines and
/// tests to tick time forward without passing around mutable references.
#[derive(Debug, Clone)]
pub struct MockClock {
    base: Instant,
    offset: Rc<RefCell<Duration>>,
}

impl MockClock {
    /// Construct a new mock clock.
    pub fn new() -> Self {
        Self {
            base: Instant::now(),
            offset: Rc::new(RefCell::new(Duration::ZERO)),
        }
    }

    /// Advance the clock by the requested delta.
    pub fn advance(&self, delta: Duration) {
        *self.offset.borrow_mut() += delta;
    }
}

impl Default for MockClock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock for MockClock {
    type Instant = Instant;

    #[inline]
    fn now(&self) -> Self::Instant {
        self.base
            .checked_add(*self.offset.borrow())
            .expect("mock clock overflow while advancing time")
    }

    #[inline]
    fn add(&self, instant: Self::Instant, duration: Duration) -> Self::Instant {
        instant
            .checked_add(duration)
            .expect("mock clock overflow while scheduling timer")
    }

    #[inline]
    fn duration_between(&self, earlier: Self::Instant, later: Self::Instant) -> Duration {
        later.saturating_duration_since(earlier)
    }
}

/// Small helper struct encapsulating deadline handling.
#[derive(Clone, Default)]
pub struct Timer<C: Clock> {
    deadline: Option<C::Instant>,
    _marker: std::marker::PhantomData<C>,
}

impl<C> fmt::Debug for Timer<C>
where
    C: Clock,
    C::Instant: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Timer")
            .field("deadline", &self.deadline)
            .finish()
    }
}

impl<C: Clock> Timer<C> {
    /// Create an empty timer.
    #[inline]
    pub fn new() -> Self {
        Self {
            deadline: None,
            _marker: std::marker::PhantomData,
        }
    }

    /// Returns whether a deadline is currently scheduled.
    #[inline]
    pub fn is_scheduled(&self) -> bool {
        self.deadline.is_some()
    }

    /// Schedule the timer to fire after the supplied delay.
    #[inline]
    pub fn schedule(&mut self, clock: &C, delay: Duration) {
        self.deadline = Some(clock.add(clock.now(), delay));
    }

    /// Cancel the timer if a deadline existed.
    #[inline]
    pub fn cancel(&mut self) {
        self.deadline = None;
    }

    /// Returns whether the timer should fire at the current instant.
    #[inline]
    pub fn should_fire(&self, clock: &C) -> bool {
        matches!(self.deadline, Some(deadline) if clock.now() >= deadline)
    }

    /// Consume the deadline if it is due.
    #[inline]
    pub fn fire_if_due(&mut self, clock: &C) -> bool {
        if self.should_fire(clock) {
            self.deadline = None;
            true
        } else {
            false
        }
    }

    /// Returns the remaining time until the deadline if one exists.
    #[inline]
    pub fn remaining(&self, clock: &C) -> Option<Duration> {
        self.deadline
            .map(|deadline| clock.duration_between(clock.now(), deadline))
    }
}
