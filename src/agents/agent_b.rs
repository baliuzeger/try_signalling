extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crate::supervisor::{Broadcast, RunningSet};
use std::thread;
use std::sync::{Mutex, Arc, Weak};
use crate::signals::signal_1::{Generate1, Propagate1, Process1, PassivePropagate1};
use crate::signals::signal_1::{Signal1Gen, Signal1Prop, Signal1Proc};
use crate::signals::signal_2::{Generate2, Propagate2, Process2, PassivePropagate2};
use crate::signals::signal_2::{Signal2Gen, Signal2Prop, Signal2Proc};
use crate::agents::{Agent, AgentPopulation, OutConnectionSet, InConnectionSet, AgentEvent};
use crate::random_sleep;
use crate::signals::PassiveConnection;



impl AgentPopulation for Population {

}

impl Population {
    pub fn new() -> Arc<Mutex<Population>> {
        Arc::new(Mutex::new(Population{
            agents: Vec::new(),
        }))
    }

    pub fn add_agent(&mut self, agnt: Arc<Mutex<Model>>) {
        self.agents.push(agnt);
    }

    
    pub fn agent_by_id(&self, n: usize) -> Arc<Mutex<Model>> {
        Arc::clone(&self.agents[n])
    }
}

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

    fn init_passive_connection<C>(conn: Arc<Mutex<C>>) -> RunningSet<bool>
    where C: 'static + PassiveConnection + Send + ?Sized
    {
        let (tx_conn_report, rx_conn_report) = crossbeam_channel::bounded(1);
        let (tx_conn_confirm, rx_conn_confirm) = crossbeam_channel::bounded(1);
        RunningSet {
            instance: thread::spawn(move || {conn.lock().unwrap().run_under_agent(rx_conn_confirm, tx_conn_report)}),
            report: rx_conn_report,
            confirm: tx_conn_confirm,
        }
    }

    fn evolve(&mut self) -> AgentEvent {
        self.store_1();
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
    
    fn store_1(&mut self) {
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
