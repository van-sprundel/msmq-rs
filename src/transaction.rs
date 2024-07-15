use crate::error::MSMQError;
use crate::features::{
    AnonymousEncryption, BasicEncryption, EmptyDeadletterQueue, EmptyJournal, TransactionalQueue,
};
use crate::message::Message;
use crate::queue::{Queue, QueueOps};
use crate::queue_builder::QueueBuilder;
use crate::security::Security;
use crate::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Transaction<E> {
    pub operations: Mutex<HashMap<String, Vec<Message<E>>>>,
}

impl<E> Transaction<E> {
    pub fn new() -> Self {
        Transaction {
            operations: Mutex::new(HashMap::new()),
        }
    }
}

impl<E> Transaction<E> {
    pub fn commit(&self) -> Result<()> {
        let operations = self.operations.lock().unwrap();
        for (queue_name, messages) in operations.iter() {
            //TODO: get queue from somewhere

            for message in messages {
                //    queue.send(message.clone())?;
            }
        }
        Ok(())
    }
}
