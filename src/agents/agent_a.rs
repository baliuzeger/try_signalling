extern crate crossbeam_channel;
// use std::time::Duration;
use std::sync::{Mutex, Arc, Weak};
use crate::signals::signal_1::{Generate1, Propagate1, Process1};
use crate::signals::signal_1::{Signal1Gen, Signal1Prop, Signal1Proc};
use crate::agents::{Agent, OutConnectionSet, InConnectionSet, AgentEvent};
// use crate::signals::signal_2::{Signal2, Generate2, Propagate2, Process2};

pub struct Model {
    gen_value: i32,
    proc_value: i32,
    pub buffer_1: Vec<Signal1Proc>,
    out_connections_1: Vec<OutConnectionSet<Signal1Gen, Weak<Mutex<dyn Propagate1 + Send>>>>,
    in_connections_1: Vec<InConnectionSet<Signal1Prop, Weak<Mutex<dyn Propagate1 + Send>>>>,
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

    fn add_in_1<T: 'static + Propagate1 + Send>(&mut self, connection: Weak<Mutex<T>>, channel: crossbeam_channel::Receiver<Signal1Prop>) {
        self.in_connections_1.push(
            InConnectionSet {
                connection,
                channel,
            });
    }
}

impl Generate1 for Model {
    fn generate_1(&self) -> Signal1Gen {
        Signal1Gen {
            msg_gen: self.gen_value,
        }
    }

    fn add_out_1<T: 'static + Propagate1 + Send> (&mut self, connection: Weak<Mutex<T>>, channel: crossbeam_channel::Sender<Signal1Gen>) {
        self.out_connections_1.push(
            OutConnectionSet {
                connection,
                channel,
            }
        );
    }
}

impl Agent for Model {
    fn evolve(&mut self) -> AgentEvent {
        self.store_1();
        self.proc_value += 1;
        match self.event_cond {
            None => AgentEvent::N,
            Some(n) => {
                match self.proc_value % n {
                    0 => {
                        self.send_count();
                        AgentEvent::Y
                    },
                    _ => AgentEvent::N,
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
    
    fn store_1(&mut self) {
        for conn in &self.in_connections_1 {
            match conn.channel.try_recv() {
                Ok(s) => self.buffer_1.push(self.process_1(s)),
                Err(crossbeam_channel::TryRecvError::Disconnected) => panic!("Sender is gone!"), //should output connection & sender id.
                Err(crossbeam_channel::TryRecvError::Empty) => (),
            }
        }
    }
    
    pub fn send_count(&mut self) {
        for conn in &self.out_connections_1 {
            conn.channel.send(self.generate_1()).unwrap();
        }
        self.gen_value += 1;
    }
    
    // pub fn show_1(&self) -> Vec<(i32, i32, i32)> {
    //     self.buffer_1.iter().collect()
    // }
    
}
