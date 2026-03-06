# Migration Guide

This guide helps you migrate from other state machine libraries or manual type-state implementations to Fluxo Typestate.

---

## Migration from Manual Type-State

### Before (Manual)

```rust
use std::marker::PhantomData;

// Define states as structs
struct Idle;
struct Running { cpu_load: f32 }
struct Sleeping;

// State trait
trait State: Sealed {}
trait Sealed {}

// Implement Sealed and State for each
impl Sealed for Idle {}
impl State for Idle {}

impl Sealed for Running {}
impl State for Running {}

impl Sealed for Sleeping {}
impl State for Sleeping {}

// Main wrapper
struct Computer<S: State> {
    _state: PhantomData<S>,
    cpu_load: f32,
}

// Implement transitions manually
impl Computer<Idle> {
    fn start(self) -> Computer<Running> {
        Computer {
            _state: PhantomData,
            cpu_load: 0.0,
        }
    }
    
    fn sleep(self) -> Computer<Sleeping> {
        Computer {
            _state: PhantomData,
            cpu_load: 0.0,
        }
    }
}

impl Computer<Running> {
    fn stop(self) -> Computer<Idle> {
        Computer {
            _state: PhantomData,
            cpu_load: 0.0,
        }
    }
}

impl Computer<Sleeping> {
    fn wake(self) -> Computer<Idle> {
        Computer {
            _state: PhantomData,
            cpu_load: 0.0,
        }
    }
}
```

### After (Fluxo)

```rust
use fluxo_typestate::state_machine;

#[state_machine]
enum Computer {
    #[transition(Idle -> Running: start)]
    #[transition(Idle -> Sleeping: sleep)]
    Idle,
    
    #[transition(Running -> Idle: stop)]
    Running { cpu_load: f32 },
    
    #[transition(Sleeping -> Idle: wake)]
    Sleeping,
}
```

That's it! The macro generates everything else.

---

## Migration from `states` Crate

### Before

```rust
use states::state;

// Define states
#[state]
enum Computer {
    Idle,
    Running { cpu_load: f32 },
    Sleeping,
}
```

### After

Replace with Fluxo's syntax:

```rust
use fluxo_typestate::state_machine;

#[state_machine]
enum Computer {
    #[transition(Computer::Idle -> Computer::Running: start)]
    Idle,
    
    Running { cpu_load: f32 },
    Sleeping,
}
```

Key differences:
- `#[state]` → `#[state_machine]`
- No automatic transitions → explicit `#[transition(...)]`
- Runtime validation → compile-time validation

---

## Migration from `rustステートマシーン` Crate

### Before

```rust
use state_machine::state_machine;

#[state_machine]
enum Computer {
    Idle -> Running [start],
    Running -> Idle [stop],
    Running -> Sleeping [suspend],
    Sleeping -> Idle [wake],
}
```

### After

```rust
use fluxo_typestate::state_machine;

#[state_machine]
enum Computer {
    #[transition(Computer::Idle -> Computer::Running: start)]
    Idle,
    #[transition(Computer::Running -> Computer::Idle: stop)]
    Running { cpu_load: f32 },
    #[transition(Computer::Running -> Computer::Sleeping: suspend)]
    Sleeping,
    #[transition(Computer::Sleeping -> Computer::Idle: wake)]
}
```

---

## Migration Patterns

### Pattern 1: Simple States

**Before:**
```rust
struct Idle;
impl State for Idle {}
```

**After:**
```rust
Idle, // Just add as a variant!
```

### Pattern 2: States with Data

**Before:**
```rust
struct Running { cpu_load: f32 }
impl State for Running {}
```

**After:**
```rust
Running { cpu_load: f32 }, // Add as a named variant
```

### Pattern 3: Transitions

**Before:**
```rust
impl Computer<Idle> {
    fn start(self) -> Running { /* ... */ }
}
```

**After:**
```rust
#[transition(Idle -> Running: start)] // Add attribute
Idle,
```

### Pattern 4: Complex Logic

**Before:**
```rust
impl Computer<Idle> {
    fn start(self) -> Result<Running, Error> {
        if can_start() {
            Ok(Running { cpu_load: 0.0 })
        } else {
            Err(Error::CannotStart)
        }
    }
}
```

**After:**

For complex logic, add a method to the generated type:

```rust
impl Computer<Idle> {
    pub fn try_start(self) -> Result<Computer<Running>, Computer<Idle>> {
        if can_start() {
            Ok(self.start())
        } else {
            Err(self)
        }
    }
}
```

---

## Common Migration Issues

### Issue 1: Missing Transitions

**Error:**
```
error[E0599]: no method named `start` found
```

**Solution:** Add `#[transition(...)]` attribute to your variant.

### Issue 2: State Not Found

**Error:**
```
error: Invalid target state 'SomeState'
```

**Solution:** Ensure the target state exists as a variant in your enum.

### Issue 3: Feature Flags

**Before using logging:**
```bash
cargo add fluxo-typestate --features logging
```

---

## Checklist

- [ ] Replace `#[state]` with `#[state_machine]`
- [ ] Convert state structs to enum variants
- [ ] Add `#[transition(...)]` attributes
- [ ] Remove manual `impl` blocks for transitions
- [ ] Enable `logging` feature if needed
- [ ] Run tests to verify

---

## Support

If you encounter issues during migration:
1. Check the [Manual](manual.md)
2. Look at [Examples](examples/)
3. Open an issue on GitHub

---

**License**: Copyright (c) 2024 Fluxo Labs  
**Author**: AI-generated code based on idea by alisio85
