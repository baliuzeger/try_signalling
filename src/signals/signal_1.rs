// use crate::agents;
// use std::rc::{Rc, Weak};
// use std::cell::RefCell;
extern crate crossbeam_channel;
// use std::time::Duration;
use std::sync::{Mutex, Arc, Weak};

pub struct Signal1Gen {
    pub msg_gen: i32
}

pub struct Signal1Prop {
    pub msg_gen: i32,
    pub msg_prop: i32,
}

pub struct Signal1Proc {
    pub msg_gen: i32,
    pub msg_prop: i32,
    pub msg_proc: i32,
}

pub trait Generate1 {
    fn generate_1 (&self) -> Signal1Gen;
    fn add_out_1 (&mut self, port_out: crossbeam_channel::Sender<Signal1Gen>);
}

pub trait Propagate1 {
    fn refine(&self, s: Signal1Gen) -> Signal1Prop;
    fn propagate(&self, s: Signal1Prop);
}

pub trait Process1 {
    fn process_1(&self, s: Signal1Prop) -> Signal1Proc;
    fn add_in_1 (&mut self, port_in: crossbeam_channel::Receiver<Signal1Prop>);
}

pub struct Connection1<S: Generate1, R: Process1> {
    in_agent: InAgentSet<Signal1Gen, S>,
    out_agent: OutAgentSet<Signal1Prop, R>,
    value: i32,
}

impl Propagate1 for Connection1 {
    fn refine(&self, s: Signal1Gen) -> Signal1Prop {
        Signal1Prop {
            msg_gen: s.msg_gen,
            msg_prop: self.value,
        }
    }
    
    fn propagate(&self, s: Signal1Prop) {
        self.port_out.send(s).unwrap();
    }
}

impl PassiveConnection for Connection1 {
    fn standby(&self) {
        match in_agent.try_recv() {
            Ok(s) => self.propagate(self.refine(s));
            Err(crossbeam_channel::TryRecvError::Disconnected) => panic!("Sender is gone!"), //should output connection & sender id.
            Err(crossbeam_channel::TryRecvError::Empty) => (),
        }
    }
}

impl Connection1 {
    pub fn new<S, R>(s: Arc<Mutex<S>>, r: Arc<Mutex<R>>, value: i32) -> Arc<Mutex<Connection1>>
    where S:'static + Generate1 + Send,
          R:'static + Process1 + Send
    {
        let (tx_pre, rx_pre) = crossbeam_channel::bounded::<Signal1Gen>(1);
        let (tx_post, rx_post) = crossbeam_channel::bounded::<Signal1Prop>(1);
        let conn = Arc::new(Mutex::new(
            Connection1 {
                in_agnet: InAgentSet {
                    agent: s,
                    channel: rx_pre,
                },
                port_out: OutAgentSet {
                    agent: r,
                    channel: tx_post,
                }
                value,
            }
        ));
        (*s.lock().unwrap()).add_out_1(Arc::downgrade(conn), tx_pre);
        (*r.lock().unwrap()).add_in_1(Arc::downgrade(conn), rx_post);
        conn
    }
}
