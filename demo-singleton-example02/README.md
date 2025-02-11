# Singleton Pattern Demo with LazyLock and Dynamic Trait Objects

This example demonstrates a simplified implementation of the `LazyLock<Mutex<Box<dyn T>>>` pattern, which is commonly used for global state management with trait objects in Rust.

## Key Pattern

```rust
static CACHE_STORE: LazyLock<Mutex<Box<dyn CacheStore>>> =
    LazyLock::new(|| Mutex::new(Box::new(InMemoryCache::new())));
```

This pattern combines several Rust features:
- `LazyLock`: For lazy initialization
- `Mutex`: For thread-safe access
- `Box<dyn T>`: For trait objects (dynamic dispatch)

## Features

- 🔒 Thread-safe global state
- 📦 Trait object implementation
- 🚀 Simple cache operations (get/set/clear)
- 🛠 Easy to extend with new cache implementations

## Running the Example

```bash
cargo run
```

The example will demonstrate:
1. Setting cache values
2. Retrieving cache values
3. Updating existing values
4. Clearing the cache
5. Handling non-existent keys

## Implementation Details

1. **Trait Definition**
   ```rust
   trait CacheStore: Send + Sync {
       fn get(&self, key: &str) -> Option<String>;
       fn set(&mut self, key: String, value: String);
       fn clear(&mut self);
   }
   ```

2. **In-Memory Implementation**
   ```rust
   struct InMemoryCache {
       data: Vec<(String, String, SystemTime)>
   }
   ```

3. **Global Singleton**
   - Uses `LazyLock` for lazy initialization
   - Wraps implementation in `Box<dyn CacheStore>`
   - Provides thread-safe access via `Mutex`

## Why This Pattern?

This pattern is useful when you need:
1. Global state with trait objects
2. Thread-safe access
3. Lazy initialization
4. Flexibility to swap implementations
