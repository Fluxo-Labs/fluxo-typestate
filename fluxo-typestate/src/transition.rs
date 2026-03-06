// Copyright (c) 2026 Fluxo Labs
// AI-generated code based on idea by alisio85
// SPDX-License-Identifier: MIT

//! Transition types and traits for Fluxo Typestate.
//!
//! This module provides traits that represent complete state machines, including
//! the `StateMachine` trait for the main wrapper type and `StateMachineExt` for
//! extension methods common to all state machines.
//!
//! # StateMachine Trait
//!
//! The `StateMachine` trait represents a generic state machine wrapper type.
//! It's implemented automatically by the `#[state_machine]` macro for the
//! generated generic struct `Name<S>` where `S` is a state type.
//!
//! This trait provides:
//! - `new()`: Create a new instance in the initial state
//! - `current_state()`: Get the current state's name
//! - `can_transition_to<T>()`: Check if a transition to a specific state is possible
//!
//! # StateMachineExt Trait
//!
//! An extension trait that provides additional convenience methods for any
//! type implementing `StateMachine`. This includes the `current_state()` method
//! for easy state introspection.
//!
//! # Generic State Machine
//!
//! The main power of the type-state pattern comes from generic type parameters.
//! A state machine like `Computer<Idle>` is a completely different type from
//! `Computer<Running>`. This means:
//!
//! - You can't call `running.stop()` on a `Computer<Idle>` - it won't compile!
//! - The compiler enforces valid transitions at compile-time
//! - No runtime checks needed - zero overhead
//!
//! # Example
//!
//! ```ignore
//! use fluxo_typestate::{StateMachine, state_machine};
//!
//! #[state_machine]
//! enum Computer {
//!     #[transition(Computer::Idle -> Computer::Running: start)]
//!     Idle,
//!     #[transition(Computer::Running -> Computer::Idle: stop)]
//!     Running { cpu_load: f32 },
//! }
//!
//! fn main() {
//!     // Create new state machine - this is Computer<Idle>
//!     let computer: Computer<Idle> = Computer::new();
//!     
//!     // This works - transition from Idle to Running
//!     let running: Computer<Running> = computer.start();
//!     
//!     // This works - transition from Running to Idle
//!     let idle: Computer<Idle> = running.stop();
//!     
//!     // This WON'T compile! Can't call start() on Computer<Idle> again
//!     // because Idle doesn't have a start() method (only Running does)
//! }
//! ```

// Copyright (c) 2026 Fluxo Labs
// AI-generated code based on idea by alisio85
// SPDX-License-Identifier: MIT

use crate::State;

/// Extension trait providing additional functionality for state machine types.
///
/// This trait is automatically implemented for any type that implements `StateMachine`.
/// It provides convenient methods that are useful across different state machines.
///
/// # Usage
///
/// This trait is primarily used internally to provide the `current_state()` method.
/// It may be extended in future versions with additional convenience methods.
///
/// # Example
///
/// ```ignore
/// use fluxo_typestate::StateMachineExt;
///
/// fn print_state<S: State>(machine: &impl StateMachineExt) {
///     println!("Current state: {}", machine.current_state());
/// }
/// ```
pub trait StateMachineExt {
    /// Returns the name of the current state.
    ///
    /// This method provides a way to get a human-readable representation of
    /// the current state at runtime. While the type system guarantees valid
    /// transitions, this method allows for debugging and logging.
    ///
    /// # Returns
    ///
    /// A static string slice containing the name of the current state type.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let computer: Computer<Idle> = Computer::new();
    /// assert_eq!(computer.current_state(), "Idle");
    /// ```
    fn current_state(&self) -> &'static str;
}

/// A trait representing a complete state machine with compile-time state guarantees.
///
/// This trait is automatically implemented by the `#[state_machine]` macro for
/// the generated generic state machine wrapper type. It provides the core
/// functionality that all state machines must support.
///
/// # Type Parameters
///
/// - `S`: The current state type, must implement `State`
///
/// # Associated Types
///
/// - `InitialState`: The state type representing the initial/starting state
///
/// # Example
///
/// When you define:
///
/// ```ignore
/// #[state_machine]
/// enum Computer {
///     Idle,
///     Running { cpu_load: f32 },
/// }
/// ```
///
/// The macro generates a `Computer<S>` struct that implements `StateMachine`.
/// The `InitialState` associated type would be `Idle` (the first variant).
///
/// # Implementing StateMachine
///
/// **Do not implement this trait manually.** The `#[state_machine]` macro
/// generates the implementation automatically.
pub trait StateMachine: Sized {
    /// The initial state type for this state machine.
    ///
    /// This is typically the first variant of the enum, representing the
    /// state the machine starts in when created with `new()`.
    type InitialState: State;

    /// Creates a new state machine in its initial state.
    ///
    /// This is the standard way to create a new state machine instance.
    /// The machine starts in the initial state defined by `InitialState`.
    ///
    /// # Returns
    ///
    /// A new state machine instance in the initial state.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let computer: Computer<Idle> = Computer::new();
    /// ```
    fn new() -> Self;

    /// Returns the name of the current state.
    ///
    /// Provides runtime access to the state name for debugging and logging.
    ///
    /// # Returns
    ///
    /// A static string slice containing the current state's name.
    fn current_state(&self) -> &'static str;

    /// Checks if a transition to the given state type is possible.
    ///
    /// This method provides runtime introspection to check if a transition
    /// to a specific state type is valid. Note that this checks if the
    /// *type* exists as a valid target, not if the transition is currently
    /// allowed from the current state.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The target state type to check
    ///
    /// # Returns
    ///
    /// `true` if a transition to `T` is defined in the state machine,
    /// `false` otherwise.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let computer: Computer<Idle> = Computer::new();
    /// // Check if we can transition to Running
    /// assert!(computer.can_transition_to::<Running>());
    /// ```
    fn can_transition_to<T: State>(&self) -> bool;
}
