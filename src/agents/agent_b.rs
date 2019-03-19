extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Mutex, Arc, Weak};
use crate::connections::{RunningPassiveConnection};
use crate::connections::signal_1::{Generate1, Propagate1, Process1, PassivePropagate1};
use crate::connections::signal_1::{Signal1Gen, Signal1Prop, Signal1Proc};
use crate::connections::signal_2::{Generate2, Propagate2, Process2, PassivePropagate2};
use crate::connections::signal_2::{Signal2Gen, Signal2Prop, Signal2Proc};
use crate::agents::{Agent, OutConnectionSet, InConnectionSet, AgentEvent};

pub struct Model {
    gen_value: i32,
    proc_value: i32,
    pub buffer_1: Vec<Signal1Proc>,
    out_connections_1: Vec<OutConnectionSet<Signal1Gen, Weak<Mutex<dyn PassivePropagate1 + Send>>>>,
    in_connections_1: Vec<InConnectionSet<Signal1Prop, Weak<Mutex<dyn Propagate1 + Send>>>>,
    pub buffer_2: Vec<Signal2Proc>,
    out_connections_2: Vec<OutConnectionSet<Signal2Gen, Weak<Mutex<dyn PassivePropagate2 + Send>>>>,
    in_connections_2: Vec<InConnectionSet<Signal2Prop, Weak<Mutex<dyn Propagate2 + Send>>>>,
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

    fn add_in_1<T: 'static + Propagate1 + Send>(&mut self, connection: Weak<Mutex<T>>, channel: CCReceiver<Signal1Prop>) {
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

    fn add_out_1<T: 'static + PassivePropagate1 + Send> (&mut self, connection: Weak<Mutex<T>>, channel: CCSender<Signal1Gen>) {
        self.out_connections_1.push(
            OutConnectionSet {
                connection,
                channel,
            }
        );
    }
}

impl Agent for Model {
    fn running_connections(&self) -> Vec<RunningPassiveConnection> {
        self.out_connections_1.iter().map(|cn| RunningPassiveConnection::new(cn.connection.upgrade().unwrap()))
            .chain(self.out_connections_2.iter().map(|cn| RunningPassiveConnection::new(cn.connection.upgrade().unwrap())))
            .collect()
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
                buffer_2: Vec::new(),
                out_connections_2: Vec::new(),
                in_connections_2: Vec::new(),
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
        for conn in &self.in_connections_2 {
            match conn.channel.try_recv() {
                Ok(s) => {
                    // println!(
                    //     "receiving: gen: {}, prop: {}; self: gen {}, proc: {}.",
                    //     s.msg_gen,
                    //     s.msg_prop,
                    //     self.gen_value,
                    //     self.proc_value
                    // );
                    self.buffer_2.push(self.process_2(s))
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
        for conn in &self.out_connections_2 {
            conn.channel.send(self.generate_2()).unwrap();
        }
        // self.gen_value += 1;
    }

    pub fn print_values(&self) {
        println!("gen: {}, proc: {}.", self.gen_value, self.proc_value);
    }
    
    pub fn show_1(&self) {
        println!("Start show_1:");
        for msg in &self.buffer_1 {
            println!(
                "buffer_1: gen: {}, prop: {}, proc: {}.",
                msg.msg_gen,
                msg.msg_prop,
                msg.msg_proc
            )
        }
    }

    pub fn show_2(&self) {
        println!("Start show_2:");
        for msg in &self.buffer_2 {
            println!(
                "buffer_2: gen: {}, prop: {}, proc: {}.",
                msg.msg_gen,
                msg.msg_prop,
                msg.msg_proc
            )
        }
    }
    
}

impl Process2 for Model {
    fn process_2(&self, s: Signal2Prop) -> Signal2Proc {
        Signal2Proc {
            msg_gen: s.msg_gen,
            msg_prop: s.msg_prop,
            msg_proc: self.proc_value,
        }
    }

    fn add_in_2<T: 'static + Propagate2 + Send>(&mut self, connection: Weak<Mutex<T>>, channel: CCReceiver<Signal2Prop>) {
        self.in_connections_2.push(
            InConnectionSet {
                connection,
                channel,
            });
    }
}

impl Generate2 for Model {
    fn generate_2(&self) -> Signal2Gen {
        Signal2Gen {
            msg_gen: self.gen_value,
        }
    }

    fn add_out_2<T: 'static + PassivePropagate2 + Send> (&mut self, connection: Weak<Mutex<T>>, channel: CCSender<Signal2Gen>) {
        self.out_connections_2.push(
            OutConnectionSet {
                connection,
                channel,
            }
        );
    }
}
