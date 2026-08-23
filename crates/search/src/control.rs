use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

/// `SearchCancellation` is a shared, one-way request to stop a search.
///
/// Clones refer to the same request, allowing a controller thread to cancel
/// work without sharing ownership of the search task itself.
///
/// @type
#[derive(Clone, Debug, Default)]
pub struct SearchCancellation {
    cancelled: Arc<AtomicBool>, // true after cancellation has been requested
}

impl SearchCancellation {
    /// `new` creates a cancellation request in its initial active state.
    ///
    /// @return: new cancellation request
    pub fn new() -> Self {
        Self::default()
    }

    /// `cancel` requests that searches observing this value stop.
    ///
    /// Cancellation is permanent for this value. Start a new search with a new
    /// value rather than attempting to reuse one that has been cancelled.
    ///
    /// @return: void
    /// @side-effects: marks this cancellation request and every clone cancelled
    pub fn cancel(&self) {
        // The flag carries no associated data. Search result publication uses
        // its own synchronization, so relaxed ordering is sufficient here.
        self.cancelled.store(true, Ordering::Relaxed);
    }

    /// `is_cancelled` reports whether cancellation has been requested.
    ///
    /// @return: true after this value or one of its clones has been cancelled
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
}

/// `SearchControl` composes the stopping conditions for one search task.
///
/// A control value may contain a deadline, explicit cancellation, or both.
/// Callers may temporarily remove every stopping condition to guarantee a
/// fallback iteration before observing cancellation or a deadline.
///
/// @type
#[derive(Clone, Debug)]
pub struct SearchControl {
    deadline: Option<Instant>,
    cancellation: Option<SearchCancellation>,
}

impl SearchControl {
    /// `new` creates deadline-only search control.
    ///
    /// @param: deadline - optional instant at which search should stop
    /// @return: new search control
    pub const fn new(deadline: Option<Instant>) -> Self {
        Self {
            deadline,
            cancellation: None,
        }
    }

    /// `with_cancellation` creates search control with explicit cancellation.
    ///
    /// @param: deadline - optional instant at which search should stop
    /// @param: cancellation - shared request used to cancel the search
    /// @return: new search control
    pub const fn with_cancellation(
        deadline: Option<Instant>,
        cancellation: SearchCancellation,
    ) -> Self {
        Self {
            deadline,
            cancellation: Some(cancellation),
        }
    }

    /// `should_stop` reports whether any configured stopping condition is met.
    ///
    /// @return: true when cancelled or when the deadline has been reached
    pub fn should_stop(&self) -> bool {
        self.cancellation
            .as_ref()
            .is_some_and(SearchCancellation::is_cancelled)
            || self
                .deadline
                .is_some_and(|deadline| Instant::now() >= deadline)
    }

    /// `with_deadline` replaces the deadline while retaining cancellation.
    ///
    /// This supports layers that independently contribute time allocation and
    /// explicit task cancellation without exposing either internal field.
    ///
    /// @param: deadline - replacement instant at which search should stop
    /// @return: cloned search control with the replacement deadline
    pub fn with_deadline(&self, deadline: Option<Instant>) -> Self {
        Self {
            deadline,
            cancellation: self.cancellation.clone(),
        }
    }

    /// `without_stopping_conditions` creates unrestricted search control.
    ///
    /// @return: search control without a deadline or cancellation request
    pub(crate) const fn without_stopping_conditions(&self) -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancellation_is_shared_across_clones() {
        let cancellation = SearchCancellation::new();
        let observer = cancellation.clone();

        cancellation.cancel();

        assert!(observer.is_cancelled());
    }

    #[test]
    fn removing_stopping_conditions_ignores_explicit_cancellation() {
        let cancellation = SearchCancellation::new();
        cancellation.cancel();
        let control = SearchControl::with_cancellation(Some(Instant::now()), cancellation);
        let unrestricted = control.without_stopping_conditions();

        assert!(control.should_stop());
        assert!(!unrestricted.should_stop());
    }

    #[test]
    fn replacing_deadline_preserves_explicit_cancellation() {
        let cancellation = SearchCancellation::new();
        let control = SearchControl::with_cancellation(None, cancellation.clone());
        let expired = control.with_deadline(Some(Instant::now()));

        assert!(expired.should_stop());
        assert!(!control.should_stop());

        cancellation.cancel();
        assert!(control.should_stop());
    }
}
