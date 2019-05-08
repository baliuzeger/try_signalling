use std::sync::{Mutex, Arc, Weak};
use crate::connectivity::s1_pre::{MultiOutComponentS1Pre, FwdPreS1};
use crate::connectivity::s1_post::{MultiInComponentS1Post, FwdPostS1};
use crate::connectivity::{Generator, Acceptor};
use crate::operation::{ActiveDevice, FiringDevice};

pub struct NeuronC {
    out_s1_pre: MultiOutComponentS1Pre,
    in_s1_post: MultiInComponentS1Post,
    gen_value: i32,
    proc_value: i32,
    event_cond: Option<i32>,
    stock: Vec<FwdEndProduct>,
}

struct FwdEndProduct {
    pub msg_gen: i32,
    pub msg_prop: i32,
    pub msg_proc: i32,
}

impl Generator<FwdPreS1> for Model {
    fn add_passive<A>(&mut self, post: Weak<Mutex<A>>, linker: Arc<Mutex<Linker<FwdPreS1>>>)
        where A: PassiveAcceptor<FwdPreS1>,
    {
        self.out_s1_pre.add_passive_target(post, linker);
    }
}

impl Acceptor<FwdPostS1> {
    fn add<G>(&mut self, pre: Weak<Mutex<G>>, linker: Arc<Mutex<Linker<FwdPostS1>>>)
    where G: Generator<FwdPostS1>,
    {
        self.in_s1_post.add_target(pre, linker);
    }
}

impl NeuronC ActiveDevice {}

impl NeuronC FiringDevice {
    fn config_run(&mut self, mode: RunMode);
    fn config_channels(&mut self);
    fn running_passive_devices(&self) -> Vec<RunningSet<Broadcast, ()>>;
    fn end(&mut self);
    fn evolve(&mut self) -> Fired;
}


impl Agent for Model {
    fn config_run(&mut self, mode: RunMode) {
        match (mode, self.mode()) {
            (RunMode::Idle, _) => println!("config_run for mode Idle, no effect."),
            (_, RunMode::Idle) => {
                self.pre_module_s1.config_run(mode);
                self.post_module_s1.config_run(mode);
            },
            (_, _) => panic!("call config_run when agent not idle!")
        }
    }

    fn config_idle(&mut self) {
        match &self.mode() {
            RunMode::Idle => println!("config_idel at mode Idle, no effect."),
            RunMode::Feedforward => {
                self.pre_module_s1.config_idle();
                self.post_module_s1.config_idle();
            },
        }
    }

    fn running_connections(&self) -> Vec<RunningPassiveConnection> {
        self.pre_module_s1.running_connections()
    }
    
    fn end(&mut self) {
        self.accept();
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
                        println!("agnet c fire. gen: {}, proc: {}.",  self.gen_value, self.proc_value);
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
        RunMode::eq_mode(self.pre_module_s1.mode(),self.post_module_s1.mode())
    }
    
    fn generate(&self) {
        self.pre_module_s1.feedforward(FwdPreS1 {
            msg_gen: self.gen_value,
        });
    }

    fn accept(&mut self) {
        let mut acc = self.post_module_s1.ffw_accepted().iter().map(|s| FwdEndProduct {
                msg_gen: s.msg_gen,
                msg_prop: s.msg_prop,
                msg_proc: self.proc_value,
            }  
        ).collect::<Vec<FwdEndProduct>>();

        // for demo accepting
        for msg in &acc {
            println!(
                "agent c accept: gen: {}, prop: {}, proc: {}.",
                msg.msg_gen,
                msg.msg_prop,
                msg.msg_proc
            )
        }
        
        self.stock.append(&mut acc);
    }

    pub fn print_values(&self) {
        println!("gen: {}, proc: {}.", self.gen_value, self.proc_value);
    }
    
    pub fn show(&self) {
        for msg in &self.stock {
            println!(
                "agent c buffer: gen: {}, prop: {}, proc: {}.",
                msg.msg_gen,
                msg.msg_prop,
                msg.msg_proc
            )
        }
    }
}
