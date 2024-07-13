use crate::error::MSMQError;
use crate::features::{EmptyDeadletterQueue, EmptyJournal, TransactionalQueue, QUEUE_REGISTRY};
use crate::message::Message;
use crate::queue::Queue;
use crate::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Transaction<E> {
    pub operations: Arc<Mutex<HashMap<String, Vec<Message<E>>>>>,
}

impl<E> Transaction<E>
where
    E: Clone + 'static,
{
    pub fn new() -> Self {
        Transaction {
            operations: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn commit(&self) -> Result<()> {
        let operations = self.operations.lock().unwrap();
        let registry = QUEUE_REGISTRY.lock().unwrap();
        for (queue_name, messages) in operations.iter() {
            if let Some(queue_arc) = registry.get(queue_name) {
                let mut queue_mutex = queue_arc.lock().unwrap();

                //TODO: can we use generics here instead of casting to unimpl's?
                if let Some(queue) = queue_mutex.downcast_mut::<Queue<
                    EmptyJournal,
                    TransactionalQueue,
                    E,
                    EmptyDeadletterQueue,
                >>() {
                    for message in messages {
                        queue.send(message.clone())?;
                    }
                } else {
                    return Err(MSMQError::Custom(format!(
                        "Queue '{}' is not of the expected type",
                        queue_name
                    )));
                }
            } else {
                return Err(MSMQError::Custom(format!(
                    "Queue '{}' not found in registry",
                    queue_name
                )));
            }
        }
        Ok(())
    }
}
