extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crate::supervisor::{RunMode ,Broadcast};


pub enum Fired {
    Y,
    N,
}

pub trait Device {
    fn config_run(&mut self, mode: RunMode);
    fn config_idle(&mut self);
}

pub trait ActiveDevice {
    fn running_passive_devices(&self) -> Vec<RunningDevice<(), Broadcast>>;
    fn end(&mut self);
    fn evolve(&mut self) -> AgentEvent;

    // // supervisor holds <dyn Active/PassiveDevice>, so conflict type safety.
    // fn run_f<F: FnMut>(&mut self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<AgentEvent>) -> F {
    //     || self.run(rx_confirm, tx_report)
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

pub trait PassiveDevice {
    fn respond(&self);
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

pub struct RunningDevice<R: Send, C: Send> {
    pub instance: JoinHandle<()>,
    pub report: CCReceiver<R>,
    pub confirm: CCSender<C>,
}

impl<R: Send, C: Send> RunningDevice<R, C> {
    // pub fn new(f: FnMut) -> RunningDevice<R, C>
    // {
    //     // for strict ordering of agent-connection_prop, bounded(1) is chosen.
    //     let (tx_report, rx_report) = crossbeam_channel::bounded(1);
    //     let (tx_confirm, rx_confirm) = crossbeam_channel::bounded(1);
    //     RunningDevice {
    //         instance: thread::spawn(move f),
    //         report: rx_report,
    //         confirm: tx_confirm,
    //     }
    // }

    pub fn new_active<T>(device: Arc<Mutex<T>>) -> RunningDevice<R, C>
    where T: 'static + ActiveDevice + Send + ?Sized
    {
        // for strict ordering of agent-connection_prop, bounded(1) is chosen.
        let (tx_report, rx_report) = crossbeam_channel::bounded(1);
        let (tx_confirm, rx_confirm) = crossbeam_channel::bounded(1);
        RunningDevice {
            instance: thread::spawn(move || {device.lock().unwrap().run(rx_confirm, tx_report)}),
            report: rx_report,
            confirm: tx_confirm,
        }
    }

    pub fn new_passive<T>(device: Arc<Mutex<T>>) -> RunningDevice<R, C>
    where T: 'static + PassiveDevice + Send + ?Sized
    {
        // for strict ordering of agent-connection_prop, bounded(1) is chosen.
        let (tx_report, rx_report) = crossbeam_channel::bounded(1);
        let (tx_confirm, rx_confirm) = crossbeam_channel::bounded(1);
        RunningDevice {
            instance: thread::spawn(move || {device.lock().unwrap().run(rx_confirm, tx_report)}),
            report: rx_report,
            confirm: tx_confirm,
        }
    }    
}
