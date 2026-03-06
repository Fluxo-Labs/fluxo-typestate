# Advanced Usage with Fluxo Typestate

This tutorial covers advanced patterns and techniques for Fluxo Typestate.

## Prerequisites

Complete the [Getting Started](getting_started.md) tutorial first.

---

## 1. Complex State Machines

### Multiple Transitions from One State

A state can have multiple transitions to different states:

```rust
use fluxo_typestate::state_machine;

#[state_machine]
enum Order {
    #[transition(Order::Created -> Order::Processing: process)]
    #[transition(Order::Created -> Order::Cancelled: cancel)]
    Created { order_id: u64, items: Vec<String> },
    
    #[transition(Order::Processing -> Order::Shipped: ship)]
    #[transition(Order::Processing -> Order::Cancelled: cancel)]
    Processing { order_id: u64 },
    
    #[transition(Order::Shipped -> Order::Delivered: deliver)]
    Shipped { order_id: u64, tracking: String },
    
    #[transition(Order::Delivered -> Order::Closed: close)]
    Delivered { order_id: u64 },
    
    Cancelled { order_id: u64, reason: String },
    Closed { order_id: u64 },
}

fn process_order(order: Order<Created>) {
    // Process the order
    let processing: Order<Processing> = order.process();
    
    // Ship it
    let shipped: Order<Shipped> = processing.ship();
    
    // Deliver it
    let delivered: Order<Delivered> = shipped.deliver();
    
    // Close it
    let closed: Order<Closed> = delivered.close();
}
```

### Wildcard Transitions

You can allow transitions from any state using `Any`:

```rust
#[state_machine]
enum Connection {
    #[transition(Any -> Disconnected: disconnect)]
    Disconnected,
    
    #[transition(Disconnected -> Connecting: connect)]
    Connecting,
    
    #[transition(Connecting -> Connected: establish)]
    Connected { latency_ms: u32 },
}

fn emergency_stop<S>(conn: Connection<S>) -> Connection<Disconnected> {
    // Works from any state!
    conn.disconnect()
}
```

---

## 2. State-Specific Methods

You can add custom methods to specific states:

```rust
use fluxo_typestate::state_machine;

#[state_machine]
enum Game {
    #[transition(Game::Menu -> Game::Playing: start)]
    Menu,
    
    #[transition(Game::Playing -> Game::Paused: pause)]
    #[transition(Game::Playing -> Game::Menu: quit)]
    Playing { score: u32, level: u32 },
    
    #[transition(Game::Paused -> Game::Playing: resume)]
    Paused { score: u32, level: u32 },
}

impl Game<Playing> {
    pub fn add_score(&mut self, points: u32) {
        self._inner_playing.score += points;
    }
    
    pub fn next_level(&mut self) {
        self._inner_playing.level += 1;
    }
}

fn play_game(mut game: Game<Playing>) {
    game.add_score(100);
    game.next_level();
    
    // Pause the game
    let paused: Game<Paused> = game.pause();
    
    // Resume
    let playing: Game<Playing> = paused.resume();
}
```

---

## 3. Guarded Transitions

For conditional transitions, use regular Rust control flow:

```rust
#[state_machine]
enum AuthService {
    #[transition(AuthService::Unauthenticated -> AuthService::Authenticated: login)]
    Unauthenticated { attempts: u32 },
    
    Authenticated { token: String, expires_at: u64 },
}

impl AuthService<Unauthenticated> {
    pub fn try_login(self, credentials: &Credentials) -> Result<AuthService<Authenticated>, AuthService<Unauthenticated>> {
        if credentials.is_valid() {
            Ok(self.login())
        } else {
            let attempts = self._inner_unauthenticated.attempts + 1;
            if attempts >= 3 {
                panic!("Too many login attempts");
            }
            Err(AuthService {
                _state: std::marker::PhantomData,
                _inner_unauthenticated: Unauthenticated { attempts },
            })
        }
    }
}
```

---

## 4. Combining States

Use tuple structs to combine multiple state aspects:

