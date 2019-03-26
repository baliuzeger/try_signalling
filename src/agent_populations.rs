use std::thread;
use std::thread::JoinHandle;
use std::sync::{Mutex, Arc};
extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crate::supervisor::{Broadcast, RunMode. DeviceMode};
use crate::random_sleep;
use crate::agents::{AgentEvent, RunningAgent, Agent};

pub struct RunningPopulation {
    pub instance: JoinHandle<()>,
    pub report: CCReceiver<AgentEvent>,
    pub confirm: CCSender<Broadcast>,
}

impl RunningPopulation {
    pub fn new<T>(device: Arc<Mutex<T>>, mode: RunMode) -> RunningPopulation
    where T: 'static + AgentPopulation + Send + ?Sized
    {
        // for strict ordering of agent-connection_prop, bounded(1) is chosen.
        let (tx_report, rx_report) = crossbeam_channel::bounded(1);
        let (tx_confirm, rx_confirm) = crossbeam_channel::bounded(1);
        RunningPopulation {
            instance: thread::spawn(move || {device.lock().unwrap().run(rx_confirm, tx_report, mode)}),
            report: rx_report,
            confirm: tx_confirm,
        }
    }    
}

pub trait AgentPopulation {
    fn running_agents(&self) -> Vec<RunningAgent>;

    fn run(&mut self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<AgentEvent>, mode: RunMode) {
        // this version make all connections (only passive supported) into threads controlled by pre-agents.
        let running_agents = self.running_agents(mode);

        let mut agents_with_event = Vec::new();
        loop {
            random_sleep();
            match rx_confirm.recv().unwrap() {

                Broadcast::Exit => {
                    for r_agnt in &running_agents {
                        r_agnt.confirm.send(Broadcast::Exit).unwrap();
                    }
                    for r_agnt in running_agents {
                        r_agnt.instance.join().expect("connection join error!");
                    }
                    break;
                },

                Broadcast::NewCycle => {
                    agents_with_event.clear();
                    for r_agnt in &running_agents {
                        r_agnt.confirm.send(Broadcast::NewCycle).unwrap();
                    }
                    for r_agnt in &running_agents {
                        if let AgentEvent::Y = r_agnt.report.recv().unwrap() {
                            agents_with_event.push((r_agnt.confirm.clone(), r_agnt.report.clone()));
                        }
                    }

                    match agents_with_event.len() {
                        0 => tx_report.send(AgentEvent::N).unwrap(),
                        _ => {
                            random_sleep();
                            tx_report.send(AgentEvent::Y).unwrap();
                            // println!("pp waiting sp confirm to Finishcycle.");
                            match rx_confirm.recv().unwrap() {
                                Broadcast::FinishCycle => {
                                    for agnt_e in &agents_with_event {
                                        agnt_e.0.send(Broadcast::FinishCycle).unwrap();
                                    }
                                    // println!("pp waiting agnt report FinishCycle.");
                                    for agnt_e in &agents_with_event {
                                        match agnt_e.1.recv().unwrap() {
                                            AgentEvent::N => (),
                                            AgentEvent::Y => panic!("agnt report Event after FinishCycle!")
                                        }
                                    }
                                    // println!("pp get report from agnt of FinishCycle.")
                                },
                                _ => panic!("sp not confirm by FinishCycle before finish cycle!"),
                            }
                            tx_report.send(AgentEvent::N).unwrap();
                        }
                    }
                },

                _ => panic!("pp should only recv confirm of NewCycle or Exit!")
            }
        }
    }    
    
}

pub struct SimplePopulation<T: Agent> {
    agents: Vec<Arc<Mutex<T>>>,
}

impl<T: 'static + Agent + Send> AgentPopulation for SimplePopulation<T> {
    fn running_agents(&self, mode: RunMode) -> Vec<RunningAgent> {
        self.agents.iter().map(|agnt| RunningAgent::new(Arc::clone(&agnt), mode)).collect()

        // for agnt in &self.agents {
        //     let (tx_agnt_report, rx_agnt_report) = crossbeam_channel::bounded(1);
        //     let (tx_agnt_confirm, rx_agnt_confirm) = crossbeam_channel::bounded(1);
        //     let running_agnt = Arc::clone(agnt);
        //     running_agents.push(RunningSet {
        //         instance: thread::spawn(move || {running_agnt.lock().unwrap().run(rx_agnt_confirm, tx_agnt_report)}),
        //         report: rx_agnt_report,
        //         confirm: tx_agnt_confirm,
        //     });
        // }
    }
}

impl<T: 'static + Agent + Send>  SimplePopulation<T> {
    pub fn new() -> Arc<Mutex<SimplePopulation<T>>> {
        Arc::new(Mutex::new(SimplePopulation{
            agents: Vec::new(),
        }))
    }

    pub fn add_agent(&mut self, agnt: Arc<Mutex<T>>) {
        self.agents.push(agnt);
    }

    
    pub fn agent_by_id(&self, n: usize) -> Arc<Mutex<T>> {
        Arc::clone(&self.agents[n])
    }    
}
