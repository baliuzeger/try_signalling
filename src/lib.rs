use std::thread;
use std::time::Duration;
use rand::Rng;


#[cfg(test)]
mod tests {
    use std::rc::{Rc};
    use std::cell::RefCell;
    use crate::agents::agent_a;
    use crate::signals::signal_1::Channel1;


    #[test]
    fn it_works() {
        let x = Rc::new(RefCell::new(agent_a::Agent::new()));
        let y = Rc::new(RefCell::new(agent_a::Agent::new()));
        let cn1 = Channel1::new(Rc::clone(&x), Rc::clone(&y));
//        let cn2 = Rc::new(Channel::new(&x, &y, Signal::Signal_2_));
        x.borrow().event();
    }
}

pub mod agents;
pub mod signals;
pub mod supervisor;
pub mod agent_populations;

fn random_sleep() {
    thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(1, 101)));
}
