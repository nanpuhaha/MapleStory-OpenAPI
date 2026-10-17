// src/models/batch.rs
//! Batch processing results and error representations.

use crate::error::MapleStoryError;

/// Individual failure record in a batch operation.
#[derive(Debug)]
pub struct BatchFailure {
    /// Identifier or character name associated with the failure.
    pub name: String,
    /// Detailed error returned for this item.
    pub error: MapleStoryError,
}

impl BatchFailure {
    /// Create a new batch failure record.
    pub fn new(name: impl Into<String>, error: MapleStoryError) -> Self {
        Self {
            name: name.into(),
            error,
        }
    }
}

/// Generic container representing the result of a batch parallel collection.
#[derive(Debug)]
pub struct BatchResult<T> {
    /// Items successfully fetched and processed.
    pub successful: Vec<T>,
    /// Items that failed during batch processing.
    pub failed: Vec<BatchFailure>,
}

impl<T> Default for BatchResult<T> {
    fn default() -> Self {
        Self {
            successful: Vec::new(),
            failed: Vec::new(),
        }
    }
}

impl<T> BatchResult<T> {
    /// Create a new batch result from successful and failed collections.
    pub fn new(successful: Vec<T>, failed: Vec<BatchFailure>) -> Self {
        Self { successful, failed }
    }

    /// Check if all items in the batch succeeded without errors.
    pub fn is_all_successful(&self) -> bool {
        self.failed.is_empty()
    }

    /// Calculate the success rate between 0.0 and 1.0.
    ///
    /// If the batch was empty, returns `1.0`.
    pub fn success_rate(&self) -> f64 {
        let total = self.total_count();
        if total == 0 {
            1.0
        } else {
            self.successful.len() as f64 / total as f64
        }
    }

    /// Total count of processed items (successful + failed).
    pub fn total_count(&self) -> usize {
        self.successful.len() + self.failed.len()
    }

    /// Check if any failures occurred in the batch.
    pub fn has_failures(&self) -> bool {
        !self.failed.is_empty()
    }
}
