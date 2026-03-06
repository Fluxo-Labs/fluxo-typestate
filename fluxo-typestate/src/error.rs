// Copyright (c) 2026 Fluxo Labs
// AI-generated code based on idea by alisio85
// SPDX-License-Identifier: MIT

//! Error types for Fluxo Typestate.
//!
//! This module provides the `FluxoError` enum that represents various error
//! conditions that can occur when working with Fluxo Typestate.
//!
//! While the type-state pattern provides compile-time guarantees for valid
//! transitions, certain operations may still fail at runtime. This module
//! defines the errors that can occur in those cases.
//!
//! # Error Types
//!
//! - `InvalidTransition`: An invalid state transition was attempted
//! - `InvalidState`: The state machine is in an invalid state for an operation
//! - `FeatureNotEnabled`: A required feature flag is not enabled
//! - `Serialization`: Serialization or deserialization failed
//!
//! # Usage
//!
//! ```text
//! use fluxo_typestate::FluxoError;
//!
//! fn handle_error(err: FluxoError) {
//!     match err {
//!         FluxoError::InvalidTransition { from, to, reason } => {
//!             println!("Invalid transition from {} to {}: {}", from, to, reason);
//!         }
//!         // ... handle other cases
//!     }
//! }
//! ```

// Copyright (c) 2026 Fluxo Labs
// AI-generated code based on idea by alisio85
// SPDX-License-Identifier: MIT
use std::fmt;

/// The main error type for Fluxo Typestate operations.
///
/// This enum represents various error conditions that can occur when working
/// with Fluxo Typestate. While the type-state pattern provides compile-time
/// guarantees for valid transitions, certain operations may still fail at
/// runtime or require error handling.
///
/// # Variants
///
/// - `InvalidTransition`: Represents an invalid state transition attempt
/// - `InvalidState`: The state machine is in an invalid state
/// - `FeatureNotEnabled`: A required feature flag is not enabled
/// - `Serialization`: Serialization/deserialization error
///
/// # Example
///
/// ```ignore
/// use fluxo_typestate::FluxoError;
///
/// let err = FluxoError::invalid_transition("Idle", "Sleeping", "Cannot sleep while processing");
/// println!("Error: {}", err);
/// ```
#[derive(Debug, Clone)]
pub enum FluxoError {
    /// An invalid state transition was attempted.
    ///
    /// This error occurs when code attempts to transition between states
    /// in a way that's not allowed by the state machine definition.
    ///
    /// Fields:
    /// - `from`: The source state name
    /// - `to`: The target state name
    /// - `reason`: Explanation of why the transition is invalid
    InvalidTransition {
        /// The source state from which the transition was attempted.
        from: String,
        /// The target state to which the transition was attempted.
        to: String,
        /// The reason why this transition is not allowed.
        reason: String,
    },
    /// The state machine is in an invalid state for the requested operation.
    ///
    /// This error occurs when an operation requires a specific state that
    /// the machine is not currently in.
    InvalidState(String),
    /// A required feature is not enabled.
    ///
    /// This error occurs when attempting to use functionality that requires
    /// a feature flag to be enabled in `Cargo.toml`.
    FeatureNotEnabled(String),
    /// A serialization or deserialization error occurred.
    ///
    /// This error occurs when serializing or deserializing a state machine
    /// fails. Requires the `serde` feature to be enabled.
    Serialization(String),
}

impl fmt::Display for FluxoError {
    /// Formats the error for display purposes.
    ///
    /// This implementation provides human-readable error messages for each
    /// variant of the enum.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FluxoError::InvalidTransition { from, to, reason } => {
                write!(
                    f,
                    "Invalid state transition from '{}' to '{}': {}",
                    from, to, reason
                )
            }
            FluxoError::InvalidState(msg) => {
                write!(f, "Invalid state: {}", msg)
            }
            FluxoError::FeatureNotEnabled(feature) => {
                write!(
                    f,
                    "Feature '{}' is not enabled. Enable it in your Cargo.toml",
                    feature
                )
            }
            FluxoError::Serialization(msg) => {
                write!(f, "Serialization error: {}", msg)
            }
        }
    }
}

impl std::error::Error for FluxoError {}

impl FluxoError {
    /// Creates a new `InvalidTransition` error.
    ///
    /// This convenience constructor creates an error representing an invalid
    /// state transition attempt.
    ///
    /// # Arguments
    ///
    /// * `from` - The source state name
    /// * `to` - The target state name  
    /// * `reason` - Explanation of why the transition is invalid
    ///
    /// # Returns
    ///
    /// A new `FluxoError::InvalidTransition` instance
    ///
    /// # Example
    ///
    /// ```ignore
    /// let err = FluxoError::invalid_transition("Idle", "Sleeping", "operation not allowed");
    /// ```
    pub fn invalid_transition(
        from: impl Into<String>,
        to: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        FluxoError::InvalidTransition {
            from: from.into(),
            to: to.into(),
            reason: reason.into(),
        }
    }

    /// Creates a new `InvalidState` error.
    ///
    /// This convenience constructor creates an error representing an invalid
    /// state condition.
    ///
    /// # Arguments
    ///
    /// * `msg` - Description of the invalid state
    ///
    /// # Returns
    ///
    /// A new `FluxoError::InvalidState` instance
    ///
    /// # Example
    ///
    /// ```ignore
    /// let err = FluxoError::invalid_state("expected running state");
    /// ```
    pub fn invalid_state(msg: impl Into<String>) -> Self {
        FluxoError::InvalidState(msg.into())
    }
}
