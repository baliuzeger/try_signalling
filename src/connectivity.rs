use std::sync::{Mutex, Weak, Arc};
use crate::operation::{PassiveDevice, ActiveDevice};
use crate::components::Linker;

pub mod s1_pre;
pub mod s1_post;
// pub mod signal_2;

pub trait Generator<S: Send> {
    fn add_active<A: ActiveAcceptor<S>>(&mut self, post: Weak<Mutex<A>>, linker: Arc<Mutex<Linker<S>>>);
    fn add_passive<A: PassiveAcceptor<S>>(&mut self, post: Weak<Mutex<A>>, linker: Arc<Mutex<Linker<S>>>);
}

pub trait Acceptor<S: Send> {
    fn add<G: Generator<S>>(&mut self, pre: Weak<Mutex<G>>, linker: Arc<Mutex<Linker<S>>>);
}

pub trait ActiveAcceptor<S: Send>: ActiveDevice + Acceptor<S> {}

impl<S, A> ActiveAcceptor<S> for A
where S: Send,
      A: Acceptor<S> + ActiveDevice,
{}


// Passive and has only 1 input channel, 1 type of input signal.
pub trait PassiveAcceptor<S: Send>: PassiveDevice + Acceptor<S> {}

impl<S, A> PassiveAcceptor<S> for A
where S: Send,
      A: Acceptor<S> + PassiveDevice,
{}


pub fn connect_active<G, A, S> (pre: Arc<Mutex<G>>, post: Arc<Mutex<A>>)
where G: Generator<S>,
      A: ActiveAcceptor<S>,
      S: Send,
{
    let linker = Linker::new();
    pre.lock().unwrap().add_active(Arc::downgrade(&post), Arc::clone(&linker));
    post.lock().unwrap().add(Arc::downgrade(&pre), linker);
}
