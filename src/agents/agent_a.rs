// use std::cell::RefCell;
// use std::rc::Rc;
extern crate crossbeam_channel;
// use std::time::Duration;
use std::sync::{Mutex, Arc};
use crate::signals::signal_1::{Generate1, Propagate1, Process1};
use crate::signals::signal_1::{Signal1Gen, Signal1Prop, Signal1Proc};
use crate::supervisor;
// use crate::signals::signal_2::{Signal2, Generate2, Propagate2, Process2};

pub struct Model {
    // ports_to_super: Option(PortsToSuper),
    gen_value: i32,
    proc_value: i32,
    pub buffer_1: Vec<Signal1Proc>,
    out_connectionss_1: Vec<OutChannelSet<Signal1Gen>>,
    in_conections_1: Vec<InChannelSet<SignalProp>>,
    event_cond: Option<i32>,
}

impl Process1 for Model {

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

impl Generate1 for Model {
    fn generate_1(&self) -> Signal1Gen {
        Signal1Gen {
            msg_gen: self.gen_value,
        }
    }

    fn add_out_1 (&mut self, port_out: ExportPair<Signal1Gen>) {
        self.ports_1_out.push(port_out);
    }
}

impl Agent for Model {
    pub fn evolve(&mut self) {
        self.store_1();
        self.proc_value += 1;
        if let Some(n) = self.event_cond {
            if self.proc_value % n == 0 {
                self.send_count();
                self.wait_connections();
            }
        }
    }
    
}

impl Model {
    pub fn new(gen_value: i32, proc_value: i32, event_cond: Option<i32>) -> Arc<Mutex<Model>> {
        Arc::new(Mutex::new(
            Model{
                // ports_to_super: None,
                gen_value,
                proc_value,
                buffer_1: Vec::new(),
                ports_1_in: Vec::new(),
                ports_1_out: Vec::new(),
                event_cond,
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
            port.sgnl.send(self.generate_1()).unwrap();
        }
        self.gen_value += 1;
    }

    fn wait_connections(&self) {
        for port in self.ports_1_out.iter() {
            port.sync.recv().unwrap();
        }
    }


    
    // pub fn show_1(&self) -> Vec<(i32, i32, i32)> {
    //     self.buffer_1.iter().collect()
    // }
    
}
