extern crate crossbeam_channel;
use std::sync::{Mutex, Arc, Weak};

pub struct Supervisor {
    agents: Vec<DeviceSet<Arc<Mutex<dyn Agent>>>>,
    passive_connections:Vec<Arc<Mutex<dyn PassiveConnection>>>,
}

struct DeviceSet<T> {
    device: T,
    report: crossbeam_channel::Receiver<bool>,
    confirm: crossbeam_channel::Sender<Broadcast>,
}

pub enum Broadcast {
    Continue,
    End,
}

impl Supervisor {
    pub fn add_agent(&mut self, device: Arc<Mutex<dyn Agent>>) {
        let (tx_report, rx_report) = crossbeam_channel::bounded(0);
        let (tx_confirm, rx_confirm) = crossbeam_channel::bounded(0);
        device.lock().unwrap().enroll(tx_report, rx_confirm);
        self.agents.push(
            DeviceSet {
                device,
                report: rx_report,
                confirm: tx_confirm,
            }
        );
    }

    pub fn add_passive_connection(&mut self, cn: Arc<Mutex<Connection1>>) {
        self.passive_connections.push(cn);
    }

    
}
