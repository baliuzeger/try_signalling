use std::rc::{Rc};
use std::cell::RefCell;
use try_signalling::agents::agent_a;
use try_signalling::signals::signal_1::Channel1;
use try_signalling::signals::signal_2::Channel2;
// use crate::agents::agent_a;
// use crate::signals::signal_1::Channel1;


fn main() {
    let x = agent_a::Agent::new(0, 0);
    let y = agent_a::Agent::new(10, 0);
    let z = agent_a::Agent::new(100, 0);
    let cn1 = Channel1::new(Rc::clone(&x), Rc::clone(&z));
    let cn1 = Channel1::new(Rc::clone(&y), Rc::clone(&z));
    for i in (0..7) {
        if i == 2 || i == 3 || i == 4 {
            x.borrow_mut().send_count();
            y.borrow_mut().send_count();            
        }
        x.borrow_mut().evolve();
        y.borrow_mut().evolve();
        z.borrow_mut().evolve();
    }
    println!("{:?}", z.borrow().show_1());
}
