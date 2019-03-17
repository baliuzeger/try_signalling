extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Mutex, Arc};
use std::thread;
use std::collections::HashMap;
use crate::agents::{AgentPopulation, AgentEvent};
use crate::signals::PassiveConnection;
use crate::random_sleep;

pub struct Supervisor {
    pub populations: HashMap<String, Arc<Mutex<dyn AgentPopulation + Send>>>,
    pub passive_connections:Vec<Arc<Mutex<dyn PassiveConnection>>>,
}

pub enum Broadcast {
    NewCycle,
    FinishCycle,
    Exit,
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
        // println!("start making threads for populations.");
        for (_, pp) in &self.populations {
            let (tx_pp_report, rx_pp_report) = crossbeam_channel::bounded(1);
            let (tx_pp_confirm, rx_pp_confirm) = crossbeam_channel::bounded(1);
            let running_pp = Arc::clone(&pp);
            // println!("making threads for populations.");
            running_populations.push(RunningSet {
                instance: thread::spawn(move || {running_pp.lock().unwrap().run(rx_pp_confirm, tx_pp_report)}),
                report: rx_pp_report,
                confirm: tx_pp_confirm,
            });
        }

        let mut populations_with_event = Vec::new();
        loop {
            if counter >= total_steps {
                for r_pp in &running_populations {
                    r_pp.confirm.send(Broadcast::Exit).unwrap();
                }
                for r_pp in running_populations {
                    r_pp.instance.join().expect("population join error!");
                }
                break;
            } else  {
                random_sleep();
                // println!("count: {}.", counter);
                populations_with_event.clear();
                for r_pp in &running_populations {
                    r_pp.confirm.send(Broadcast::NewCycle).unwrap();
                }
                for r_pp in &running_populations {
                    if let AgentEvent::Y = r_pp.report.recv().unwrap() {
                        populations_with_event.push((r_pp.confirm.clone(), r_pp.report.clone()));
                    }
                }
                for pp_e in &populations_with_event {
                    pp_e.0.send(Broadcast::FinishCycle).unwrap();
                }
                // println!("sp waiting pp FinishCycle.");
                for pp_e in &populations_with_event {
                    match pp_e.1.recv().unwrap() {
                        AgentEvent::N => (),
                        AgentEvent::Y => panic!("pp report Event after FinishCycle!")
                    }
                }
                // println!("sp get pp report FinishCycle.");
                counter += 1;
            }
        }
    }

    fn run_population(&self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<AgentEvent>) {
        // this version make all connections (only passive supported) into threads controlled by pre-agents.
        let mut running_agents = 
        
    }
}
