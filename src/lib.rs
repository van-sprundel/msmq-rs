#![allow(unused)]

pub mod distributed_transaction;
mod error;
pub mod features;
pub mod message;
pub mod multicast_group;
pub mod queue;
pub mod queue_builder;
pub mod security;
pub mod transaction;

use crate::queue::QueueOps;
use error::{MSMQError, Result};
use message::Message;
use queue::Queue;
use queue_builder::QueueBuilder;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Serialize, Deserialize, Debug)]
enum ReceivedMessage {
    Enqueue { content: String },
    Dequeue,
}

#[derive(Serialize, Deserialize, Debug)]
enum Response {
    Success,
    Error { message: String },
    Dequeued { content: String },
}

struct QueueServer {
    queue: Arc<Mutex<Queue>>,
}

impl QueueServer {
    fn new(queue_path: &str) -> Result<Self> {
        let queue = QueueBuilder::new(queue_path).build();
        Ok(QueueServer {
            queue: Arc::new(Mutex::new(queue)),
        })
    }

    fn start(&self, address: &str) -> Result<()> {
        let listener = TcpListener::bind(address)?;
        println!("Server listening on {}", address);

        for stream in listener.incoming() {
            let stream = stream?;
            let queue = Arc::clone(&self.queue);
            thread::spawn(move || {
                if let Err(e) = handle_client(stream, queue) {
                    eprintln!("Error handling client: {}", e);
                }
            });
        }
        Ok(())
    }
}

fn handle_client(mut stream: TcpStream, queue: Arc<Mutex<Queue>>) -> Result<()> {
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            return Ok(());
        }

        let received_message: ReceivedMessage = serde_json::from_slice(&buffer[..bytes_read])?;
        let response = match received_message {
            ReceivedMessage::Enqueue { content } => {
                let mut queue = queue.lock().unwrap();
                match queue.send(Message::new(&content)) {
                    Ok(_) => Response::Success,
                    Err(e) => Response::Error {
                        message: e.to_string(),
                    },
                }
            }
            ReceivedMessage::Dequeue => {
                let mut queue = queue.lock().unwrap();
                match queue.receive() {
                    Some(msg) => Response::Dequeued {
                        content: msg.content().to_string(),
                    },
                    None => Response::Error {
                        message: "Queue is empty".to_string(),
                    },
                }
            }
        };

        let response_json = serde_json::to_vec(&response)?;
        stream.write_all(&response_json)?;
    }
}

pub fn run_server(queue_path: &str, address: &str) -> Result<()> {
    let server = QueueServer::new(queue_path)?;
    server.start(address)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;

    fn start_test_server(queue_path: String, address: String) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let server = QueueServer::new(&queue_path).unwrap();
            server.start(&address).unwrap();
        })
    }

    fn send_message(address: &str, message: ReceivedMessage) -> Response {
        let mut stream = TcpStream::connect(address).unwrap();
        let message_json = serde_json::to_vec(&message).unwrap();
        stream.write_all(&message_json).unwrap();

        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();
        serde_json::from_slice(&buffer[..bytes_read]).unwrap()
    }

    #[test]
    fn test_enqueue_and_dequeue() {
        let address = "127.0.0.1:8001".to_string();
        let server_handle =
            start_test_server("./test_enqueue_dequeue.msmq".to_string(), address.clone());
        thread::sleep(Duration::from_millis(100)); // Give the server time to start

        let enqueue_response = send_message(
            &address,
            ReceivedMessage::Enqueue {
                content: "Test message".to_string(),
            },
        );
        assert!(matches!(enqueue_response, Response::Success));

        let dequeue_response = send_message(&address, ReceivedMessage::Dequeue);
        assert!(
            matches!(dequeue_response, Response::Dequeued { content } if content == "Test message")
        );

        // dequeue from an empty queue
        let empty_dequeue_response = send_message(&address, ReceivedMessage::Dequeue);
        assert!(
            matches!(empty_dequeue_response, Response::Error { message } if message == "Queue is empty")
        );
    }

    #[test]
    fn test_multiple_clients() {
        let address = "127.0.0.1:8002".to_string();
        let server_handle =
            start_test_server("./test_multiple_clients.msmq".to_string(), address.clone());
        thread::sleep(Duration::from_millis(100)); // Give the server time to start

        let address_clone = address.clone();
        let client1 = thread::spawn(move || {
            send_message(
                &address_clone,
                ReceivedMessage::Enqueue {
                    content: "Message 1".to_string(),
                },
            )
        });
        thread::sleep(Duration::from_millis(100)); // Give the server time

        let address_clone = address.clone();
        let client2 = thread::spawn(move || {
            send_message(
                &address_clone,
                ReceivedMessage::Enqueue {
                    content: "Message 2".to_string(),
                },
            )
        });
        thread::sleep(Duration::from_millis(100)); // Give the server time

        let address_clone = address.clone();
        let client3 = thread::spawn(move || send_message(&address_clone, ReceivedMessage::Dequeue));
        thread::sleep(Duration::from_millis(100)); // Give the server time

        let address_clone = address.clone();
        let client4 = thread::spawn(move || send_message(&address_clone, ReceivedMessage::Dequeue));
        thread::sleep(Duration::from_millis(100)); // Give the server time

        assert!(matches!(client1.join().unwrap(), Response::Success));
        assert!(matches!(client2.join().unwrap(), Response::Success));

        let dequeue1 = client3.join().unwrap();
        let dequeue2 = client4.join().unwrap();

        assert!(
            (matches!(dequeue1, Response::Dequeued { ref content } if content == "Message 1")
                && matches!(dequeue2, Response::Dequeued { ref content } if content == "Message 2"))
                || (matches!(dequeue1, Response::Dequeued { ref content } if content == "Message 2")
                    && matches!(dequeue2, Response::Dequeued { ref content } if content == "Message 1"))
        );
    }

    #[test]
    fn test_server_persistence() {
        let address = "127.0.0.1:8003".to_string();
        let queue_path = "./test_persistence.msmq".to_string();

        {
            let server_handle = start_test_server(queue_path.clone(), address.clone());
            thread::sleep(Duration::from_millis(100));

            let enqueue_response = send_message(
                &address,
                ReceivedMessage::Enqueue {
                    content: "Persistent message".to_string(),
                },
            );
            assert!(matches!(enqueue_response, Response::Success));

            // server shut down
            drop(server_handle);
        }

        {
            let server_handle = start_test_server(queue_path, address.clone());
            thread::sleep(Duration::from_millis(100));

            let dequeue_response = send_message(&address, ReceivedMessage::Dequeue);
            assert!(
                matches!(dequeue_response, Response::Dequeued { content } if content == "Persistent message")
            );
        }
    }
}
