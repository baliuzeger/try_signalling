extern crate crossbeam_channel;
use std::sync::{Mutex, Arc, Weak};
use std::thread;

pub struct Supervisor {
    agents: Vec<Arc<Mutex<dyn Agent>>>,
    passive_connections:Vec<Arc<Mutex<dyn PassiveConnection>>>,
}

struct RunningSet<T> {
    instance: thread::JoinHandle<()>,
    report: crossbeam_channel::Receiver<bool>,
    confirm: crossbeam_channel::Sender<Broadcast>,
}

pub enum Broadcast {
    Continue,
    End,
}

impl Supervisor {
    pub fn add_agent(&mut self, device: Arc<Mutex<dyn Agent>>) {
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

    pub fn run(&self, total_steps: u32) {
        // this version make all connections (only passive supported) into threads controlled by pre-agents.
        let mut counter = 0;
        let mut running_agents = Vec::new();
        let mut running_connections = Vec::new();
        for agnt in self.agents {
            let (tx_report, rx_report) = crossbeam_channel::bounded(0);
            let (tx_confirm, rx_confirm) = crossbeam_channel::bounded(0);
            running_agents.push(RunningDeviceSet {
                instance: thread::spawn(move || {
                    loop {
                        agnt.device.lock().unwrap().evolve();
                        tx_report.send(true).unwrap();
                        if let Broadcast::End = rx_confirm.recv().unwrap() {
                            break;
                        }
                    }
                }),
                report: rx_report,
                confirm: tx_confirm,
            });
        }
        for p_conn in self.passive_connections {
            // threads of connections should be initiated by agents!
            running_connections.push(thread::spawn(move || {
                
            }))
        }
    }
}
