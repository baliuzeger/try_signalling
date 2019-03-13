//use std::sync::{Mutex, Weak};
extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;

pub mod agent_a;

pub trait Agent {
    // fn evolve(&mut self) -> AgentEvent;
    fn run(&mut self);
    // fn enroll(&mut self);
}

struct OutConnectionSet<T: Send, C> {
    connection: C,
    channel: CCSender<T>,
}

struct InConnectionSet<T: Send, C> {
    connection: C,
    channel: CCReceiver<T>,
}

pub enum AgentEvent {
    Y,
    N,
}
