use std::sync::{Mutex, Weak};
use crate::operation::passive_device::{PassiveDevice, FiringDevice};
extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;

pub mod s1_pre;
pub mod s1_post;
// pub mod signal_2;

pub trait Generator<S: Send> {
    fn add_active<A: ActiveAcceptor<S>>(&mut self, post: Arc<Mutex<A>>);
    fn add_passive<A: PassiveAcceptor<S>>(&mut self, post: Arc<Mutex<A>>);
}

pub trait Acceptor<S: Send> {
    fn add<G: Generator<S>>(&mut self, pre: Arc<Mutex<G>>);
}

pub trait ActiveAcceptor<S: Send>: Acceptor<S> + ActiveDevice {}

impl<S, A> ActiveAcceptor<S> for A
where S: Send,
      A: Acceptor<S> + ActiveDevice,
{}


// Passive and has only 1 input channel, 1 type of input signal.
pub trait PassiveAcceptor: Acceptor<<Self as PassiveAcceptor>::Signal> + PassiveDevice {
    type Signal: Send;
}


