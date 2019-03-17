// use std::sync::{Arc, Mutex};
extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crate::supervisor::Broadcast;

pub mod agent_a;
pub mod agent_b;

pub trait Agent {
    fn running_connections(&self) -> Vec<>;
    fn end(&mut self);
    fn evolve(&mut self);
    
    fn run(&mut self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<AgentEvent>) {
        let mut running_connections = running_connections();

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

pub trait AgentPopulation {
    
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
