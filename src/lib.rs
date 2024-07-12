#![allow(unused)]

use std::{
    collections::VecDeque,
    default,
    sync::{Arc, Mutex},
};

use error::Result;

mod error;

pub struct Security {
    username: String,
    password: String,
}

impl Security {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

pub struct MulticastGroup {
    name: String,
}

impl MulticastGroup {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn send(&self, message: Message) -> Result<()> {
        Ok(())
    }
}

pub struct Queue {
    name: String,
    queue: Arc<Mutex<VecDeque<Message>>>,
}

impl Queue {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn recover(name: &str) -> Self {
        Self {
            name: name.to_string(),
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn with_journaling(self) -> JournaledQueue {
        JournaledQueue {
            name: self.name,
            queue: self.queue,
            journal_queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn with_security(mut self, security: Security) -> Self {
        self
    }

    pub fn join_group(mut self, multicast_group: &MulticastGroup) -> Self {
        self
    }

    pub fn send(&self, message: Message) -> Result<()> {
        Ok(())
    }

    pub fn send_encrypted(&self, message: Message<Encrypted>) -> Result<()> {
        Ok(())
    }

    pub fn send_transactional(&self, message: Message, transaction: &Transaction) -> Result<()> {
        Ok(())
    }

    pub fn send_distributed_transactional(
        &self,
        message: Message,
        dtx: &DistributedTransaction,
    ) -> Result<()> {
        Ok(())
    }

    pub fn receive(&self) -> Option<Message> {
        None
    }

    pub fn receive_authenticated(
        &self,
        username: &str,
        password: &str,
    ) -> Option<Message<Encrypted>> {
        None
    }

    pub fn message_count(&self) -> usize {
        self.queue.lock().into_iter().len()
    }

    pub fn move_to_dlq(&self, dlq: &DeadLetterQueue) -> Result<()> {
        Ok(())
    }
}

pub struct JournaledQueue {
    name: String,
    queue: Arc<Mutex<VecDeque<Message>>>,
    journal_queue: Arc<Mutex<Vec<Message>>>,
}

impl JournaledQueue {
    pub fn journal_length(&self) -> usize {
        self.journal_queue
            .lock()
            .expect("Couldnt get journal queue")
            .len()
    }

    pub fn message_count(&self) -> usize {
        self.queue.lock().into_iter().len()
    }

    pub fn send(&self, message: Message) -> Result<()> {
        Ok(())
    }

    pub fn receive(&self) -> Option<Message> {
        None
    }
}

pub struct DeadLetterQueue {
    name: String,
    queue: Arc<Mutex<VecDeque<Message>>>,
}

impl DeadLetterQueue {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn message_count(&self) -> usize {
        self.queue.lock().into_iter().len()
    }
}

#[derive(Default)]
pub enum Priority {
    High,
    #[default]
    Low,
}

pub struct Encrypted;
pub struct Decrypted;

pub struct Message<S = Decrypted> {
    content: String,
    priority: Priority,
    state: std::marker::PhantomData<S>,
}

impl Message<Decrypted> {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            priority: Priority::default(),
            state: Default::default(),
        }
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn encrypt(self) -> Message<Encrypted> {
        Message {
            content: self.content,
            priority: self.priority,
            state: std::marker::PhantomData::<Encrypted>,
        }
    }

    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }
}

impl Message<Encrypted> {
    pub fn decrypt(self) -> Message<Decrypted> {
        Message {
            content: self.content,
            priority: self.priority,
            state: std::marker::PhantomData::<Decrypted>,
        }
    }
}

pub struct Transaction {}

impl Transaction {
    pub fn new() -> Self {
        Self {}
    }

    pub fn commit(&self) -> Result<()> {
        Ok(())
    }
}

pub struct DistributedTransaction {}

impl DistributedTransaction {
    pub fn new() -> Self {
        Self {}
    }

    pub fn commit(&self) -> Result<()> {
        Ok(())
    }

    pub fn prepare(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_message_to_queue() {
        let queue = Queue::new("test_queue");
        let message = Message::new("Test message");
        assert!(queue.send(message).is_ok());
    }

    #[test]
    fn test_retrieve_message_from_queue() {
        let queue = Queue::new("test_queue");

        let message = Message::new("Test message");
        queue.send(message).unwrap();

        let received = queue.receive();
        assert!(received.is_some());
        assert_eq!(received.unwrap().content(), "Test message");
    }

    #[test]
    fn test_guaranteed_delivery() {
        {
            let queue = Queue::new("test_queue");
            let message = Message::new("Test message");
            queue.send(message).unwrap();
            assert_eq!(queue.message_count(), 1);
        }
        // out of scope, system is shut off
        {
            let recovered_queue = Queue::recover("test_queue");
            assert_eq!(recovered_queue.message_count(), 1);
        }
    }

    #[test]
    fn test_transactional_operations() {
        let queue = Queue::new("test_queue");

        let message = Message::new("Test message");
        let txn = Transaction::new();
        queue.send_transactional(message, &txn).unwrap();
        assert_eq!(queue.message_count(), 0);

        txn.commit();
        assert_eq!(queue.message_count(), 1);
    }

    #[test]
    fn test_message_prioritization() {
        let queue = Queue::new("test_queue");

        let high_priority = Message::new("High priority").with_priority(Priority::High);
        let low_priority = Message::new("Low priority").with_priority(Priority::Low);

        queue.send(low_priority).unwrap();
        queue.send(high_priority).unwrap();

        assert_eq!(queue.receive().unwrap().content(), "High priority");
    }

    #[test]
    fn test_dead_letter_queue() {
        let queue = Queue::new("test_queue");
        let dlq = DeadLetterQueue::new("test_dlq");

        let message = Message::new("Undeliverable message");
        queue.send(message).unwrap();
        queue.move_to_dlq(&dlq).unwrap();

        assert_eq!(queue.message_count(), 0);
        assert_eq!(dlq.message_count(), 1);
    }

    #[test]
    fn test_authentication_and_encryption() {
        let queue = Queue::new("secure_queue").with_security(Security::new("user", "password"));

        let message = Message::new("Secure message").encrypt();
        queue.send_encrypted(message).unwrap();

        let received = queue.receive_authenticated("user", "password").unwrap();
        assert_eq!(received.decrypt().content(), "Secure message");
    }

    #[test]
    fn test_distributed_transactions() {
        let queue1 = Queue::new("queue1");
        let queue2 = Queue::new("queue2");
        let dtx = DistributedTransaction::new();

        queue1
            .send_distributed_transactional(Message::new("Message 1"), &dtx)
            .unwrap();
        queue2
            .send_distributed_transactional(Message::new("Message 2"), &dtx)
            .unwrap();

        dtx.prepare();
        dtx.commit();

        assert_eq!(queue1.message_count(), 1);
        assert_eq!(queue2.message_count(), 1);
    }

    #[test]
    fn test_journaling() {
        let queue = Queue::new("journal_queue").with_journaling();
        queue.send(Message::new("Journaled message")).unwrap();
        queue.receive();
        assert_eq!(queue.journal_length(), 2); // One send, one receive
    }

    #[test]
    fn test_multicast_messaging() {
        let multicast_group = MulticastGroup::new("test_group");
        let queue1 = Queue::new("queue1").join_group(&multicast_group);
        let queue2 = Queue::new("queue2").join_group(&multicast_group);

        multicast_group
            .send(Message::new("Multicast message"))
            .unwrap();

        assert_eq!(queue1.receive().unwrap().content(), "Multicast message");
        assert_eq!(queue2.receive().unwrap().content(), "Multicast message");
    }
}
