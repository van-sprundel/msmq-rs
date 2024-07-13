use crate::{queue::Queue, Result};

pub struct DeadletterQueued;
pub struct NonDeadletterQueued;

impl<J, T, E> Queue<J, T, E, DeadletterQueued> {
    pub fn move_to_dlq(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn dlq_count(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::{message::Message, queue_builder::QueueBuilder};

    use super::*;

    #[test]
    fn test_dead_letter_queue() {
        let mut queue = QueueBuilder::new("test_queue").with_dlq().build();

        let message = Message::new("Undeliverable message");
        queue.send(message).unwrap();
        queue.move_to_dlq().unwrap();

        assert_eq!(queue.message_count().unwrap(), 0);
        assert_eq!(queue.dlq_count(), 1);
    }
}
