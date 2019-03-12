use std::sync::{Mutex, Arc, Weak};
extern crate crossbeam_channel;
use crate::supervisor;

pub mod agent_a;

pub trait Agent {
    fn evolve(&mut self);
    fn enroll(&mut self);
}

struct OutChannelSet<T: Send, C: Send> {
    connection: Weak<Mutex<C>>,
    channel: crossbeam_channel::Sender<T>,
}

struct InChannelSet<T: Send, C: Send> {
    connection: Weak<Mutex<C>>,
    channel: crossbeam_channel::Receiver<T>,
}

pub enum AgentEvent {
    Y,
    N,
}
