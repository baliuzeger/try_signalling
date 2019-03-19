extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Mutex, Arc};
use std::thread;
use std::collections::HashMap;
use crate::agents::{AgentEvent};
use crate::agent_populations::{RunningPopulation, AgentPopulation};
use crate::connections::PassiveConnection;
use crate::random_sleep;

pub struct Supervisor {
    pub populations: HashMap<String, Arc<Mutex<dyn AgentPopulation + Send>>>,
}

pub enum Broadcast {
    NewCycle,
    FinishCycle,
    Exit,
}

impl Supervisor {
    pub fn add_population<T>(&mut self, key: String, pp: Arc<Mutex<T>>)
    where T: 'static + AgentPopulation + Send
    {
        self.populations.insert(key, pp);
    }
    
    pub fn run(&self, total_steps: u32) {
        // this version make all connections (only passive supported) into threads controlled by pre-agents.
        let mut counter = 0;
        // println!("start making threads for populations.");
        let mut running_populations: Vec<_> = self.running_populations();
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

    fn running_populations(&self) -> Vec<RunningPopulation> {
        self.populations.iter()
            .map(|(_, pp)| RunningPopulation::new(Arc::clone(&pp))).collect()
    }
    
}
