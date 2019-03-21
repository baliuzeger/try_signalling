extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Mutex, Arc, Weak};
use crate::connections::{InAgentSet, OutAgentSet, PassiveConnection};
// use crate::supervisor::Broadcast;
// use crate::random_sleep;

pub struct Connection<S: Generate1 + Send, R: Process1 + Send> {
    in_agent: InAgentSet<Signal1Gen, S>,
    out_agent: OutAgentSet<Signal1Prop, R>,
    value: i32,
}



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