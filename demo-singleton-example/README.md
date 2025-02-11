# Singleton Pattern Demo with LazyLock and Signal Handling

This example demonstrates how to implement a thread-safe singleton pattern in Rust using `std::sync::LazyLock`. The example uses a global counter that can be modified through Unix signals, showcasing a practical application of the singleton pattern in an async context.

## Key Concepts Demonstrated

1. **Singleton Pattern Implementation**
   - Using `std::sync::LazyLock` for lazy initialization
   - Thread-safe state management with `Mutex`
   - Global state accessible throughout the application

2. **Async Programming**
   - Tokio runtime for async operations
   - Signal handling in an async context
   - Graceful shutdown handling

## Features

- 🔒 Thread-safe global counter using `LazyLock` and `Mutex`
- 📡 Unix signal handling for state modification:
  - `SIGUSR1`: Increment counter by 1
  - `SIGUSR2`: Increment counter by 10
- 📝 Tracing-based logging for operations
- 🛑 Graceful shutdown with final state reporting

## Running the Example

1. Build and start the application:
   ```bash
   cargo run
   ```

2. The application will display:
   - Process ID (PID)
   - Parent Process ID
   - Executable path
   - Usage instructions

3. In another terminal, send signals to modify the counter:
   ```bash
   # Increment counter by 1
   kill -SIGUSR1 <pid>

   # Increment counter by 10
   kill -SIGUSR2 <pid>
   ```

4. To exit:
   - Press Ctrl+C
   - Or send SIGTERM: `kill -TERM <pid>`

## Implementation Details

### 1. Singleton Implementation
```rust
// Global counter with initial value 0
static COUNTER: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
```

Key aspects:
- `LazyLock` ensures the counter is initialized only when first accessed
- `Mutex` provides thread-safe access to the counter
- Static lifetime makes it globally accessible

### 2. Signal Handling
```rust
fn spawn_signal_handler() -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut sigusr1 = signal(SignalKind::user_defined1())...
        let mut sigusr2 = signal(SignalKind::user_defined2())...
        // Handle signals asynchronously
    })
}
```

Key aspects:
- Async signal processing using Tokio
- `tokio::select!` for concurrent signal handling
- Clean separation of signal handling logic

## Why This Pattern?

The singleton pattern with `LazyLock` is useful when you need:
1. Global state that's initialized lazily
2. Thread-safe access to shared resources
3. Guaranteed single instance throughout the application
4. Clean and idiomatic Rust implementation

## Notes

- This example requires a Unix-like operating system for signal handling
- The pattern can be adapted for other types of global state beyond simple counters
- Consider using this pattern when you need guaranteed single instance and thread-safe access
