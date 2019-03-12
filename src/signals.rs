/// functionality needed:
/// 1. a channel should not connect from/to an identical agent.

use std::sync::{Arc, Mutex};
pub mod signal_1;
// pub mod signal_2;

pub trait PassiveConnection {
    fn standby(&self);
}

pub trait ActiveConnection {
    fn evolve(&self);
}

pub struct InAgentSet<T: Send, A: Send> {
    agent: Arc<Mutex<A>>,
    channel: crossbeam_channel::Receiver<T>,
}

pub struct OutAgentSet<T: Send, A: Send> {
    agent: Arc<Mutex<A>>,
    channel: crossbeam_channel::Sender<T>,
}
