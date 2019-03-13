/// functionality needed:
/// 1. a channel should not connect from/to an identical agent.
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Arc, Mutex};
use crate::supervisor::Broadcast;
pub mod signal_1;
// pub mod signal_2;

pub trait PassiveConnection {
    fn run_under_agent(&self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<bool>);
}

pub trait ActiveConnection {
    fn evolve(&self);
}

pub struct InAgentSet<T: Send, A: Send> {
    agent: Arc<Mutex<A>>,
    channel: CCReceiver<T>,
}

pub struct OutAgentSet<T: Send, A: Send> {
    agent: Arc<Mutex<A>>,
    channel: CCSender<T>,
}
