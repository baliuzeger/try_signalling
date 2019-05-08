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

    }
}

pub mod supervisor;
// pub mod populations;
// pub mod devices;
// pub mod components;
pub mod operation;
// pub mod connectivity;

fn random_sleep() {
    thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(1, 101)));
}
