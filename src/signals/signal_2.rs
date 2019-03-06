/// should impl copy trait on Signal for use in agent.generate
// use crate::agents;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct Signal2 {
    pub message: i32,
}

pub trait Propagate2 {
    fn refine(&self, s: Signal2) -> Signal2;
    fn propagate(&self, s: Signal2);
}

pub trait Process2 {
    fn process_2(&self, s: Signal2);
    fn add_in_2<C:'static + Propagate2> (&mut self, ch: Rc<C>);
}

pub trait Generate2 {
    fn generate_2 (&self) -> Signal2;
    fn add_out_2<C:'static + Propagate2> (&mut self, ch: Rc<C>);
}

pub struct Channel2<S: Generate2, R: Process2> {
    sender: Weak<RefCell<S>>,
    receiver: Weak<RefCell<R>>,
    value: i32,
}

impl<S: Generate2, R: Process2> Propagate2 for Channel2<S, R> {
    fn refine(&self, s: Signal2) -> Signal2 {
        Signal2 {
            message: self.value + s.message,
        }
    }
    
    fn propagate(&self, s: Signal2) {
        self.receiver.upgrade().unwrap().borrow().process_2(self.refine(s));
    }
}

impl<S:'static + Generate2, R:'static + Process2> Channel2<S, R> {
    pub fn new(s: Rc<RefCell<S>>, r: Rc<RefCell<R>>) -> Rc<Channel2<S, R>> {
        let ch = Rc::new(Channel2 {
            sender: Rc::downgrade(&s),
            receiver: Rc::downgrade(&r),
            value: 20,
        });
        s.borrow_mut().add_out_2(Rc::clone(&ch));
        r.borrow_mut().add_in_2(Rc::clone(&ch));
        ch
    }
}
