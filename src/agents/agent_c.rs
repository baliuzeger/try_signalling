extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Mutex, Arc, Weak};
use crate::connections::{RunningPassiveConnection};
use crate::connections::signal_1::{PreAgentModuleS1, PostAgentModuleS1, S1Generator, S1Propagator, S1Acceptor};
use crate::connections::signal_1::{FwdPreS1, FwdPostS1};
use crate::agents::{Agent, OutConnectionSet, InConnectionSet, AgentEvent};
use crate::supervisor::RunMode;

pub struct Model {
    gen_value: i32,
    proc_value: i32,
    pre_module_s1: PreAgentModuleS1,
    post_module_s1: PostAgentModuleS1,
    event_cond: Option<i32>,
}

impl S1Generator for Model {
    fn generate_s1(&self) {
        self.pre_module_s1.feedforward(FwdPreS1 {
            msg_gen: self.gen_value,
        });
    }

    fn add_out_passive_s1<T> (&mut self, connection: Weak<Mutex<T>>)
        where T: 'static + S1PassivePropagator + Send
    {
        self.pre_module_s1.add_connection(connection);
        
    }
}

impl S1Acceptor for Model {
    fn accept_s1(&mut self) {
        self.post_module_s1.store();
    }

    fn add_in_s1<T>(&mut self, connection: Weak<Mutex<T>>) {
        self.post_module_s1.add_connection(connection);
    }
}

impl Agent for Model {
    fn config_run(&mut self, mode: RunMode<bool, bool>) {
        match mode {
            RunMode::Idle(_) => println!("config_run for mode Idle, no effect."),
            RunMode::Feedforward(_) => {
                self.pre_module_s1.config_run(mode);
                self.post_module_s1.config_run(mode);
            },
        }
        
    }

    fn config_idle(&mut self) {
        
    }

    fn running_connections(&self) -> Vec<RunningPassiveConnection> {
        self.out_connections_1.iter().map(|cn| RunningPassiveConnection::new(cn.connection.upgrade().unwrap())).collect()
    }
    
    fn end(&mut self) {
        self.store();
    }
    
    fn evolve(&mut self) -> AgentEvent {
        self.store();
        self.proc_value += 1;
        self.gen_value += 1;
        match self.event_cond {
            None => {
                // println!("agnet a go on. gen: {}, proc: {}.",  self.gen_value, self.proc_value);
                AgentEvent::N   
            },
            Some(n) => {
                match self.proc_value % n {
                    0 => {
                        // println!("agnet a fire. gen: {}, proc: {}.",  self.gen_value, self.proc_value);
                        self.send_count();
                        AgentEvent::Y
                    },
                    _ => {
                        // println!("agnet a go on. gen: {}, proc: {}.",  self.gen_value, self.proc_value);
                        AgentEvent::N
                    },
                }
            }
        }
    }
}

impl Model {
    pub fn new(gen_value: i32, proc_value: i32, event_cond: Option<i32>) -> Arc<Mutex<Model>> {
        Arc::new(Mutex::new(
            Model{
                gen_value,
                proc_value,
                buffer_1: Vec::new(),
                out_connections_1: Vec::new(),
                in_connections_1: Vec::new(),
                event_cond,
            }
        ))
    }
    
    fn store(&mut self) {
        for conn in &self.in_connections_1 {
            match conn.channel.try_recv() {
                Ok(s) => {
                    self.buffer_1.push(self.process_1(s))
                },
                Err(crossbeam_channel::TryRecvError::Disconnected) => panic!("Sender is gone!"),
                Err(crossbeam_channel::TryRecvError::Empty) => (),
            }
        }
    }
    
    pub fn send_count(&mut self) {
        for conn in &self.out_connections_1 {
            conn.channel.send(self.generate_1()).unwrap();
        }
        // self.gen_value += 1;
    }

    pub fn print_values(&self) {
        println!("gen: {}, proc: {}.", self.gen_value, self.proc_value);
    }
    
    pub fn show_1(&self) {
        for msg in &self.buffer_1 {
            println!(
                "buffer_1: gen: {}, prop: {}, proc: {}.",
                msg.msg_gen,
                msg.msg_prop,
                msg.msg_proc
            )
        }
    }
}

struct BkwdProcSignal1 {
    pub msg_gen: i32,
    pub msg_prop: i32,
    pub msg_proc: i32,
}

struct FrwdProcSignal1 {
    pub msg_gen: i32,
    pub msg_prop: i32,
    pub msg_proc: i32,
}
