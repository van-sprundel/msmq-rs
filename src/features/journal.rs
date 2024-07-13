use crate::{
    message::Message,
    queue::{BasicQueue, Queue},
};

#[derive(Default)]
pub struct JournaledQueue<E>(pub BasicQueue<Message<E>>);

pub trait Journal {
    fn append_journal_messages(&self, content: &str);
}

#[derive(Default)]
pub struct EmptyJournal;

impl Journal for EmptyJournal {
    fn append_journal_messages(&self, _message: &str) {
        // no-op
    }
}

impl<E> Journal for JournaledQueue<E> {
    fn append_journal_messages(&self, content: &str) {
        let mut queue = self.0.lock().expect("Couldn't lock queue");
        queue.push_back(Message::new(&*format!("Sent: {}", content)));
        queue.push_back(Message::new(&*format!("Received: {}", content)));
    }
}

impl<T, E, D> Queue<JournaledQueue<E>, T, E, D> {
    pub fn journal_length(&self) -> usize {
        self.journaled_queue
            .0
            .as_ref()
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
