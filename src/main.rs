use std::rc::{Rc};
use std::cell::RefCell;
use try_signalling::agents::agent_a;
use try_signalling::signals::signal_1::Channel1;
use try_signalling::signals::signal_2::Channel2;
// use crate::agents::agent_a;
// use crate::signals::signal_1::Channel1;


fn main() {
    let x = agent_a::Agent::new();
    let y = agent_a::Agent::new();
    let cn1 = Channel1::new(Rc::clone(&x), Rc::clone(&y));
    x.borrow().event(); // agent_a generate: 1, Channel1: 10, agent_a process: 100. output 111.
    let cn2 = Channel2::new(Rc::clone(&y), Rc::clone(&x));
    y.borrow().event(); // agent_a generate: 1, Channel1: 20, agent_a process: 100. output 111.
}
