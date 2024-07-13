use crate::Result;
use crate::{message::Message, queue::Queue, transaction::Transaction};

pub struct Transactional;
pub struct NonTransactional;

impl<J, E, D> Queue<J, Transactional, E, D> {
    pub fn send_transactional(&self, message: Message, transaction: &Transaction) -> Result<()> {
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
            let mut queue = QueueBuilder::new("test_queue").with_transactional().build();

            let message = Message::new("Test message");
            queue.send(message).unwrap();

            assert_eq!(queue.message_count().unwrap(), 1);
        }
        // out of scope, e.g. system is shut off
        {
            let recovered_queue = QueueBuilder::new("test_queue").with_transactional().build();
            assert_eq!(recovered_queue.message_count().unwrap(), 1);
        }
    }
}
