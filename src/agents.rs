use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crate::random_sleep;
use crate::supervisor::{RunMode, Broadcast};
use crate::connections::RunningPassiveConnection;

// pub mod agent_a;
// pub mod agent_b;
pub mod agent_c;

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
    fn config_run(&mut self, mode: RunMode);
    fn config_idle(&mut self);
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

pub enum AgentEvent {
    Y,
    N,
}

pub struct AgentModuleIdle<C: Send> {
    connections: Vec<Weak<Mutex<C>>>
}

impl<C: Send> AgentModuleIdle<C> {
    fn new() -> AgentModuleIdle<C> {
        AgentModuleIdle {
            connections: Vec::new(),
        }
    }

    fn add_connection(&mut self, connection: Weak<Mutex<C>>) {
        self.connections.push(connection);
    }

    fn make_ffw_pre(&self) -> PreAgentModuleFFW<C: Send, S: Send> {
        PreAgentModuleFFW {
            connections: self.connections.iter().map(|conn| OutSetFFW {
                connection: Arc::downgrade(conn.upgrade().expect("no object in Weak<connection>!")),
                channel: match conn.mode() {
                    RunMode::Feedforward -> None,
                    RunMode::Feedforward -> {
                        let (tx, rx) = crossbeam_channel::bounded(1);
                        conn.set_pre_channel(Some(rx));
                        Some(tx)
                    },
                }
            }).collect(),
        }
    }

    fn make_ffw_post(&self) -> PostAgentModuleFFW<C: Send, S: Send> {
        PostAgentModuleFFW {
            connections: self.connections.iter().map(|conn| InSetFFW {
                connection: Arc::downgrade(conn.upgrade().expect("no object in Weak<connection>!")),
                channel: match conn.mode() {
                    RunMode::Feedforward -> None,
                    RunMode::Feedforward -> {
                        let (tx, rx) = crossbeam_channel::bounded(1);
                        conn.set_post_channel(Some(tx));
                        Some(rx)
                    },
                }
            }).collect(),
            buffer: Vec::new(),
        }
    }
}

pub struct PreAgentModuleFFW<C: Send, S: Send> {
    connections: Vec<OutSetFFW<C, S>>,
}

impl<C: Send, S: Send> PreAgentModuleFFW<C, S> {
    fn feedforward(&self, s: S) {
        for conn in self.connections {
            match &conn.channel {
                None => (),
                Some(tx) => tx.send(s),
            }
        }
    }
}

pub struct PostAgentModuleFFW<C: Send> {
    connections: Vec<InSetFFW<C, R>>,
}

impl<C: Send, R: Send> PostAgentModuleFFW<C, R> {
    fn accepted(&self) -> Vec<R> {
        connections.iter()
            .filter_map(|conn| {
                match conn.channel {
                    None => None,
                    Some => Some(rx.try_iter()),
                }
            }).flatten().collect();
    }

    fn store(&mut self) {
        for conn in connections {
            match &conn.channel {
                None => (),
                Some(rx) => self.buffer.append(rx.try_iter().collectr()),
            }
        }
    }
}

struct OutSetFFW<C: Send, S: Send> {
    connection: Weak<Mutex<C>>,
    channel: Option<CCSender<S>>,
}

struct InSetFFW<C: Send, R: Send> {
    connection: Weak<Mutex<C>>,
    channel: Option<CCReceiver<R>>,
}
