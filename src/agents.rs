use std::sync::{Mutex, Arc, Weak};
extern crate crossbeam_channel;
use crate::supervisor;

pub mod agent_a;

pub trait Agent {
    fn evolve(&mut self);
    fn enroll(&mut self);
}

struct OutChannelSet<T> {
    connection: Weak<Mutex<dyn Propagate1 + Send>>,
    channel: crossbeam_channel::Sender<T>,
    // sync: crossbeam_channel::Receiver<bool>,
}

struct InChannelSet {
    connection: Weak<Mutex<dyn Propagate1 + Send>>,
    channel: crossbeam_channel::Receiver<T>,
}

// struct PortsToSuper {
//     report: crossbeam_channel::Sender<bool>,
//     confirm: crossbeam_channel::Receiver<supervisor::Broadcast>,
// }
