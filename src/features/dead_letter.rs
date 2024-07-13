use crate::{
    message::Message,
    queue::{BasicQueue, Queue},
    Result,
};

use super::Journal;

#[derive(Default, Clone)]
pub struct DeadletterQueue<E>(BasicQueue<Message<E>>);

#[derive(Default, Clone)]
pub struct EmptyDeadletterQueue;

impl<J, T, E> Queue<J, T, E, DeadletterQueue<E>>
where
    J: Journal + Clone,
    T: Clone,
    E: Clone,
{
    pub fn move_to_dlq(&mut self) -> Result<()> {
        if let Some(message) = self.receive() {
            self.dlq
                .0
                .lock()
                .expect("Couldnt lock queue")
                .push_back(message);
        }

        Ok(())
    }

    pub fn dlq_count(&self) -> usize {
        self.dlq.0.lock().expect("Couldnt lock queue").len()
    }
}

#[cfg(test)]
mod tests {
    use crate::{message::Message, queue_builder::QueueBuilder};

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
