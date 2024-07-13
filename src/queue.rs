use crate::{
    distributed_transaction::DistributedTransaction, error::MSMQError, features::*,
    message::Message, multicast_group::MulticastGroup, security::Security, Result,
};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub type BasicQueue<E> = Arc<Mutex<VecDeque<Message<E>>>>;

pub struct Queue<J = NonJournaled, T = NonTransactional, E = NonEncrypted, D = NonDeadletterQueued>
{
    pub(crate) name: String,
    pub(crate) queue: BasicQueue<E>,
    pub(crate) journaled_queue: Option<BasicQueue<E>>,
    pub(crate) security: Option<Security>,
    _marker: std::marker::PhantomData<(J, T, E, D)>,
}

impl<J, T, E, D> Queue<J, T, E, D> {
    pub fn new(name: &str, security: Option<Security>) -> Self {
        Self {
            name: name.to_string(),
            queue: Arc::new(Mutex::new(VecDeque::new())),
            journaled_queue: None,
            security,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn send(&mut self, message: Message<E>) -> Result<()> {
        self.queue
            .lock()
            .map_err(|e| MSMQError::Custom(e.to_string()))?
            .push_back(message);
        Ok(())
    }

    pub fn send_distributed_transactional(
        &mut self,
        message: Message<E>,
        distributed_transaction: &DistributedTransaction,
    ) -> Result<()> {
        Ok(())
    }

    pub fn join_group(&mut self, group: &MulticastGroup) -> Result<()> {
        Ok(())
    }

    pub fn receive(&mut self) -> Option<Message<E>> {
        self.queue
            .lock()
            .expect("Failed to lock the queue")
            .pop_front()
    }

    pub fn message_count(&self) -> Result<usize> {
        Ok(self
            .queue
            .lock()
            .map_err(|e| MSMQError::Custom(e.to_string()))?
            .len())
    }
}

#[cfg(test)]
mod tests {
    use crate::{message::Priority, queue_builder::QueueBuilder};

    use super::*;

    #[test]
    fn test_send_message_to_queue() {
        let mut queue = QueueBuilder::new("test_queue").build();
        let message = Message::new("Test message");
        assert!(queue.send(message).is_ok());
    }

    #[test]
    fn test_retrieve_message_from_queue() {
        let mut queue = QueueBuilder::new("test_queue").build();

        let message = Message::new("Test message");
        queue.send(message).unwrap();

        let received = queue.receive();
        assert!(received.is_some());
        assert_eq!(received.unwrap().content(), "Test message");
    }

    #[test]
    fn test_message_prioritization() {
        let mut queue = QueueBuilder::new("test_queue").build();

        let high_priority = Message::new("High priority").with_priority(Priority::High);
        let low_priority = Message::new("Low priority").with_priority(Priority::Low);

        queue.send(low_priority).unwrap();
        queue.send(high_priority).unwrap();

        assert_eq!(queue.receive().unwrap().content(), "High priority");
    }
}
