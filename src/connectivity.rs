use std::sync::{Mutex, Weak};
use crate::operation::passive_device::PassiveDevice;
extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;

pub trait Generator<S: Send> {
    fn set_channel_ffw(&mut self, channel: Option<CCSender<S>>);
}

pub trait Acceptor<S: Send> {
    fn set_channel_ffw(&mut self, channel: Option<CCSender<S>>);
}

pub trait PassiveAcceptor<S: Send>: Acceptor<S> + PassiveDevice {}


pub trait MultiOut<S: Send>: Generator<S> {
    fn plug_to_passive<C> (&mut self, target: Weak<Mutex<C>>)
    where C: 'static + Acceptor<S> + Send;
    fn plug_from_passive<C> (&mut self, target: Weak<Mutex<C>>)
    where C: 'static + Acceptor<S> + Send;
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
    // not needed by "Connector" formulation.
    fn set_channel_ffw(&mut self, channel: Option<CCSender<S>>);
}

pub trait SingleIn<S: Send>: Acceptor<S> {
    // not needed by "Connector" formulation.
    fn set_channel_ffw(&mut self, channel: Option<CCReceiver<S>>);
}

pub trait OptionOut<S: Send>: Generator<S> {

}

pub trait OptionIn<S: Send>: Acceptor<S> {

}
