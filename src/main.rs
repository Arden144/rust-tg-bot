mod tdjson;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::sync::mpsc;

fn main() {
    // Create Tdlib wrapper instance
    let tdlib = Arc::new(Mutex::new(tdjson::Tdlib::new()));

    // Receiver thread
    let tdlib_thread = tdlib.clone();
    let (tx, rx) = mpsc::channel();
    let reciever = thread::spawn(move || {
        let now = Instant::now();
        while now.elapsed().as_millis() < 10000 {
            let tdlib_lock = tdlib_thread.lock().unwrap();
            match tdlib_lock.receive(1.0) {
                Ok(result) => tx.send(result).unwrap(),
                Err(_) => (),
            }
            std::mem::drop(tdlib_lock);
        }
    });

    // Orchestrator thread
    // Wait for messages from other threads
    loop {
        match rx.recv() {
            Ok(message) => println!("{}", message),
            Err(_) => break,
        }
    }

    // Wait for all threads to reach a safe state before terminating
    reciever.join().unwrap();
}
