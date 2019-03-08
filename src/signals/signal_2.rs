/// should impl copy trait on Signal for use in agent.generate
// use crate::agents;
// use std::rc::{Rc, Weak};
// use std::cell::RefCell;
extern crate crossbeam_channel;
// use std::time::Duration;
use std::sync::{Mutex, Arc, Weak};

pub struct Signal2 {
    pub message: i32,
}

pub trait Propagate2 {
    fn refine(&self, s: Signal2) -> Signal2;
    fn propagate(&self, s: Signal2);
}

pub trait Process2 {
    fn process_2(&self, s: Signal2);
    fn add_in_2<C:'static + Propagate2 + Send> (&mut self, ch: Arc<Mutex<C>>);
}

pub trait Generate2 {
    fn generate_2 (&self) -> Signal2;
    fn add_out_2<C:'static + Propagate2 + Send> (&mut self, ch: Arc<Mutex<C>>);
}

pub struct Channel2<S: Generate2, R: Process2> {
    sender: Weak<Mutex<S>>,
    receiver: Weak<Mutex<R>>,
    value: i32,
}

impl<S: Generate2, R: Process2> Propagate2 for Channel2<S, R> {
    fn refine(&self, s: Signal2) -> Signal2 {
        Signal2 {
            message: self.value + s.message,
        }
    }
    
    fn propagate(&self, s: Signal2) {
        self.receiver.upgrade().unwrap().lock().unwrap().process_2(self.refine(s));
    }
}

impl<S, R> Channel2<S, R>
where S:'static + Generate2 + Send,
      R:'static + Process2 + Send

{
    pub fn new(s: Arc<Mutex<S>>, r: Arc<Mutex<R>>) -> Arc<Mutex<Channel2<S, R>>> {
        let ch = Arc::new(Mutex::new(
            Channel2 {
                sender: Arc::downgrade(&s),
                receiver: Arc::downgrade(&r),
                value: 20,
            }
        ));
        s.lock().unwrap().add_out_2(Arc::clone(&ch));
        r.lock().unwrap().add_in_2(Arc::clone(&ch));
        ch
    }
}
