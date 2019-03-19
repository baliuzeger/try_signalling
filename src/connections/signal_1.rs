extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Mutex, Arc, Weak};
use crate::connections::{InAgentSet, OutAgentSet, PassiveConnection};
// use crate::supervisor::Broadcast;
// use crate::random_sleep;

#[derive(Debug)]
pub struct Signal1Gen {
    pub msg_gen: i32
}

#[derive(Debug)]
pub struct Signal1Prop {
    pub msg_gen: i32,
    pub msg_prop: i32,
}

#[derive(Debug)]
pub struct Signal1Proc {
    pub msg_gen: i32,
    pub msg_prop: i32,
    pub msg_proc: i32,
}

pub trait Generate1 {
    fn generate_1 (&self) -> Signal1Gen;
    fn add_out_1<T: 'static + PassivePropagate1 + Send> (&mut self, connection: Weak<Mutex<T>>, channel: CCSender<Signal1Gen>);
}

pub trait Propagate1 {
    fn refine(&self, s: Signal1Gen) -> Signal1Prop;
    fn propagate_1(&self, s: Signal1Prop);
}

pub trait Process1 {
    fn process_1(&self, s: Signal1Prop) -> Signal1Proc;
    fn add_in_1<T: 'static + Propagate1 + Send> (&mut self, connection: Weak<Mutex<T>>, channel: CCReceiver<Signal1Prop>);
}

pub struct Connection<S: Generate1 + Send, R: Process1 + Send> {
    in_agent: InAgentSet<Signal1Gen, S>,
    out_agent: OutAgentSet<Signal1Prop, R>,
    value: i32,
}

pub trait PassivePropagate1: PassiveConnection + Propagate1 {}

impl<S: Generate1 + Send, R: Process1 + Send> PassivePropagate1 for Connection<S, R> {}

impl<S: Generate1 + Send, R: Process1 + Send> Propagate1 for Connection<S, R> {
    fn refine(&self, s: Signal1Gen) -> Signal1Prop {
        Signal1Prop {
            msg_gen: s.msg_gen,
            msg_prop: self.value,
        }
    }
    
    fn propagate_1(&self, s: Signal1Prop) {
        self.out_agent.channel.send(s).unwrap();
    }
}

impl<S: Generate1 + Send, R: Process1 + Send> PassiveConnection for Connection<S, R> {
    fn propagate(&self) {
        self.propagate_1(self.refine(self.in_agent.channel.recv().unwrap()));
    }
}

impl<S: Generate1 + Send, R: Process1 + Send> Connection<S, R> {
    pub fn new(s: Arc<Mutex<S>>, r: Arc<Mutex<R>>, value: i32) -> Arc<Mutex<Connection<S, R>>>
    where S:'static + Generate1 + Send,
          R:'static + Process1 + Send
    {
        let (tx_pre, rx_pre) = crossbeam_channel::bounded::<Signal1Gen>(1);
        let (tx_post, rx_post) = crossbeam_channel::bounded::<Signal1Prop>(1);
        let conn = Arc::new(Mutex::new(
            Connection {
                in_agent: InAgentSet {
                    agent: Arc::clone(&s),
                    channel: rx_pre,
                },
                out_agent: OutAgentSet {
                    agent: Arc::clone(&r),
                    channel: tx_post,
                },
                value,
            }
        ));
        (*s.lock().unwrap()).add_out_1(Arc::downgrade(&conn), tx_pre);
        (*r.lock().unwrap()).add_in_1(Arc::downgrade(&conn), rx_post);
        conn
    }

}
