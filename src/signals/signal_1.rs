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
    fn propagate(&self);
}

pub trait Process1 {
    fn process_1(&self, s: Signal1Prop) -> Signal1Proc;
    fn add_in_1 (&mut self, port_in: crossbeam_channel::Receiver<Signal1Prop>);
}

pub struct Connection1 {
    port_in: ImportPair<Signal1Gen>,
    port_out: crossbeam_channel::Sender<Signal1Prop>,
    value: i32,
}

impl Propagate1 for Connection1 {
    fn refine(&self, s: Signal1Gen) -> Signal1Prop {
        Signal1Prop {
            msg_gen: s.msg_gen,
            msg_prop: self.value,
        }
    }
    
    fn propagate(&self) {
        self.port_out.send(
            self.refine(
                self.port_in.sgnl.recv().unwrap()
            )
        ).unwrap();
    }
}

impl PassiveConnection for Connection1 {
    fn standby(&self) {
        self.propagate();
        self.port_in.sync.send(true).unwrap();
    }
}

impl Connection1 {
    pub fn new<S, R>(s: Arc<Mutex<S>>, r: Arc<Mutex<R>>, value: i32) -> Arc<Mutex<Connection1>>
    where S:'static + Generate1 + Send,
          R:'static + Process1 + Send
    {
        let (tx_pre_sgnl, rx_pre_sgnl) = crossbeam_channel::bounded::<Signal1Gen>(1);
        let (tx_pre_sync, rx_pre_sync) = crossbeam_channel::bounded::<Signal1Gen>(1);
        let (tx_post, rx_post) = crossbeam_channel::bounded::<Signal1Prop>(1);
        (*s.lock().unwrap()).add_out_1(ExportPair {
            sgnl: tx_pre_sgnl,
            sync: rx_pre_sync,
        });
        (*r.lock().unwrap()).add_in_1(rx_post);
        Arc::new(Mutex::new(
            Connection1 {
                port_in: ImportPair {
                    sgnl: rx_pre_sgnl,
                    sync: tx_pre_sync,
                },
                port_out: tx_post,
                value,
            }
        ))
            
    }
}
