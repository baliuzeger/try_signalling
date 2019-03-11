extern crate crossbeam_channel;
use crate::supervisor;

pub mod agent_a;

pub trait Agent {
    fn evolve(&mut self);
    fn enroll(&mut self);
}

struct ExportPair<T> {
    sgnl: crossbeam_channel::Sender<T>,
    sync: crossbeam_channel::Receiver<bool>,
}

struct PortsToSuper {
    report: crossbeam_channel::Sender<bool>,
    confirm: crossbeam_channel::Receiver<supervisor::Broadcast>,
}
