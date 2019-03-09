// use std::cell::RefCell;
// use std::rc::Rc;
extern crate crossbeam_channel;
// use std::time::Duration;
use std::sync::{Mutex, Arc};
use crate::signals::signal_1::{Generate1, Propagate1, Process1};
use crate::signals::signal_1::{Signal1Gen, Signal1Prop, Signal1Proc};
// use crate::signals::signal_2::{Signal2, Generate2, Propagate2, Process2};

pub struct Agent {
    gen_value: i32,
    proc_value: i32,
    buffer_1: Vec<Signal1Proc>,
    ports_1_out: Vec<crossbeam_channel::Sender<Signal1Gen>>,
    ports_1_in: Vec<crossbeam_channel::Receiver<Signal1Prop>>,
}

impl Process1 for Agent {

    fn process_1(&self, s: Signal1Prop) -> Signal1Proc {
        Signal1Proc {
            msg_gen: s.msg_gen,
            msg_prop: s.msg_prop,
            msg_proc: self.proc_value,
        }
    }

    fn add_in_1 (&mut self, port_in: crossbeam_channel::Receiver<Signal1Prop>) {
        self.ports_1_in.push(port_in);
    }
}

impl Generate1 for Agent {
    fn generate_1(&self) -> Signal1Gen {
        Signal1Gen {
            msg_gen: self.gen_value,
        }
    }

    fn add_out_1 (&mut self, port_out: crossbeam_channel::Sender<Signal1Gen>) {
        self.ports_1_out.push(port_out);
    }
}

impl Agent {
    pub fn new(gen_value: i32, proc_value: i32) -> Arc<Mutex<Agent>> {
        Arc::new(Mutex::new(
            Agent{
                gen_value,
                proc_value,
                buffer_1: Vec::new(),
                ports_1_in: Vec::new(),
                ports_1_out: Vec::new(),
            }
        ))
    }

    fn store_1(&mut self) {
        for port in &self.ports_1_in {
            match port.try_recv() {
                Ok(s) => self.buffer_1.push(self.process_1(s)),
                Err(crossbeam_channel::TryRecvError::Disconnected) => panic!("Sender is gone!"), //should output connection & sender id.
                Err(crossbeam_channel::TryRecvError::Empty) => (),
            }
        }
    }
    
    pub fn send_count(&mut self) {
        for port in self.ports_1_out.iter() {
            port.send(self.generate_1()).unwrap();
        }
        self.gen_value += 1;
    }

    pub fn evolve(&mut self) {
        self.proc_value += 1;
    }


    
    // pub fn show_1(&self) -> Vec<(i32, i32, i32)> {
    //     self.buffer_1.iter().collect()
    // }
    
}
