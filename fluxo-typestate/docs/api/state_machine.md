# API Reference

This document provides detailed API documentation for Fluxo Typestate.

---

## Crate: fluxo-typestate

### Macros

#### `#[state_machine]`

The main procedural macro that generates type-state implementations.

**Usage:**

```rust
use fluxo_typestate::state_machine;

#[state_machine]
enum MyState {
    #[transition(A -> B: go)]
    A,
    B,
}
```

**Attributes:**

- `#[transition(Source -> Target: method)]` - Define a state transition
- `#[trace]` - Enable transition logging
- `#[visualize]` - Enable Mermaid diagram generation

---

### Traits

#### `State`

```rust
pub trait State: Sealed + 'static {
    fn name(&self) -> &'static str;
}
```

The core trait implemented by all state types.

**Methods:**

- `fn name(&self) -> &'static str` - Get the state's name

---

#### `Sealed`

```rust
pub trait Sealed: /* ... */ {}
```

Marker trait preventing external implementations of `State`.

---

#### `StateMachine`

```rust
pub trait StateMachine {
    type InitialState: State;
    
    fn new() -> Self;
    fn current_state(&self) -> &'static str;
    fn can_transition_to<T: State>(&self) -> bool;
}
```

Trait for state machine types.

**Associated Types:**

- `InitialState: State` - The starting state type

**Methods:**

- `fn new() -> Self` - Create a new state machine in initial state
- `fn current_state(&self) -> &'static str` - Get current state name
- `fn can_transition_to<T: State>(&self) -> bool` - Check if transition is valid

---

#### `StateMachineExt`

```rust
pub trait StateMachineExt {
    fn current_state(&self) -> &'static str;
}
```

Extension trait with additional methods.

---

### Types

#### `FluxoError`

```rust
pub enum FluxoError {
    InvalidTransition { from: String, to: String, reason: String },
    InvalidState(String),
    FeatureNotEnabled(String),
    Serialization(String),
}
```

Error type for Fluxo operations.

---

## Crate: fluxo-typestate-macros

This crate is not intended for direct use. Use `fluxo-typestate` instead.

---

## Feature Flags

### `logging`

Enables automatic state transition logging via `tracing`.

```toml
[dependencies]
fluxo-typestate = { version = "0.1", features = ["logging"] }
```

### `serde`

Enables serialization support.

```toml
[dependencies]
fluxo-typestate = { version = "0.1", features = ["serde"] }
```

---

**License**: Copyright (c) 2024 Fluxo Labs  
**Author**: AI-generated code based on idea by alisio85
