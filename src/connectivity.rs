use std::sync::{Mutex, Weak};
use crate::operation::passive_device::PassiveDevice;
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


pub trait MultiOut<S: Send>: Generator<S> {
    fn plug_to_passive<C> (&mut self, target: Weak<Mutex<C>>)
    where C: 'static + PassiveAcceptor<S> + Send;
    fn plug_from_passive<C> (&mut self, target: Weak<Mutex<C>>)
    where C: 'static + PassiveAcceptor<S> + Send;
    fn plug_to_active<C> (&mut self, target: Weak<Mutex<C>>)
    where C: 'static + Acceptor<S> + Send;
    fn plug_from_active<C> (&mut self, target: Weak<Mutex<C>>)
    where C: 'static + Acceptor<S> + Send;
}

pub trait MultiIn<S: Send>: Acceptor<S> {
    fn plug_to<C> (&mut self, source: Weak<Mutex<C>>)
    where C: 'static + Generator<S> + Send;
    fn plug_from<C> (&mut self, source: Weak<Mutex<C>>)
    where C: 'static + Generator<S> + Send;   
}

pub trait SingleOut<S: Send>: Generator<S> {
}

pub trait SingleIn<S: Send>: Acceptor<S> {
}

pub trait OptionOut<S: Send>: Generator<S> {

}

pub trait OptionIn<S: Send>: Acceptor<S> {

}
