mod tdjson;
use std::thread;
use std::sync::Arc;
use std::time::Instant;
use std::sync::mpsc;
use json;
use base64;

fn main() {
    // Create Tdlib wrapper instance
    tdjson::Tdlib::log_path("/home/sincl/rust-tg-bot/log.txt");
    let tdlib = Arc::new(tdjson::Tdlib::new());

    // Receiver thread
    let tdlib_thread = tdlib.clone();
    let (tx, rx) = mpsc::channel();
    let reciever = thread::spawn(move || {
        let now = Instant::now();
        while now.elapsed().as_millis() < 10000 {
            match tdlib_thread.receive(1.0) {
                Ok(result) => tx.send(result).unwrap(),
                Err(_) => (),
            }
        }
    });

    // Orchestrator thread
    // Wait for messages from other threads
    loop {
        match rx.recv() {
            Ok(message) => {
                let obj = json::parse(&message).unwrap();
                println!("{}", json::stringify_pretty(obj.clone(), 2));
                match obj["@type"].as_str().unwrap() {
                    "updateAuthorizationState" => {
                        match obj["authorization_state"]["@type"].as_str().unwrap() {
                            "authorizationStateWaitTdlibParameters" => {
                                let response = json::object!{
                                    "@type": "setTdlibParameters",
                                    "parameters": {
                                        "@type": "tdlibParameters",
                                        "use_test_dc": true,
                                        "use_file_database": true,
                                        "use_chat_info_database": true,
                                        "use_message_database": true,
                                        "use_secret_chats": false,
                                        "api_id": 0, // removed
                                        "api_hash": "removed",
                                        "system_language_code": "en-US",
                                        "device_model": "Bot",
                                        "system_version": "Ubuntu-20.04",
                                        "application_version": "0.1.0",
                                        "enable_storage_optimizer": true,
                                        "ignore_file_names": false,
                                    }
                                };
                                println!("{}", json::stringify_pretty(response.clone(), 2));
                                tdlib.send(&json::stringify(response));
                            },
                            "authorizationStateWaitEncryptionKey" => {
                                let response = json::object!{
                                    "@type": "setDatabaseEncryptionKey",
                                    "new_encryption_key": base64::encode("removed"),
                                };
                                println!("{}", json::stringify_pretty(response.clone(), 2));
                                tdlib.send(&json::stringify(response));
                            },
                            "authorizationStateWaitPhoneNumber" => {
                                let response = json::object!{
                                    "@type": "checkAuthenticationBotToken",
                                    "token": "removed",
                                };
                                println!("{}", json::stringify_pretty(response.clone(), 2));
                                tdlib.send(&json::stringify(response));
                            }
                            _ => (),
                        }
                    },
                    _ => (),
                }
            },
            Err(_) => break,
        }
    }

    // Wait for all threads to reach a safe state before terminating
    reciever.join().unwrap();
}
