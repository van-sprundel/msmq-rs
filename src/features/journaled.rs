use crate::queue::Queue;
pub struct NonJournaled;
pub struct Journaled;

impl<T, E, D> Queue<Journaled, T, E, D> {
    pub fn journal_length(&self) -> usize {
        self.journaled_queue
            .as_ref()
            .expect("Journaled queue does not have a journal queue")
            .lock()
            .expect("Failed to lock journal queue")
            .len()
    }
}

#[cfg(test)]
mod tests {
    use crate::{message::Message, queue_builder::QueueBuilder};

    #[test]
    fn test_journaling() {
        let mut queue = QueueBuilder::new("journal_queue").with_journaling().build();

        queue.send(Message::new("Journaled message")).unwrap();
        queue.receive();

        assert_eq!(queue.journal_length(), 2); // One send, one receive
    }
}
