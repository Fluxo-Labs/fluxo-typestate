// Copyright (c) 2026 Fluxo Labs
// AI-generated code based on idea by alisio85
// SPDX-License-Identifier: MIT

//! # Fluxo Typestate
//!
//! Fluxo Typestate is a Rust library that provides zero-cost type-state pattern
//! implementations via procedural macros. It automatically generates type-safe state
//! machines from simple enum definitions, ensuring compile-time guarantees of valid
//! state transitions.
//!
//! ## Overview
//!
//! The type-state pattern is a powerful technique in Rust that uses the type system
//! to encode state information. By representing each state as a different type,
//! the compiler can enforce that invalid state transitions are caught at compile-time,
//! rather than causing runtime errors.
//!
//! Fluxo Typestate automates this pattern by allowing you to define your state machine
//! as a simple enum, and then automatically generating all the necessary structs,
//! traits, and transition methods.
//!
//! ## Features
//!
//! - **Zero-Cost Abstractions**: All state checking happens at compile-time. The
//!   generated code has no runtime overhead compared to hand-written implementations.
//!
//! - **Compile-Time Graph Validation**: Invalid transitions produce clear compile
//!   errors that show you exactly which transitions are valid from the current state.
//!
//! - **Automatic Event Logging**: When the `logging` feature is enabled, every state
//!   transition can automatically log its occurrence using the `tracing` crate.
//!
//! - **Mermaid Visualization**: Generate Mermaid.js state diagrams to visualize
//!   your state machine structure in documentation or IDEs.
//!
//! ## Quick Start
//!
//! Here's a simple example of how to use Fluxo Typestate:
//!
//! ```ignore
//! use fluxo_typestate::state_machine;
//!
//! #[state_machine]
//! enum Computer {
//!     Idle,
//!     Running { cpu_load: f32 },
//!     Sleeping,
//! }
//!
//! fn main() {
//!     let computer: Computer<Idle> = Computer::new();
//!     let running: Computer<Running> = computer.start();
//!     println!("CPU Load: {}", running._inner_running.cpu_load);
//! }
//! ```
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! fluxo-typestate = "0.1"
//! ```
//!
//! For logging support, enable the `logging` feature:
//!
//! ```toml
//! [dependencies]
//! fluxo-typestate = { version = "0.1", features = ["logging"] }
//! ```
//!
//! For serialization support, enable the `serde` feature:
//!
//! ```toml
//! [dependencies]
//! fluxo-typestate = { version = "0.1", features = ["serde"] }
//! ```
//!
//! ## Defining a State Machine
//!
//! To define a state machine, create an enum and annotate it with `#[state_machine]`.
//! Each variant of the enum represents a state. You can have:
//!
//! - **Unit variants**: `Idle` - states with no associated data
//! - **Tuple variants**: `Running(f32)` - states with unnamed data
//! - **Named variants**: `Running { cpu_load: f32 }` - states with named data
//!
//! ## Defining Transitions
//!
//! Transitions are defined using the `#[transition]` attribute on enum variants.
//! The syntax is:
//!
//! ```ignore
//! #[transition(SourceState -> TargetState: method_name)]
//! ```
//!
//! This generates a method called `method_name` that transitions from the source
//! state to the target state. The method consumes the current state and returns
//! the new state, making it impossible to accidentally use the old state after
//! a transition.
//!
//! ## The State Trait
//!
//! All generated state structs implement the `State` trait, which provides the
//! `name()` method to get a string representation of the state type.
//!
//! ## The Sealed Trait
//!
//! The `Sealed` trait prevents external crates from implementing the `State`
//! trait, which ensures the type safety guarantees of the pattern.
//!
//! ## Example: A Computer State Machine
//!
//! ```ignore
//! use fluxo_typestate::state_machine;
//!
//! #[state_machine]
//! enum Computer {
//!     #[transition(Computer::Idle -> Computer::Running: start)]
//!     #[transition(Computer::Idle -> Computer::Sleeping: sleep)]
//!     Idle,
//!     #[transition(Computer::Running -> Computer::Idle: stop)]
//!     #[transition(Computer::Running -> Computer::Sleeping: suspend)]
//!     Running { cpu_load: f32 },
//!     #[transition(Computer::Sleeping -> Computer::Idle: wake)]
//!     Sleeping,
//! }
//!
//! fn main() {
//!     // Create a new computer in the Idle state
//!     let computer: Computer<Idle> = Computer::new();
//!     
//!     // Start the computer (transition to Running)
//!     let running: Computer<Running> = computer.start();
//!     println!("CPU Load: {}", running._inner_running.cpu_load);
//!     
//!     // Suspend the computer (transition to Sleeping)
//!     let sleeping: Computer<Sleeping> = running.suspend();
//!     
//!     // Wake up the computer (transition back to Idle)
//!     let idle: Computer<Idle> = sleeping.wake();
//! }
//! ```
//!
//! ## Viewing the State Machine
//!
//! You can generate a Mermaid diagram of your state machine:
//!
//! ```ignore
//! println!("{}", Computer::<Idle>::mermaid_diagram());
//! ```
//!
//! ## Performance
//!
//! Fluxo Typestate is designed with zero-cost abstractions in mind:
//!
//! - All state checking happens at compile-time through Rust's type system
//! - The `PhantomData<S>` marker has zero size
//! - Transition methods are statically dispatched with no function pointer overhead
//!
//! ## Comparison with Alternatives
//!
//! | Feature | Fluxo | Manual Type-State | Other Crates |
//! |---------|-------|------------------|--------------|
//! | Auto-generated transitions | ✓ | ✗ | Partial |
//! | Compile-time validation | ✓ | ✓ | Partial |
//! | Mermaid visualization | ✓ | ✗ | ✗ |
//! | Zero-cost | ✓ | ✓ | ✓ |
//! | Logging integration | ✓ | ✗ | ✗ |
//!
//! ## License
//!
//! Copyright (c) 2026 Fluxo Labs  
//! AI-generated code based on idea by alisio85  
//! SPDX-License-Identifier: MIT
//!
//! For more information, see the [documentation](https://docs.rs/fluxo-typestate).

// Copyright (c) 2026 Fluxo Labs
// AI-generated code based on idea by alisio85
// SPDX-License-Identifier: MIT

#![doc(html_root_url = "https://docs.rs/fluxo-typestate/0.1")]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

//! The main entry point for the Fluxo Typestate library.
//!
//! This module re-exports the core types and traits needed to use Fluxo Typestate.
//! The main component is the `state_machine` procedural macro which transforms
//! enum definitions into complete type-state implementations.

pub mod error;
pub mod state;
pub mod transition;

pub use error::FluxoError;
pub use state::{Sealed, State};
pub use transition::{StateMachine, StateMachineExt};

#[cfg(feature = "logging")]
pub use tracing;

pub use fluxo_typestate_macros::state_machine;
