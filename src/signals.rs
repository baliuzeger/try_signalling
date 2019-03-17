/// functionality needed:
/// 1. a channel should not connect from/to an identical agent.
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Arc, Mutex};
use crate::supervisor::Broadcast;
pub mod signal_1;
pub mod signal_2;

pub trait PassiveConnection {
    fn propogate(&self);
    
    fn run(&self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<bool>);

    fn run_under_agent(&self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<bool>){
        loop {
            random_sleep();
            match rx_confirm.recv().unwrap() {
                Broadcast::Exit => break,
                Broadcast::NewCycle => panic!("agent confirm by NewCycle!"),
                Broadcast::FinishCycle => {
                    // println!("conn wait recv signal.");
                    self.propagate();
                    // println!("conn got & propagated signal.");
                    tx_report.send(true).unwrap();
                }
            }
        }
    }
}

pub trait ActiveConnection {
    fn evolve(&self);
}

#[derive(Debug)]
pub struct InAgentSet<T: Send, A: Send> {
    agent: Arc<Mutex<A>>,
    channel: CCReceiver<T>,
}

#[derive(Debug)]
pub struct OutAgentSet<T: Send, A: Send> {
    agent: Arc<Mutex<A>>,
    channel: CCSender<T>,
}