```rust
use fluxo_typestate::state_machine;

#[state_machine]
enum Document {
    #[transition(Draft -> Review: submit)]
    Draft { content: String, author: String },
    
    #[transition(Review -> Approved: approve)]
    #[transition(Review -> Draft: reject)]
    Review { content: String, reviewer: String },
    
    #[transition(Approved -> Published: publish)]
    Approved { content: String },
    
    Published { content: String, views: u32 },
}
```

---

## 5. Error Handling States

Include error states in your machine:

```rust
use fluxo_typestate::state_machine;

#[state_machine]
enum NetworkRequest {
    #[transition(Idle -> Loading: fetch)]
    Idle,
    
    #[transition(Loading -> Success: ok)]
    #[transition(Loading -> Error: fail)]
    Loading { url: String },
    
    Success { data: Vec<u8>, status: u16 },
    
    #[transition(Error -> Idle: retry)]
    Error { message: String, status_code: u16 },
}

fn handle_request(req: NetworkRequest<Loading>) {
    match req.fail_reason() {
        Ok(success) => println!("Got {} bytes", success._inner_success.data.len()),
        Err(error) => {
            println!("Error: {}", error._inner_error.message);
            // Can retry!
            let idle: NetworkRequest<Idle> = error.retry();
        }
    }
}
```

---

## 6. Integration with Async

Combine with async/await for async state machines:

```rust
use fluxo_typestate::state_machine;

#[state_machine]
enum AsyncConnection {
    #[transition(Disconnected -> Connecting: connect)]
    Disconnected,
    
    #[transition(Connecting -> Connected: complete)]
    Connecting { address: String },
    
    #[transition(Connected -> Disconnected: close)]
    Connected { stream: TcpStream },
}

async fn connect(addr: &str) -> AsyncConnection<Connecting> {
    AsyncConnection {
        _state: std::marker::PhantomData,
        _inner_connecting: Connecting { address: addr.to_string() },
    }
}

async fn example() {
    let connecting = connect("127.0.0.1:8080").await;
    let connected = connecting.complete().await;
    // Use connected stream...
}
```

---

## 7. Serialization

Enable serde support for serialization:

```toml
# Cargo.toml
[dependencies]
fluxo-typestate = { version = "0.1", features = ["serde"] }
```

```rust
use fluxo_typestate::state_machine;
use serde::{Serialize, Deserialize};

#[state_machine]
#[serde(tag = "type")]
enum GameState {
    #[transition(MainMenu -> Playing: start_game)]
    MainMenu,
    
    #[transition(Playing -> Paused: pause)]
    #[transition(Playing -> MainMenu: quit)]
    Playing { score: u32 },
    
    Paused { score: u32 },
}

fn serialize_game(state: &GameState<Playing>) -> Result<String, serde_json::Error> {
    serde_json::to_string(state)
}
```

---

## 8. Testing

Test state machines easily:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_transition() {
        let state: Computer<Idle> = Computer::new();
        let running: Computer<Running> = state.start();
        
        assert_eq!(running.current_state(), "Running");
    }
    
    #[test]
    fn test_invalid_transition() {
        let state: Computer<Idle> = Computer::new();
        
        // This won't compile - great for tests!
        // let sleeping = state.sleep();
    }
    
    #[test]
    fn test_full_cycle() {
        let idle: Computer<Idle> = Computer::new();
        let running = idle.start();
        let sleeping = running.suspend();
        let idle = sleeping.wake();
        
        assert_eq!(idle.current_state(), "Idle");
    }
}
```

---

## 9. Documentation Generation

Use the visualizer for documentation:

```rust
/// State machine for the document workflow
///
/// ## States
/// - `Draft`: Initial document creation
/// - `Review`: Under review
/// - `Approved`: Approved for publishing
/// - `Published`: Live document
///
/// ## Transitions
/// See [`Document::mermaid_diagram()`]
#[state_machine]
enum Document {
    // ...
}
```

---

## 10. Best Practices Summary

1. **Always define transitions explicitly** - Let the compiler catch errors
2. **Use descriptive state names** - Make the machine readable
3. **Keep transitions simple** - Complex logic in methods, not transitions
4. **Use the visualizer** - Verify your machine visually
5. **Test invalid transitions** - Ensure they don't compile

---

**License**: Copyright (c) 2024 Fluxo Labs  
**Author**: AI-generated code based on idea by alisio85
