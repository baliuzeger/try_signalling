use crate::supervisor::Broadcast;
use crate::random_sleep();

pub struct RunningPopulation {
    pub instance: JoinHandle<()>,
    pub report: CCReceiver<AgentEvent>,
    pub confirm: CCSender<BroadCast>,
}

impl RunningPoulation {
    fn new<T>(device: Arc<Mutex<T>>) -> RunningPopulation
    where T: 'static + AgentPopulation + Send + ?Sized
    {
        // for strict ordering of agent-connection_prop, bounded(1) is chosen.
        let (tx_report, rx_report) = crossbeam_channel::bounded(1);
        let (tx_confirm, rx_confirm) = crossbeam_channel::bounded(1);
        RunningSet {
            instance: thread::spawn(move || {device.lock().unwrap().run(rx_confirm, tx_report)}),
            report: rx_report,
            confirm: tx_confirm,
        }
    }    
}

pub trait AgentPopulation {
    fn running_agents(&self) -> Vec<RuuningPopulation<Broadcast, AgentEvent>>;

    fn run(&mut self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<AgentEvent>) {
        // this version make all connections (only passive supported) into threads controlled by pre-agents.
        let mut running_agents = self.running_agents();

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

impl<T: Agent> AgentPopulation for SimplePopulation<T> {
    fn ruuning_agents(&self) -> Vec<RuuningPopulation> {
        self.agents.iter().map(|agnt| RuningAgent::new(Arc::clone(&agnt))).collect();

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


