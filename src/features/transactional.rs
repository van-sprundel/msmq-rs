use lazy_static::lazy_static;

use crate::Result;
use crate::{message::Message, queue::Queue, transaction::Transaction};
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::Journal;

#[derive(Clone)]
pub struct TransactionalQueue;
#[derive(Clone)]
pub struct EmptyTransactionalQueue;

lazy_static! {
    pub static ref QUEUE_REGISTRY: Arc<Mutex<HashMap<String, Arc<Mutex<dyn Any + Send + Sync>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

impl<J, E, D> Queue<J, TransactionalQueue, E, D>
where
    J: Journal + Clone,
    E: Clone,
    D: Clone,
{
    pub fn send_transactional(&self, message: Message<E>, txn: &Transaction<E>) -> Result<()> {
        txn.operations
            .lock()
            .unwrap()
            .entry(self.name.clone())
            .or_insert_with(Vec::new)
            .push(message);
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use crate::{message::Message, queue_builder::QueueBuilder, transaction::Transaction};

    #[test]
    fn test_transactional_operations() {
        let queue = QueueBuilder::new("test_queue").with_transactional().build();

        let message = Message::new("Test message");
        let txn = Transaction::new();
        queue.send_transactional(message, &txn).unwrap();
        assert_eq!(queue.message_count().unwrap(), 0);

        txn.commit().unwrap();
        assert_eq!(queue.message_count().unwrap(), 1);
    }

    #[test]
    fn test_guaranteed_delivery() {
        {
            let mut queue = QueueBuilder::new("test_queue").build();

            let message = Message::new("Test message");
            queue.send(message).unwrap();

            assert_eq!(queue.message_count().unwrap(), 1);
        }
        // out of scope, e.g. system is shut off
        {
            let recovered_queue = QueueBuilder::new("test_queue").build();
            assert_eq!(recovered_queue.message_count().unwrap(), 1);
        }
    }
}
