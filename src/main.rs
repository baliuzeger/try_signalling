use std::rc::{Rc};
use std::cell::RefCell;
use try_signalling::agents::agent_a;
use try_signalling::signals::signal_1::Channel1;
// use crate::agents::agent_a;
// use crate::signals::signal_1::Channel1;


fn main() {
    let x = Rc::new(RefCell::new(agent_a::Agent::new()));
    let y = Rc::new(RefCell::new(agent_a::Agent::new()));
    let cn1 = Channel1::new(Rc::clone(&x), Rc::clone(&y));
    //        let cn2 = Rc::new(Channel::new(&x, &y, Signal::Signal_2_));
    x.borrow().event();
}
