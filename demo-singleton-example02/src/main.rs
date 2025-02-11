use std::sync::{LazyLock, Mutex};
use std::time::SystemTime;
use tokio::signal::unix::{signal, SignalKind};
use tracing::info;

// Define a trait for our cache operations
trait CacheStore: Send + Sync {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: String, value: String);
    fn clear(&mut self);
    fn list(&self) -> Vec<(String, String, SystemTime)>;
}

// Simple in-memory implementation
struct InMemoryCache {
    data: Vec<(String, String, SystemTime)>,
}

impl InMemoryCache {
    fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl CacheStore for InMemoryCache {
    fn get(&self, key: &str) -> Option<String> {
        self.data
            .iter()
            .find(|(k, _, _)| k == key)
            .map(|(_, v, _)| v.clone())
    }

    fn set(&mut self, key: String, value: String) {
        // Remove existing entry if it exists
        self.data.retain(|(k, _, _)| k != &key);
        // Add new entry
        self.data.push((key, value, SystemTime::now()));
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn list(&self) -> Vec<(String, String, SystemTime)> {
        self.data.clone()
    }
}

// Global singleton cache store
pub(crate) static CACHE_STORE: LazyLock<Mutex<Box<dyn CacheStore>>> =
    LazyLock::new(|| Mutex::new(Box::new(InMemoryCache::new())));

// Function to handle signals and modify the cache
fn spawn_signal_handler() -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut sigusr1 = signal(SignalKind::user_defined1())
            .expect("failed to create SIGUSR1 handler");
        let mut sigusr2 = signal(SignalKind::user_defined2())
            .expect("failed to create SIGUSR2 handler");

        info!("Signal handler started");
        loop {
            tokio::select! {
                // SIGUSR1 -> Add/update cache entry
                _ = sigusr1.recv() => {
                    info!("Received SIGUSR1");
                    let mut store = CACHE_STORE.lock().unwrap();
                    let key = format!("key{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());
                    let value = format!("value-{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos());
                    store.set(key.clone(), value.clone());
                    println!("\n=== SIGUSR1 Received ===");
                    println!("Added cache entry: {} = {}", key, value);
                    drop(store); // Release the lock before printing status
                    print_cache_status();
                }
                // SIGUSR2 -> Clear cache
                _ = sigusr2.recv() => {
                    info!("Received SIGUSR2");
                    let mut store = CACHE_STORE.lock().unwrap();
                    store.clear();
                    println!("\n=== SIGUSR2 Received ===");
                    println!("Cleared all cache entries");
                    drop(store); // Release the lock before printing status
                    print_cache_status();
                }
            }
        }
    })
}

// Helper function to print cache status
fn print_cache_status() {
    let store = CACHE_STORE.lock().unwrap();
    let entries = store.list();
    println!("=== Cache Status ===");
    if entries.is_empty() {
        println!("No entries in cache");
    } else {
        println!("Total entries: {}", entries.len());
        for (key, value, time) in entries {
            if let Ok(age) = SystemTime::now().duration_since(time) {
                println!("  {} = {} (age: {}ms)", key, value, age.as_millis());
            }
        }
    }
    println!();
}

#[tokio::main]
async fn main() {
    // Initialize tracing for better logging
    tracing_subscriber::fmt::init();

    // Print process information
    let pid = std::process::id();
    println!("\n=== LazyLock<Mutex<Box<dyn T>>> Cache Demo ===");
    println!("\nProcess ID (PID): {}", pid);
    println!("\nSignal Commands:");
    println!("  kill -SIGUSR1 {}  # Add new cache entry", pid);
    println!("  kill -SIGUSR2 {}  # Clear all entries", pid);
    println!("  kill -TERM {}     # Exit gracefully", pid);
    println!("  # or press Ctrl+C");
    
    // Print initial cache status
    print_cache_status();

    // Spawn signal handler
    let _signal_handler = spawn_signal_handler();
    info!("Signal handler spawned");

    // Wait for Ctrl+C
    tokio::signal::ctrl_c().await
        .expect("failed to listen for ctrl+c");

    // Print final cache status
    println!("\n=== Final Cache Status ===");
    print_cache_status();
    println!("Demo completed!");
}
