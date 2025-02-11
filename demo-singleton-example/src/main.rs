use std::sync::LazyLock;
use std::sync::Mutex;
use tokio::signal::unix::{signal, SignalKind};

// Global counter with initial value 0
static COUNTER: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

// Function to increment counter by a specific amount
fn increment_by(amount: i32) {
    if let Ok(mut counter) = COUNTER.lock() {
        *counter += amount;
        println!("\nCounter incremented by {}", amount);
        println!("Current value: {}\n", *counter);
    }
}

// Function to handle signals and modify the counter
fn spawn_signal_handler() -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut sigusr1 = signal(SignalKind::user_defined1())
            .expect("failed to create SIGUSR1 handler");
        let mut sigusr2 = signal(SignalKind::user_defined2())
            .expect("failed to create SIGUSR2 handler");

        loop {
            tokio::select! {
                // SIGUSR1 -> Increment by 1
                _ = sigusr1.recv() => {
                    tracing::info!("Received SIGUSR1, incrementing by 1");
                    increment_by(1);
                }
                // SIGUSR2 -> Increment by 10
                _ = sigusr2.recv() => {
                    tracing::info!("Received SIGUSR2, incrementing by 10");
                    increment_by(10);
                }
            }
        }
    })
}

#[tokio::main]
async fn main() {
    // Initialize tracing for better logging
    tracing_subscriber::fmt::init();

    // Print process information
    let pid = std::process::id();
    let exe_path = std::env::current_exe().unwrap_or_default();
    let parent_pid = unsafe { libc::getppid() };

    println!("\n=== Process Information ===");
    println!("PID:        {}", pid);
    println!("Parent PID: {}", parent_pid);
    println!("Executable: {}", exe_path.display());
    println!("\n=== Usage ===");
    println!("Send signals to this process to modify the counter:");
    println!("  kill -SIGUSR1 {}  # Increment by 1", pid);
    println!("  kill -SIGUSR2 {}  # Increment by 10", pid);
    println!("  kill -TERM {}     # Exit gracefully", pid);
    println!("  # or press Ctrl+C");
    println!();

    // Print initial counter value
    if let Ok(counter) = COUNTER.lock() {
        println!("Initial counter value: {}", *counter);
    }
    println!();

    // Spawn signal handler
    let _signal_handler = spawn_signal_handler();

    // Wait for Ctrl+C
    tokio::signal::ctrl_c().await
        .expect("failed to listen for ctrl+c");

    // Print final counter value
    println!("\n=== Final State ===");
    if let Ok(counter) = COUNTER.lock() {
        println!("Final counter value: {}\n", *counter);
    }
}
