extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crate::supervisor::{RunMode ,Broadcast};


pub enum Fired {
    Y,
    N,
}

/// used by Supervisor.config()
pub trait Device {
    fn config_run(&mut self, mode: RunMode);
    fn config_idle(&mut self);
}


pub trait ActivePopulation: Device {
    fn running_devices(&self) -> Vec<RunningDevice<(), Broadcast>>;

    fn run(&mut self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<AgentEvent>) {
        // this version make all connections (only passive supported) into threads controlled by pre-agents.
        let running_agents = self.running_agents();

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

/// used by Population.runningdevices()
pub trait ActiveDevice: Device {
    fn running_passive_devices(&self) -> Vec<RunningDevice<(), Broadcast>>;
    fn end(&mut self);
    fn evolve(&mut self) -> AgentEvent;

    // fn run_f(&mut self) -> Box<dyn FnMut(CCReceiver<Broadcast>, CCSender<AgentEvent>)> {
    //     Box::new(|rx_confirm, tx_report| self.run(rx_confirm, tx_report))
    // }
    
    fn run(&mut self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<AgentEvent>) {
        let running_devices = self.running_passive_devices();

        loop {
            random_sleep();
            match rx_confirm.recv().unwrap() {

                Broadcast::Exit => {
                    self.end();
                    for r_cn in &running_devices {
                        r_cn.confirm.send(Broadcast::Exit).unwrap();
                    }
                    for r_cn in running_devices {
                        r_cn.instance.join().expect("connection join error!");
                    }
                    break;
                },

                Broadcast::NewCycle => {
                    // for r_cn in &running_active_connections {
                    //     r_cn.confirm.send(Broadcast::NewCycle).unwrap();
                    // }
                    match self.evolve() {
                        AgentEvent::N => tx_report.send(AgentEvent::N).unwrap(),
                        AgentEvent::Y => {
                            random_sleep();
                            tx_report.send(AgentEvent::Y).unwrap();
                            // println!("agnt waiting pp confirm FinishCycle.");
                            match rx_confirm.recv().unwrap() {
                                Broadcast::FinishCycle => {
                                    for r_cn in &running_devices {
                                        r_cn.confirm.send(Broadcast::FinishCycle).unwrap();
                                    }
                                    // println!("agnt waiting conn report finish Prop.");
                                    for r_cn in &running_devices {
                                        r_cn.report.recv().unwrap();
                                    }
                                    // println!("agnt get conn report finish Prop.");
                                    tx_report.send(AgentEvent::N).unwrap();
                                },
                                _ => panic!("sp not confirm by FinishCycle before finish cycle!"),
                            }
                        }
                    }
                },

                _ => panic!("agent should only get Exit or NewCycle at beginning of cycle!")
            }
        }
    }    
}

/// used by Components.runningdevices()
pub trait PassiveDevice: Device {
    fn mode(&self) -> RunMode;
    fn respond(&self);

    // fn run_f(&mut self) -> Box<dyn FnMut(CCReceiver<Broadcast>, CCSender<()>)> {
    //     Box::new(|rx_confirm, tx_report| self.run(rx_confirm, tx_report))
    // }

    fn run(&self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<()>){
        loop {
            random_sleep();
            match rx_confirm.recv().unwrap() {
                Broadcast::Exit => break,
                Broadcast::NewCycle => panic!("Passivedevice confirmed by NewCycle!"),
                Broadcast::FinishCycle => {
                    // println!("conn wait recv signal.");
                    self.respond();
                    // println!("conn got & propagated signal.");
                    tx_report.send(()).unwrap();
                }
            }
        }
    }    
}

pub struct RunningDevice<C: Send, R: Send> {
    pub instance: JoinHandle<()>,
    pub confirm: CCSender<C>,
    pub report: CCReceiver<R>,
}

impl<C: Send, R: Send> RunningDevice<C, R> {
    pub fn new_active<T>(device: Arc<Mutex<T>>) -> RunningDevice<C, R>
    where T: 'static + ActiveDevice + Send + ?Sized
    {
        // for strict ordering of agent-connection_prop, bounded(1) is chosen.
        let (tx_confirm, rx_confirm) = crossbeam_channel::bounded(1);
        let (tx_report, rx_report) = crossbeam_channel::bounded(1);
        RunningDevice {
            instance: thread::spawn(move || {device.lock().unwrap().run(rx_confirm, tx_report)}),
            confirm: tx_confirm,
            report: rx_report,
        }
    }

    pub fn new_passive<T>(device: Arc<Mutex<T>>) -> RunningDevice<C, R>
    where T: 'static + PassiveDevice + Send + ?Sized
    {
        // for strict ordering of agent-connection_prop, bounded(1) is chosen.
        let (tx_confirm, rx_confirm) = crossbeam_channel::bounded(1);
        let (tx_report, rx_report) = crossbeam_channel::bounded(1);
        RunningDevice {
            instance: thread::spawn(move || {device.lock().unwrap().run(rx_confirm, tx_report)}),
            report: rx_report,
            confirm: tx_confirm,
        }
    }

    // pub fn new<C, R>(f: Box<dyn FnMut(CCReceiver<C>, CCSender<R>)>) -> RunningDevice<C, R>
    // {
    //     // for strict ordering of agent-connection_prop, bounded(1) is chosen.
    //     let (tx_confirm, rx_confirm) = crossbeam_channel::bounded(1);
    //     let (tx_report, rx_report) = crossbeam_channel::bounded(1);
    //     RunningDevice {
    //         instance: thread::spawn(move || {*f(rx_confirm, tx_report)}),
    //         confirm: tx_confirm,
    //         report: rx_report,
    //     }
    // }    
}
