extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Mutex, Arc};
use std::thread;
use std::collections::HashMap;
use crate::agents::{AgentPopulation};
use crate::signals::PassiveConnection;

pub struct Supervisor {
    pub populations: HashMap<String, Arc<Mutex<dyn AgentPopulation + Send>>>,
    pub passive_connections:Vec<Arc<Mutex<dyn PassiveConnection>>>,
}

pub struct RunningSet {
    pub instance: thread::JoinHandle<()>,
    pub report: CCReceiver<bool>,
    pub confirm: CCSender<Broadcast>,
}

pub enum Broadcast {
    Continue,
    End,
}

impl Supervisor {
    // pub fn add_passive_connection<S, R>(&mut self, cn: Arc<Mutex<Connection1<S, R>>>)
    // where S: 'static + Generate1 + Send,
    //       R: 'static + Process1 + Send
    // {
    //     self.passive_connections.push(cn);
    // }

    pub fn add_passive_connection<T>(&mut self, cn: Arc<Mutex<T>>)
    where T: 'static + PassiveConnection + Send,
    {
        self.passive_connections.push(cn);
    }

    pub fn add_population<T>(&mut self, key: String, pp: Arc<Mutex<T>>)
    where T: 'static + AgentPopulation + Send
    {
        self.populations.insert(key, pp);
    }
    
    pub fn run(&self, total_steps: u32) {
        // this version make all connections (only passive supported) into threads controlled by pre-agents.
        let mut counter = 0;
        let mut running_populations = Vec::new();

        for (_, pp) in &self.populations {
            let (tx_pp_report, rx_pp_report) = crossbeam_channel::bounded(1);
            let (tx_pp_confirm, rx_pp_confirm) = crossbeam_channel::bounded(1);
            let running_pp = Arc::clone(&pp);
            running_populations.push(RunningSet {
                instance: thread::spawn(move || {running_pp.lock().unwrap().run(rx_pp_confirm, tx_pp_report)}),
                report: rx_pp_report,
                confirm: tx_pp_confirm,
            });
        }

        loop {
            if counter >= total_steps {
                for r_pp in &running_populations {
                    r_pp.confirm.send(Broadcast::End).unwrap();
                }
                for r_pp in running_populations {
                    r_pp.instance.join().expect("population join error!");
                }
                break;
            } else  {
                for r_pp in &running_populations {
                    r_pp.confirm.send(Broadcast::Continue).unwrap();
                }
                for r_pp in &running_populations {
                    r_pp.report.recv().unwrap();
                }
                counter += 1;
            }
        }
    }
}
