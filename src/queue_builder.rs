use crate::{features::*, queue::Queue, security::Security};

pub struct QueueBuilder<
    J = EmptyJournal,
    T = EmptyTransactionalQueue,
    E = AnonymousEncryption,
    D = EmptyDeadletterQueue,
> {
    name: String,
    encryption: E,
    _marker: std::marker::PhantomData<(J, T, E, D)>,
}

impl
    QueueBuilder<EmptyJournal, EmptyTransactionalQueue, AnonymousEncryption, EmptyDeadletterQueue>
{
    pub fn new(name: &str) -> QueueBuilder {
        QueueBuilder {
            name: name.to_string(),
            encryption: AnonymousEncryption,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<J, T, E, D> QueueBuilder<J, T, E, D>
where
    J: Default + Journal,
    D: Default,
    E: EncryptionType,
{
    pub fn build(self) -> Queue<J, T, E, D> {
        Queue::<J, T, E, D>::new(&self.name, J::default(), D::default(), self.encryption)
    }

    pub fn with_journaling(self) -> QueueBuilder<JournaledQueue<E>, T, E, D> {
        QueueBuilder {
            name: self.name,
            encryption: self.encryption,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_transactional(self) -> QueueBuilder<J, TransactionalQueue, E, D> {
        QueueBuilder {
            name: self.name,
            encryption: self.encryption,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_encryption(self, security: Security) -> QueueBuilder<J, T, BasicEncryption, D> {
        QueueBuilder {
            name: self.name,
            encryption: BasicEncryption(security),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_dlq(self) -> QueueBuilder<J, T, E, DeadletterQueue<E>> {
        QueueBuilder {
            name: self.name,
            encryption: self.encryption,
            _marker: std::marker::PhantomData,
        }
    }
}
