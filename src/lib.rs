pub mod distributed_transaction;
mod error;
pub mod features;
pub mod message;
pub mod multicast_group;
pub mod queue;
pub mod queue_builder;
pub mod security;
pub mod transaction;

use error::{MSMQError, Result};

