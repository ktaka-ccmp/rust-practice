/// This program demonstrates handling Unix signals using the `tokio` asynchronous runtime.
/// It listens for SIGINT, SIGTERM, and SIGHUP signals and performs specific actions upon receiving them.
///
/// The `handle_signals` function sets up signal listeners for SIGINT, SIGTERM, and SIGHUP.
/// - On receiving SIGINT, it toggles a boolean flag and sends it through a `watch` channel.
/// - On receiving SIGTERM, it prints a message and breaks the loop to initiate shutdown.
/// - On receiving SIGHUP, it prints a message indicating that it received the signal.
///
/// The `main` function initializes a `watch` channel and spawns two asynchronous tasks:
/// - `main_task`: This task continuously checks the boolean flag from the `watch` channel and performs actions based on its value.
/// - `signal_task`: This task runs the `handle_signals` function to listen for signals.
///
/// The program uses `tokio::select!` to wait for either the `main_task` to complete or the `signal_task` to receive a shutdown signal.
use tokio::signal::unix::{signal, SignalKind};
use std::time::Duration;
use tokio::sync::watch;

async fn handle_signals(tx: watch::Sender<bool>) {
    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    let mut sigterm = signal(SignalKind::terminate()).unwrap();
    let mut sighup = signal(SignalKind::hangup()).unwrap();

    let mut flag = false;

    loop {
        tokio::select! {
            _ = sigint.recv() => {
                println!("Received SIGINT");
                flag = !flag;
                tx.send(flag).unwrap();
                // tokio::time::sleep(Duration::from_secs(1)).await;
                // tx.send(false).unwrap();
                // break;
            }
            _ = sigterm.recv() => {
                println!("Received SIGTERM");
                // Handle SIGTERM
                break;
            }
            _ = sighup.recv() => {
                println!("Received SIGHUP");
                // Handle SIGHUP (e.g., reload configuration)
            }
        }
    }

    // Cleanup code here
    println!("Shutting down...");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (tx, rx) = watch::channel(false);

    let main_task = tokio::spawn(async move {
        loop {
            if *rx.borrow() {
                println!("Received HUP signal...");
                println!("Send Hup signal to restart...");
                loop {
                    if !*rx.borrow() {
                        break;
                    }
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
                println!("Main task resumed.");
            }
            println!("main loop");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    let signal_task = tokio::spawn(handle_signals(tx));

    tokio::select! {
        _ = main_task => println!("Main task completed"),
        _ = signal_task => println!("Received shutdown signal"),
    }

    Ok(())
}
