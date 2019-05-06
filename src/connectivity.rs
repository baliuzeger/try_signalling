use std::sync::{Mutex, Weak};
use crate::operation::passive_device::{PassiveDevice, FiringDevice};
extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;

pub mod s1_pre;
pub mod s1_post;
// pub mod signal_2;

pub trait Generator<S: Send> {
}

pub trait Acceptor<S: Send> {
}

pub trait PassiveAcceptor<S: Send>: Acceptor<S> + PassiveDevice {}

pub trait ActiveAcceptor<S: Send>: Acceptor<S> + ActiveDevice {} // but how about firingdevice?



pub trait Leader<S: Send> {
    fn add_responder<A: Responder<S>>(&mut self, post: Arc<Mutex<A>>);
    fn add_evolver<A: Evolver<S>>(&mut self, post: Arc<Mutex<A>>);
}

pub trait Doer<S: Send> {
    fn add_evolver<A: Evolver<S>>(&mut self, post: Arc<Mutex<A>>);
}

pub trait Evolver<S: Send> {
    fn add<G: Generator<S>>(&mut self, post: Arc<Mutex<G>>);
}

pub trait Responder<S: Send> {
    fn add<G: Generator<S>>(&mut self, post: Arc<Mutex<G>>);
}
