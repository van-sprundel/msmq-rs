use crate::{
    distributed_transaction::DistributedTransaction, error::MSMQError, features::*,
    message::Message, multicast_group::MulticastGroup, Result,
};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub trait QueueOps<E>: Send + Sync
where
    E: EncryptFeature,
{
    fn send(&mut self, message: Message<E>) -> Result<()>;
    fn send_distributed_transactional(
        &mut self,
        message: Message<E>,
        distributed_transaction: &DistributedTransaction,
    ) -> Result<()>;
    fn receive(&mut self) -> Option<Message<E>>;
    fn join_group(&mut self, group: &MulticastGroup) -> Result<()>;
    fn message_count(&self) -> Result<usize>;
}

pub type BasicQueue<T> = Arc<Mutex<VecDeque<T>>>;

#[derive(Clone)]
pub struct Queue<
    J = EmptyJournal,
    T = EmptyTransactionalQueue,
    E = AnonymousEncryption,
    D = EmptyDeadletterQueue,
> where
    E: EncryptFeature,
    D: DeadLetterFeature,
{
    pub(crate) name: String,
    pub(crate) queue: BasicQueue<Message<E>>,
    pub(crate) journaled_queue: J,
    pub(crate) dlq: D,
    pub(crate) security: E,
    _marker: std::marker::PhantomData<(J, T, E, D)>,
}

impl<J, T, E, D> Queue<J, T, E, D>
where
    E: EncryptFeature,
    D: DeadLetterFeature,
{
    pub fn new(name: &str, j: J, e: E, d: D) -> Self {
        Self {
            name: name.to_string(),
            queue: Arc::new(Mutex::new(VecDeque::new())),
            journaled_queue: j,
            dlq: d,
            security: e,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<J, T, E, D> QueueOps<E> for Queue<J, T, E, D>
where
    J: JournalFeature,
    T: TransactionalFeature,
    E: EncryptFeature,
    D: DeadLetterFeature,
{
    fn send(&mut self, message: Message<E>) -> Result<()> {
        self.queue
            .lock()
            .map_err(|e| MSMQError::Custom(e.to_string()))?
            .push_back(message);

        Ok(())
    }

    fn send_distributed_transactional(
        &mut self,
        message: Message<E>,
        distributed_transaction: &DistributedTransaction,
    ) -> Result<()> {
        Ok(())
    }

    fn receive(&mut self) -> Option<Message<E>> {
        let result = self
            .queue
            .lock()
            .expect("Failed to lock the queue")
            .pop_front();

        if let Some(ref message) = result {
            self.journaled_queue
                .append_journal_messages(message.content());
        }

        result
    }

    fn join_group(&mut self, group: &MulticastGroup) -> Result<()> {
        Ok(())
    }

    fn message_count(&self) -> Result<usize> {
        Ok(self
            .queue
            .lock()
            .map_err(|e| MSMQError::Custom(e.to_string()))?
            .len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::queue_builder::QueueBuilder;

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
}
