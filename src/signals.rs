use std::rc::Rc;
// use std::ops::Deref;
use crate::agents::{Is_sender, Is_receiver, agent_a};

pub enum Sender<'a> {
    Agent_a_(Rc<agent_a::Agent<'a>>), // send & receive
//    Agent_b_(Rc<Agent_b::Agent>), // send only
}

pub enum Receiver<'a> {
    Agent_a_(Rc<agent_a::Agent<'a>>), // send & receive
//    Agent_c_(Rc<Agent_c>), // send only
}

// impl Receiver {
//     pub fn unwrap<T: Is_receiver> (&mut self) -> T {
//         match self {
//             Receiver::Agent_a_(rc_agnt) => Rc::get_mut(&mut rc_agnt).unwrap(),
// //            Agent_c_(Rc(agnt)) => agnt,
//         }
//     }
// }

// impl Sender {
//     pub fn unwrap<T: Is_sender> (&mut self) -> T {
//         match self {
//             Sender::Agent_a_(rc_agnt) => Rc::get_mut(&mut rc_agnt).unwrap(),
// //            Agent_c_(Rc(agnt)) => agnt,
//         }
//     }
// }

pub enum Signal {
    Signal_1_,
    Signal_2_,
}

pub struct Channel<'a> {
    pub sender: Rc<&'a mut Sender<'a>>,
    pub receiver: Rc<&'a mut Receiver<'a>>,
    pub signal_sample: Signal,
}

impl<'a> Channel<'a> {
    fn new<'b, 'c, 'd: 'b + 'c, S, R> (sender: &'b Rc<S>, receiver: &'c Rc<R>, signal_sample: Signal) -> Channel<'d>
    where S: Is_sender<'b>,
          R: Is_receiver<'c>,
    {
        let ch = Channel {
            sender: Rc::new(sender.wrap_sender()),
            receiver: Rc::new(receiver.wrap_receiver()),
            signal_sample: signal_sample,
        };
        sender.add_out_channel(ch);
        receiver.add_in_channel(ch);
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



