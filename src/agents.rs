use std::sync::{Arc, Mutex};
extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crate::supervisor::Broadcast;

pub mod agent_a;

pub trait Agent {
    // fn evolve(&mut self) -> AgentEvent;
    fn run(&mut self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<bool>);
    // fn enroll(&mut self);
}

pub trait AgentPopulation {
    fn run(&mut self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<bool>);
}

#[derive(Debug)]
struct OutConnectionSet<T: Send, C> {
    connection: C,
    channel: CCSender<T>,
}

#[derive(Debug)]
struct InConnectionSet<T: Send, C> {
    connection: C,
    channel: CCReceiver<T>,
}

#[derive(Debug)]
pub enum AgentEvent {
    Y,
    N,
}
