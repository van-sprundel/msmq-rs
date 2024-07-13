use crate::{features::*, queue::Queue, security::Security};

pub struct QueueBuilder<
    J = NonJournaled,
    T = NonTransactional,
    E = NonEncrypted,
    D = NonDeadletterQueued,
> {
    name: String,
    security: Option<Security>,
    _marker: std::marker::PhantomData<(J, T, E, D)>,
}

impl QueueBuilder<NonJournaled, NonTransactional, NonEncrypted, NonDeadletterQueued> {
    pub fn new(name: &str) -> QueueBuilder {
        QueueBuilder {
            name: name.to_string(),
            security: None,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<J, T, E, D> QueueBuilder<J, T, E, D> {
    pub fn build(self) -> Queue<J, T, E, D> {
        Queue::<J, T, E, D>::new(&self.name, self.security)
    }

    pub fn with_journaling(self) -> QueueBuilder<Journaled, T, E, D> {
        QueueBuilder {
            name: self.name,
            security: self.security,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_transactional(self) -> QueueBuilder<J, Transactional, E, D> {
        QueueBuilder {
            name: self.name,
            security: self.security,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_encryption(self, security: Security) -> QueueBuilder<J, T, Encrypted, D> {
        QueueBuilder {
            name: self.name,
            security: Some(security),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_dlq(self) -> QueueBuilder<J, T, E, DeadletterQueued> {
        QueueBuilder {
            name: self.name,
            security: self.security,
            _marker: std::marker::PhantomData,
        }
    }
}
