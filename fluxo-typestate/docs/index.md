# Fluxo Typestate Documentation

Welcome to the Fluxo Typestate documentation!

## Quick Links

- [Manual](manual.md) - Comprehensive manual
- [Getting Started](tutorials/getting_started.md) - For beginners
- [Advanced Usage](tutorials/advanced_usage.md) - Advanced topics
- [Migration Guide](tutorials/migration_guide.md) - Migrating from other solutions
- [API Reference](api/state_machine.md) - API documentation

## What is Fluxo Typestate?

Fluxo Typestate is a Rust library that provides zero-cost type-state pattern implementations via procedural macros. It automatically generates type-safe state machines from simple enum definitions.

## Key Features

- **Zero-Cost Abstractions**: All state checking at compile-time
- **Compile-Time Validation**: Invalid transitions caught at compile-time
- **Automatic Logging**: Optional tracing integration
- **Visualization**: Generate Mermaid diagrams

## Installation

```toml
[dependencies]
fluxo-typestate = "0.1"
```

## Quick Example

```rust
use fluxo_typestate::state_machine;

#[state_machine]
enum Computer {
    #[transition(Idle -> Running: start)]
    Idle,
    #[transition(Running -> Idle: stop)]
    Running { cpu_load: f32 },
}

fn main() {
    let computer: Computer<Idle> = Computer::new();
    let running: Computer<Running> = computer.start();
    let idle: Computer<Idle> = running.stop();
}
```

## License

Copyright (c) 2024 Fluxo Labs  
AI-generated code based on idea by alisio85  
SPDX-License-Identifier: MIT
