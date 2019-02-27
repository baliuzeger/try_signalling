use std::rc::Rc;
use std::ops::Deref;
use crate::agents::{Is_sender, Is_receiver, agent_a};

pub enum Sender {
    Agent_a_(Rc<agent_a::Agent>), // send & receive
//    Agent_b_(Rc<Agent_b::Agent>), // send only
}

pub enum Receiver {
    Agent_a_(Rc<agent_a::Agent>), // send & receive
//    Agent_c_(Rc<Agent_c>), // send only
}

impl Receiver {
    pub fn unwrap<T> (&mut self) -> T {
        match self {
            Receiver::Agent_a_(rc_agnt) => Rc::get_mut(&mut rc_agnt).unwrap(),
//            Agent_c_(Rc(agnt)) => agnt,
        }
    }
}

impl Sender {
    pub fn unwrap<T> (&mut self) -> T {
        match self {
            Receiver::Agent_a_(rc_agnt) => Rc::get_mut(&mut rc_agnt).unwrap(),
//            Agent_c_(Rc(agnt)) => agnt,
        }
    }
}

pub enum Signal {
    Signal_1_,
    Signal_2_,
}

pub struct Channel {
    pub sender: Rc<Sender>,
    pub receiver: Rc<Receiver>,
    pub signal_sample: Signal,
}

impl Channel {
    fn new<S: Is_sender, R: Is_receiver> (sender: &Rc<S>, receiver: &Rc<R>, signal_sample: Signal) -> Channel {
        let ch = Channel {
            sender: Rc::clone(sender.wrap_sender()),
            receiver: Rc::clone(receiver.wrap_receiver()),
            signal_sample: signal_sample,
        };
        receiver.deref().unwrap().add_in_channel(ch);
        sender.deref().unwrap().add_out_channel(ch);
        ch
    }
}


pub struct Signal_1 {
    pub message: String,
}

impl Signal_1 {
    fn sample() -> Signal_1 {
        Signal_1 {message: String::from("sample s1.")}
    }
}

pub struct Signal_2 {
    pub message: String,
}

impl Signal_2 {
    fn sample() -> Signal_2 {
        Signal_2 {message: String::from("ref s2.")}
    }
}



