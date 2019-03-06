use crate::signals::signal_1::{Signal1, Generate1, Propagate1, Process1};

use std::rc::Rc;

pub struct Agent {
    gen_value: i32,
    proc_value: i32,
    out_channels_1: Vec<Rc<dyn Propagate1>>,
//    out_channels_2: Vec<Rc<dyn Propagate_2>>,
    in_channels_1: Vec<Rc<dyn Propagate1>>,
//    in_channels_2: Vec<Rc<dyn Propagate_2>>,
}

impl Process1 for Agent {
    fn process_1(&self, s: Signal1) {
        println!("{}", self.proc_value + s.message);
    }

    fn add_in_1<C:'static + Propagate1> (&mut self, ch: Rc<C>) {
        self.in_channels_1.push(ch);
    }
}

impl Generate1 for Agent {
    fn generate_1(&self) -> Signal1 {
        Signal1 {
            message: self.gen_value,
        }
    }

    fn add_out_1<C:'static + Propagate1> (&mut self, ch: Rc<C>) {
        self.out_channels_1.push(ch);
    }
}

impl Agent {
    pub fn new() -> Agent {
        Agent {
            gen_value: 1,
            proc_value: 100,
            out_channels_1: Vec::new(),
            // out_channels_2: Vec::new();
            in_channels_1: Vec::new(),
            // in_channels_2: Vec::new();
        }
    }

    pub fn event(&self) {
        // let a_sgnl_1 = self.generate_1();
        for cn in self.out_channels_1.iter() {
            cn.propagate(self.generate_1());
        }        
    }
}
