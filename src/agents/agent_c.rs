use std::sync::{Mutex, Arc, Weak};
use crate::connections::{RunningPassiveConnection};
use crate::connections::signal_1::{S1Generator, S1Acceptor, S1PassivePropagator};
use crate::connections::signal_1::{FwdPreS1};
use crate::connections::signal_1::{PreAgentComponentS1, PostAgentComponentS1};
use crate::agents::{Agent, AgentEvent};
use crate::supervisor::{RunMode, DeviceMode};

pub struct Model {
    config: RunMode,
    pre_module_s1: PreAgentComponentS1,
    post_module_s1: PostAgentComponentS1,
    gen_value: i32,
    proc_value: i32,
    event_cond: Option<i32>,
    stock: Vec<EndProduct>,
}

struct EndProduct {
    pub msg_gen: i32,
    pub msg_prop: i32,
    pub msg_proc: i32,
}

impl S1Generator for Model {
    fn add_out_passive_s1 (&mut self, connection: Weak<Mutex<dyn S1PassivePropagator + Send>>)
    {
        self.pre_module_s1.add_connection(connection);
        
    }
}

impl S1Acceptor for Model {
    fn add_in_s1(&mut self, connection: Weak<Mutex<dyn S1PassivePropagator + Send>>) {
        self.post_module_s1.add_connection(connection);
    }
}

impl Agent for Model {
    fn config_run(&mut self, mode: RunMode) {
        match DeviceMode::eq_variant(mode, self.mode()) {
            DeviceMode::Idle(_) => println!("config_run for mode Idle, no effect."),
            DeviceMode::Feedforward(_) => {
                self.pre_module_s1.config_run(mode);
                self.post_module_s1.config_run(mode);
            },
        }
        
    }

    fn config_idle(&mut self) {
        match &self.mode() {
            DeviceMode::Idle(_) => println!("config_idel at mode Idle, no effect."),
            DeviceMode::Feedforward(_) => {
                self.pre_module_s1.config_idle();
                self.post_module_s1.config_idle();
            },
        }
    }

    fn running_connections(&self) -> Vec<RunningPassiveConnection> {
        self.pre_module_s1.running_connections()
    }
    
    fn end(&mut self) {
        self.store();
    }
    
    fn evolve(&mut self) -> AgentEvent {
        self.accept();
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
                        self.generate();
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
                pre_module_s1: PreAgentComponentS1::new(),
                post_module_s1: PostAgentComponentS1::new(),
                gen_value,
                proc_value,
                event_cond,
                stock: Vec::new(),
            }
        ))
    }

    fn mode(&self) -> RunMode {
        DeviceMode::eq_variant(self.pre_module_s1.mode(),self.post_module_s1.mode())
    }
    
    fn generate(&self) {
        self.pre_module_s1.feedforward(FwdPreS1 {
            msg_gen: self.gen_value,
        });
    }

    fn accept(&mut self) {
        self.stock.append(
            self.post_module_s1.ffw_accepted().iter().map(|s| EndProduct {
                msg_gen: s.msg_gen,
                msg_prop: s.msg_prop,
                msg_proc: self.proc_value,
            }).collect()
        );
    }

    pub fn print_values(&self) {
        println!("gen: {}, proc: {}.", self.gen_value, self.proc_value);
    }
    
    pub fn show(&self) {
        for msg in &self.stock {
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
