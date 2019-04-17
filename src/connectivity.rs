use std::sync::{Arc, Mutex, Weak};
pub trait Generator<S: Send> {

}

pub trait Acceptor<S: Send> {

}

pub trait MultiOut<S: Send>: Generator<S> {
    fn plug_to_passive<C> (&mut self, target: Weak<Mutex<C>>)
    where C: 'static + Acceptor<S> + Send;
    fn plug_from_passive<C> (&mut self, target: Weak<Mutex<C>>)
    where C: 'static + Acceptor<S> + Send;
    // fn add_out_active<T: 'static + ActivePropagator + Send> (&mut self, connection: Weak<Mutex<T>>, channel: CCSender<Signal1Gen>);    
}

pub trait MultiIn<S: Send>: Acceptor<S> {
    fn plug_to<C> (&mut self, source: Weak<Mutex<C>>)
    where C: 'static + Generator<S> + Send;
    fn plug_from<C> (&mut self, source: Weak<Mutex<C>>)
    where C: 'static + Generator<S> + Send;   
}

pub trait SingleOut<S>: Generator<S> {

}

pub trait SingleIn<S>: Acceptor<S> {

}

pub trait OptionOut<S>: Generator<S> {

}

pub trait OptionIn<S>: Acceptor<S> {

}
