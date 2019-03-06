/// should impl copy trait on Signal for use in agent.generate
// use crate::agents;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct Signal1 {
    pub message: i32,
}

pub trait Propagate1 {
    fn refine(&self, s: Signal1) -> Signal1;
    fn propagate(&self, s: Signal1);
}

pub trait Process1 {
    fn process_1(&self, s: Signal1);
    fn add_in_1<C:'static + Propagate1> (&mut self, ch: Rc<RefCell<C>>);
}

pub trait Generate1 {
    fn generate_1 (&self) -> Signal1;
    fn add_out_1<C:'static + Propagate1> (&mut self, ch: Rc<RefCell<C>>);
}

pub struct Channel1<S: Generate1, R: Process1> {
    sender: Weak<RefCell<S>>,
    receiver: Weak<RefCell<R>>,
    value: i32,
}

impl<S: Generate1, R: Process1> Propagate1 for Channel1<S, R> {
    fn refine(&self, s: Signal1) -> Signal1 {
        Signal1 {
            message: self.value + s.message,
        }
    }
    
    fn propagate(&self, s: Signal1) {
        self.receiver.upgrade().unwrap().borrow().process_1(self.refine(s));
    }
}

impl<S:'static + Generate1, R:'static + Process1> Channel1<S, R> {
    pub fn new(s: Rc<RefCell<S>>, r: Rc<RefCell<R>>) -> Rc<RefCell<Channel1<S, R>>> {
        let ch = Rc::new(RefCell::new(
            Channel1 {
                sender: Rc::downgrade(&s),
                receiver: Rc::downgrade(&r),
                value: 10,
            }   
        ));
        s.borrow_mut().add_out_1(Rc::clone(&ch));
        r.borrow_mut().add_in_1(Rc::clone(&ch));
        ch
    }
}
