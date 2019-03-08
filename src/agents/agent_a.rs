// use std::cell::RefCell;
// use std::rc::Rc;
extern crate crossbeam_channel;
// use std::time::Duration;
use std::sync::{Mutex, Arc};
use crate::signals::signal_1::{Signal1, Generate1, Propagate1, Process1};
use crate::signals::signal_2::{Signal2, Generate2, Propagate2, Process2};

pub struct Agent {
    gen_value: i32,
    proc_value: i32,
    buffer_1: Vec<Signal1>,
    out_channels_1: Vec<Arc<Mutex<dyn Propagate1 + Send>>>,
    in_channels_1: Vec<Arc<Mutex<dyn Propagate1 + Send>>>,
    buffer_2: Vec<Signal2>,
    out_channels_2: Vec<Arc<Mutex<dyn Propagate2 + Send>>>,
    in_channels_2: Vec<Arc<Mutex<dyn Propagate2 + Send>>>,
}

impl Process1 for Agent {
    fn process_1(&mut self, s: Signal1) {
        self.buffer_1.push(
            Signal1 {
                message: (s.message.0, s.message.1, self.proc_value)
            }
        );
    }

    fn add_in_1<C:'static + Propagate1 + Send> (&mut self, ch: Arc<Mutex<C>>) {
        self.in_channels_1.push(ch);
    }
}

impl Generate1 for Agent {
    fn generate_1(&self) -> Signal1 {
        Signal1 {
            message: (self.gen_value, 0, 0),
        }
    }

    fn add_out_1<C:'static + Propagate1 + Send> (&mut self, ch: Arc<Mutex<C>>) {
        self.out_channels_1.push(ch);
    }
}

impl Process2 for Agent {
    fn process_2(&self, s: Signal2) {
        println!("{}", self.proc_value + s.message);
    }

    fn add_in_2<C:'static + Propagate2 + Send> (&mut self, ch: Arc<Mutex<C>>) {
        self.in_channels_2.push(ch);
    }
}

impl Generate2 for Agent {
    fn generate_2(&self) -> Signal2 {
        Signal2 {
            message: self.gen_value,
        }
    }

    fn add_out_2<C:'static + Propagate2 + Send> (&mut self, ch: Arc<Mutex<C>>) {
        self.out_channels_2.push(ch);
    }
}

impl Agent {
    pub fn new(gen_value: i32, proc_value: i32) -> Arc<Mutex<Agent>> {
        Arc::new(Mutex::new(
            Agent{
                gen_value,
                proc_value,
                buffer_1: Vec::new(),
                out_channels_1: Vec::new(),
                in_channels_1: Vec::new(),
                buffer_2: Vec::new(),
                out_channels_2: Vec::new(),
                in_channels_2: Vec::new(),
            }
        ))
    }

    pub fn event(&self) {
        // let a_sgnl_1 = self.generate_1();
        for cn in self.out_channels_1.iter() {
            cn.lock().unwrap().propagate(self.generate_1());
        }
        for cn in self.out_channels_2.iter() {
            cn.lock().unwrap().propagate(self.generate_2());
        }        
    }

    pub fn send_count(&mut self) {
        // let a_sgnl_1 = self.generate_1();
        for cn in self.out_channels_1.iter() {
            cn.lock().unwrap().propagate(self.generate_1());
        }
        self.gen_value += 1;
    }

    pub fn evolve(&mut self) {
        self.proc_value += 1;
    }


    
    pub fn show_1(&self) -> Vec<(i32, i32, i32)> {
        self.buffer_1.iter().map(|s| s.message ).collect()
    }
    
}
