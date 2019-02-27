// use std::rc::Rc;
// use crate::generators::{Generator_1, Generator_2};
// use crate::processors::{Processor_1, Processor_2};
use crate::signals::{Signal_1, Signal_2, Sender, Receiver, Channel};
// use crate::signals::{Signal_1, Signal_2};

pub mod agent_a;
// pub mod agent_b;

pub trait Process_1 {
    fn process_1(&mut self, s: Signal_1);
}

pub trait Process_2 {
    fn process_2(&mut self, s: Signal_2);
}

pub trait Is_sender<'a> {
    fn wrap_sender(self) -> Sender<'a>;
    fn add_out_channel(&mut self, ch: Channel);
}

pub trait Is_receiver<'a> {
    fn wrap_receiver(self) -> Receiver<'a>;
    fn add_in_channel(&mut self, ch: Channel);
}
