/// functionality needed:
/// 1. a channel should not connect from/to an identical agent.
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use crate::random_sleep;
use crate::supervisor::Broadcast;
// pub mod signal_1;
pub mod signal_2;

pub struct RunningPassiveConnection {
    pub instance: JoinHandle<()>,
    pub report: CCReceiver<bool>,
    pub confirm: CCSender<Broadcast>,
}

impl RunningPassiveConnection {
    pub fn new<T>(device: Arc<Mutex<T>>) -> RunningPassiveConnection
    where T: 'static + PassiveConnection + Send + ?Sized
    {
        // for strict ordering of agent-connection_prop, bounded(1) is chosen.
        let (tx_report, rx_report) = crossbeam_channel::bounded(1);
        let (tx_confirm, rx_confirm) = crossbeam_channel::bounded(1);
        RunningPassiveConnection {
            instance: thread::spawn(move || {device.lock().unwrap().run(rx_confirm, tx_report)}),
            report: rx_report,
            confirm: tx_confirm,
        }
    }    
}

pub trait PassiveConnection {
    fn propagate(&self);
    
    fn run(&self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<bool>){
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

// pub trait ActiveConnection {
//     fn evolve(&self);
// }

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
