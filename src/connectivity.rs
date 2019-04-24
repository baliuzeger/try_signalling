use std::sync::{Arc, Mutex, Weak};
pub trait Generator<S: Send> {
    fn set_channel_ffw(&mut self, channel: Option<CCSender<Signal>>);
}

pub trait Acceptor<S: Send> {
    fn set_channel_ffw(&mut self, channel: Option<CCSender<Signal>>);
}

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

pub trait SingleOut<S>: Generator<S> {
    // not needed by "Connector" formulation.
    fn set_channel_ffw(&mut self, channel: Option<CCSender<Signal>>);
}

pub trait SingleIn<S>: Acceptor<S> {
    // not needed by "Connector" formulation.
    fn set_channel_ffw(&mut self, channel: Option<CCReceiver<Signal>>);
}

pub trait OptionOut<S>: Generator<S> {

}

pub trait OptionIn<S>: Acceptor<S> {

}
