use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crate::random_sleep;
use crate::supervisor::Broadcast;
use crate::connections::RunningPassiveConnection;

// pub mod agent_a;
pub mod agent_b;

pub struct RunningAgent {
    pub instance: JoinHandle<()>,
    pub report: CCReceiver<AgentEvent>,
    pub confirm: CCSender<Broadcast>,
}

impl RunningAgent {
    pub fn new<T>(device: Arc<Mutex<T>>) -> RunningAgent
    where T: 'static + Agent + Send + ?Sized
    {
        // for strict ordering of agent-connection_prop, bounded(1) is chosen.
        let (tx_report, rx_report) = crossbeam_channel::bounded(1);
        let (tx_confirm, rx_confirm) = crossbeam_channel::bounded(1);
        RunningAgent {
            instance: thread::spawn(move || {device.lock().unwrap().run(rx_confirm, tx_report)}),
            report: rx_report,
            confirm: tx_confirm,
        }
    }    
}

pub trait Agent {
    fn running_connections(&self) -> Vec<RunningPassiveConnection>;
    fn end(&mut self);
    fn evolve(&mut self) -> AgentEvent;
    
    fn run(&mut self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<AgentEvent>) {
        let running_connections = self.running_connections();

        loop {
            random_sleep();
            match rx_confirm.recv().unwrap() {

                Broadcast::Exit => {
                    self.end();
                    for r_cn in &running_connections {
                        r_cn.confirm.send(Broadcast::Exit).unwrap();
                    }
                    for r_cn in running_connections {
                        r_cn.instance.join().expect("connection join error!");
                    }
                    break;
                },

                Broadcast::NewCycle => {
                    match self.evolve() {
                        AgentEvent::N => tx_report.send(AgentEvent::N).unwrap(),
                        AgentEvent::Y => {
                            random_sleep();
                            tx_report.send(AgentEvent::Y).unwrap();
                            // println!("agnt waiting pp confirm FinishCycle.");
                            match rx_confirm.recv().unwrap() {
                                Broadcast::FinishCycle => {
                                    for r_cn in &running_connections {
                                        r_cn.confirm.send(Broadcast::FinishCycle).unwrap();
                                    }
                                    // println!("agnt waiting conn report finish Prop.");
                                    for r_cn in &running_connections {
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

#[derive(Debug)]
struct OutConnectionSet<T: Send, C> {
    connection: C,
    channel: CCSender<T>,
}

#[derive(Debug)]
struct InConnectionSet<T: Send, C> {
    connection: C,
    channel: CCReceiver<T>,
}

#[derive(Debug)]
pub enum AgentEvent {
    Y,
    N,
}
