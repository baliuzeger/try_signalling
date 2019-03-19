extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Mutex, Arc, Weak};
use crate::connections::{InAgentSet, OutAgentSet, PassiveConnection};
// use crate::supervisor::Broadcast;
// use crate::random_sleep;

#[derive(Debug)]
pub struct Signal2Gen {
    pub msg_gen: i32
}

#[derive(Debug)]
pub struct Signal2Prop {
    pub msg_gen: i32,
    pub msg_prop: i32,
}

#[derive(Debug)]
pub struct Signal2Proc {
    pub msg_gen: i32,
    pub msg_prop: i32,
    pub msg_proc: i32,
}

pub trait Generate2 {
    fn generate_2 (&self) -> Signal2Gen;
    fn add_out_2<T: 'static + PassivePropagate2 + Send> (&mut self, connection: Weak<Mutex<T>>, channel: CCSender<Signal2Gen>);
}

pub trait Propagate2 {
    fn refine(&self, s: Signal2Gen) -> Signal2Prop;
    fn propagate_2(&self, s: Signal2Prop);
}

pub trait Process2 {
    fn process_2(&self, s: Signal2Prop) -> Signal2Proc;
    fn add_in_2<T: 'static + Propagate2 + Send> (&mut self, connection: Weak<Mutex<T>>, channel: CCReceiver<Signal2Prop>);
}

pub struct Connection<S: Generate2 + Send, R: Process2 + Send> {
    in_agent: InAgentSet<Signal2Gen, S>,
    out_agent: OutAgentSet<Signal2Prop, R>,
    value: i32,
}

pub trait PassivePropagate2: PassiveConnection + Propagate2 {}

impl<S: Generate2 + Send, R: Process2 + Send> PassivePropagate2 for Connection<S, R> {}

impl<S: Generate2 + Send, R: Process2 + Send> Propagate2 for Connection<S, R> {
    fn refine(&self, s: Signal2Gen) -> Signal2Prop {
        Signal2Prop {
            msg_gen: s.msg_gen,
            msg_prop: self.value,
        }
    }
    
    fn propagate_2(&self, s: Signal2Prop) {
        self.out_agent.channel.send(s).unwrap();
    }
}

impl<S: Generate2 + Send, R: Process2 + Send> PassiveConnection for Connection<S, R> {
    fn propagate(&self) {
        self.propagate_2(self.refine(self.in_agent.channel.recv().unwrap()));
    }
}

impl<S: Generate2 + Send, R: Process2 + Send> Connection<S, R> {
    pub fn new(s: Arc<Mutex<S>>, r: Arc<Mutex<R>>, value: i32) -> Arc<Mutex<Connection<S, R>>>
    where S:'static + Generate2 + Send,
          R:'static + Process2 + Send
    {
        let (tx_pre, rx_pre) = crossbeam_channel::bounded::<Signal2Gen>(1);
        let (tx_post, rx_post) = crossbeam_channel::bounded::<Signal2Prop>(1);
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
        (*s.lock().unwrap()).add_out_2(Arc::downgrade(&conn), tx_pre);
        (*r.lock().unwrap()).add_in_2(Arc::downgrade(&conn), rx_post);
        conn
    }

}
