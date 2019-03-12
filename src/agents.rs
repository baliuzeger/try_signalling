//use std::sync::{Mutex, Weak};
extern crate crossbeam_channel;

pub mod agent_a;

pub trait Agent {
    fn evolve(&mut self) -> AgentEvent;
    // fn enroll(&mut self);
}

struct OutConnectionSet<T: Send, C> {
    connection: C,
    channel: crossbeam_channel::Sender<T>,
}

struct InConnectionSet<T: Send, C> {
    connection: C,
    channel: crossbeam_channel::Receiver<T>,
}

pub enum AgentEvent {
    Y,
    N,
}
