# Fluxo Typestate

[![CI](https://github.com/fluxo-labs/fluxo-typestate/workflows/CI/badge.svg)](https://github.com/fluxo-labs/fluxo-typestate/actions)
[![crates.io](https://img.shields.io/crates/v/fluxo-typestate.svg)](https://crates.io/crates/fluxo-typestate)
[![docs.rs](https://docs.rs/fluxo-typestate/badge.svg)](https://docs.rs/fluxo-typestate)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust 2024](https://img.shields.io/badge/Rust-2024-orange.svg)](https://blog.rust-lang.org/2024/11/19/Rust-2024.html)

Fluxo Typestate is a Rust library that provides **zero-cost type-state pattern** implementations via procedural macros. It automatically generates type-safe state machines from simple enum definitions, ensuring compile-time guarantees of valid state transitions.

## Philosophy

> *Complex design patterns shouldn't be a burden, but a compile-time guarantee.*

Fluxo Typestate bridges the gap between abstract architecture and safe, efficient code. We believe that complex design patterns shouldn't be a burden, but a compile-time guarantee.

## Features

- **Zero-Cost Abstractions**: All state checking happens at compile-time. No runtime overhead.
- **Compile-Time Graph Validation**: Invalid transitions produce clear compile errors
- **Automatic Event Logging**: Optional `tracing` integration for debugging
- **Visualization**: Generate Mermaid diagrams of your state machines

## Quick Start

```rust
use fluxo_typestate::state_machine;

#[state_machine]
enum Computer {
    #[transition(Idle -> Running: start)]
    #[transition(Idle -> Sleeping: sleep)]
    Idle,
    #[transition(Running -> Idle: stop)]
    #[transition(Running -> Sleeping: suspend)]
    Running { cpu_load: f32 },
    #[transition(Sleeping -> Idle: wake)]
    Sleeping,
}

fn main() {
    let computer: Computer<Idle> = Computer::new();
    
    // Transition to Running state
    let running: Computer<Running> = computer.start();
    println!("CPU Load: {}", running._inner_running.cpu_load);
    
    // Transition to Sleeping state
    let sleeping: Computer<Sleeping> = running.suspend();
    
    // Transition back to Idle
    let idle: Computer<Idle> = sleeping.wake();
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fluxo-typestate = "0.1"
```

For logging support, enable the `logging` feature:

```toml
[dependencies]
fluxo-typestate = { version = "0.1", features = ["logging"] }
```

## Documentation

- [Manual](fluxo-typestate/docs/manual.md) - Comprehensive guide
- [Getting Started](fluxo-typestate/docs/tutorials/getting_started.md) - Tutorial for beginners
- [API Reference](https://docs.rs/fluxo-typestate) - API documentation

## Examples

Check out the `examples/` directory for more usage examples:

- `basic/computer.rs` - Basic state machine
- `advanced/order_processing.rs` - Complex state transitions
- `real_world/http_connection.rs` - Real-world usage

## License

Copyright (c) 2026 Fluxo Labs  
AI-generated code based on idea by alisio85  
SPDX-License-Identifier: MIT

## Contributing

Contributions are welcome! Please read our [contributing guidelines](https://github.com/Fluxo-Labs/fluxo-typestate/blob/main/CONTRIBUTING.md) before submitting PRs.
