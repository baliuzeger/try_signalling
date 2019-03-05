// use crate::agents;
use std::rc::{Rc, Weak};


pub struct Signal_1 {
    pub message: i32,
}

pub trait Propagate_1 {
    fn refine(&self, s: Signal_1) -> Signal_1;
    fn propagate(&self, s: Signal_1);
}

pub trait Process_1 {
    fn process_1(&self, s: Signal_1);
    // fn add_in_1<'a, T> (&self, ch: &'a T) where &'a T: Propagate_1,;
    // fn add_in_1<T> (&self, ch: &T);
    fn add_in_1<T> (&self, ch: &'static T) where T: Propagate_1,;
}

pub trait Generate_1 {
    fn generate_1 (&self) -> Signal_1;
    fn add_out_1<T: Propagate_1> (&self, ch: &T);
}

pub struct Channel_1<S: Generate_1, R: Process_1> {
    sender: Weak<S>,
    receiver: Weak<R>,
    value: i32,
}

impl<S: Generate_1, R: Process_1> Propagate_1 for Channel_1<S, R> {
    fn refine(&self, s: Signal_1) -> Signal_1 {
        Signal_1 {
            message: self.value + s.message,
        }
    }
    
    fn propagate(&self, s: Signal_1) {
        self.receiver.upgrade().unwrap().process_1(self.refine(s));
    }
}

impl<S: Generate_1, R: Process_1> Channel_1<S, R> {
    fn new(s: Rc<S>, r: Rc<R>) -> Channel_1<S, R> {
        let ch = Channel_1 {
            sender: Rc::downgrade(&s),
            receiver: Rc::downgrade(&r),
            value: 10,
        };
        s.add_out_1(&ch);
        r.add_in_1(&ch);
        ch
    }
}
