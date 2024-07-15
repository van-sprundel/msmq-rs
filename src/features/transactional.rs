use lazy_static::lazy_static;

use crate::Result;
use crate::{message::Message, queue::Queue, transaction::Transaction};
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::Journal;

#[derive(Default, Clone)]
pub struct TransactionalQueue;

#[derive(Default, Clone)]
pub struct EmptyTransactionalQueue;

impl<J, E, D> Queue<J, TransactionalQueue, E, D>
where
    J: Journal + Clone + Send + Sync,
    E: Clone + Send + Sync,
    D: Clone + Send + Sync,
{
    pub fn send_transactional(&self, message: Message<E>, txn: &Transaction<E>) -> Result<()> {
        txn.operations
            .lock()
            .unwrap()
            .entry(self.name.clone())
            .or_default()
            .push(message);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::queue::QueueOps;
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
}
