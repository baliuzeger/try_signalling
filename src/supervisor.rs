extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Mutex, Arc};
use std::thread;
use crate::agents::{Agent, AgentEvent};
use crate::signals::PassiveConnection;
use crate::signals::signal_1::{Generate1, Process1, Connection1};

pub struct Supervisor {
    agents: Vec<Arc<Mutex<dyn Agent>>>,
    passive_connections:Vec<Arc<Mutex<dyn PassiveConnection>>>,
}

struct RunningSet {
    instance: thread::JoinHandle<()>,
    report: CCReceiver<bool>,
    confirm: CCSender<Broadcast>,
}

pub enum Broadcast {
    Continue,
    End,
}

impl Supervisor {
    pub fn add_agent(&mut self, agnt: Arc<Mutex<dyn Agent>>) {
        self.agents.push(agnt);
    }

    pub fn add_passive_connection<S, R>(&mut self, cn: Arc<Mutex<Connection1<S, R>>>)
    where S: 'static + Generate1 + Send,
          R: 'static + Process1 + Send
    {
        self.passive_connections.push(cn);
    }

    pub fn run(&self, total_steps: u32) {
        // this version make all connections (only passive supported) into threads controlled by pre-agents.
        let mut counter = 0;
        let mut running_agents = Vec::new();

        for agnt in self.agents {
            let (tx_agnt_report, rx_agnt_report) = crossbeam_channel::bounded(1);
            let (tx_agnt_confirm, rx_agnt_confirm) = crossbeam_channel::bounded(1);
            let mut running_connections = Vec::new();

            for conn in agnt.lock().unwrap().out_connections_1 {
                let (tx_conn_report, rx_conn_report) = crossbeam_channel::bounded(1);
                let (tx_conn_confirm, rx_conn_confirm) = crossbeam_channel::bounded(1);

                running_connections.push(RunningSet {
                    instance: thread::spawn(move || {
                        loop {
                            match rx_conn_confirm.recv().unwrap() {
                                Broadcast::End => {break},
                                Broadcast::Continue => {
                                    conn.lock().unwrap().standby();
                                    tx_conn_report.send(true).unwrap();                                    
                                },
                            }
                        }
                    }),
                    report: rx_conn_report,
                    confirm: tx_conn_confirm,
                })
            }

            running_agents.push(RunningSet {
                instance: thread::spawn(move || {
                    loop {
                        match rx_agnt_confirm.recv().unwrap() {
                            Broadcast::End => {
                                for r_cn in running_connections {
                                    r_cn.confirm.send(Broadcast::End).unwrap();
                                }
                                for r_cn in running_connections {
                                    r_cn.instance.join().expect("connection join error!");
                                }
                                break;
                            },
                            Broadcast::Continue => {
                                if let AgentEvent::Y = agnt.lock().unwrap().evolve() {
                                    for r_cn in running_connections {
                                        r_cn.confirm.send(Broadcast::Continue).unwrap();
                                    }
                                    for r_cn in running_connections {
                                        r_cn.report.recv().unwrap();
                                    }                                    
                                }
                                tx_agnt_report.send(true).unwrap();
                            },
                        }
                    }
                }),
                report: rx_agnt_report,
                confirm: tx_agnt_confirm,
            });
        }

        loop {
            if counter >= total_steps {
                for r_agnt in running_agents {
                    r_agnt.confirm.send(Broadcast::End).unwrap();
                }
                for r_agnt in running_agents {
                    r_agnt.instance.join().expect("agent join error!");
                }
                break;
            } else  {
                for r_agnt in running_agents {
                    r_agnt.confirm.send(Broadcast::Continue).unwrap();
                }
                for r_agnt in running_agents {
                    r_agnt.report.recv().unwrap();
                }
                counter += 1;
            }
        }
    }
}
