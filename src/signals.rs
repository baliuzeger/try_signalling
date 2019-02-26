use std::rc::Rc;
use crate::agents::{agent_a, agent_b};

pub enum Sender {
    Agent_a_(Rc<Agent_a>), // send & receive
    Agent_b_(Rc<Agent_b>), // send only
}

pub enum Receiver {
    Agent_a_(Rc<Agent_a>), // send & receive
    Agent_c_(Rc<Agent_c>), // send only
}

pub enum Signal {
    Signal_1_,
    Signal_2_,
}

pub struct Channel {
    sender: Rc<Sender>,
    receiver: Rc<Receiver>,
    pub signal_sample: Signal,
}

impl Channel {
    fn new (&sender: Rc<T>, &receiver: Rc<U>, signal_sample: Signal) -> Channel {
        let ch = Channel {
            sender: Rc::clone(sender.wrap_sender()),
            receiver: Rc::clone(receiver.wrap_receiver()),
            signal_sample: signal_sample,
        };
        receiver.add_in_channel(ch);
        sender.add_out_channel(ch);
        ch
    }
}


pub struct Signal_1 {
    pub message: String,
}

impl Signal_1 {
    fn sample() -> Signal_1 {
        Signal_1 {name: String::from("sample s1.")}
    }
}

pub struct Signal_2 {
    pub message: String,
}

impl Signal_2 {
    fn sample() -> Signal_2 {
        Signal_2 {name: String::from("ref s2.")}
    }
}



