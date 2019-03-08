/// should impl copy trait on Signal for use in agent.generate
// use crate::agents;
// use std::rc::{Rc, Weak};
// use std::cell::RefCell;
extern crate crossbeam_channel;
// use std::time::Duration;
use std::sync::{Mutex, Arc, Weak};

pub struct Signal1 {
    pub message: (i32, i32, i32),
}

pub trait Propagate1 {
    fn refine(&self, s: Signal1) -> Signal1;
    fn propagate(&self, s: Signal1);
}

pub trait Process1 {
    fn process_1(&mut self, s: Signal1);
    fn add_in_1<C:'static + Propagate1 + Send> (&mut self, ch: Arc<Mutex<C>>);
}

pub trait Generate1 {
    fn generate_1 (&self) -> Signal1;
    fn add_out_1<C:'static + Propagate1 + Send> (&mut self, ch: Arc<Mutex<C>>);
}

pub struct Channel1<S: Generate1, R: Process1> {
    sender: Weak<Mutex<S>>,
    receiver: Weak<Mutex<R>>,
    value: i32,
}

impl<S: Generate1, R: Process1> Propagate1 for Channel1<S, R> {
    fn refine(&self, s: Signal1) -> Signal1 {
        Signal1 {
            message: (s.message.0, self.value, 0)
        }
    }
    
    fn propagate(&self, s: Signal1) {
        self.receiver.upgrade().unwrap().lock().unwrap().process_1(self.refine(s));
    }
}

impl<S, R> Channel1<S, R>
where S:'static + Generate1 + Send,
      R:'static + Process1 + Send
{
    pub fn new(s: Arc<Mutex<S>>, r: Arc<Mutex<R>>) -> Arc<Mutex<Channel1<S, R>>> {
        let ch = Arc::new(Mutex::new(
            Channel1 {
                sender: Arc::downgrade(&s),
                receiver: Arc::downgrade(&r),
                value: 10,
            }   
        ));
        (*s.lock().unwrap()).add_out_1(Arc::clone(&ch));
        (*r.lock().unwrap()).add_in_1(Arc::clone(&ch));
        ch
    }
}
