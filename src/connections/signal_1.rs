use std::sync::{Mutex, Weak};
use crate::connections::{PassiveConnection};
use crate::connection_component::{ConnectionComponent};
use crate::agent_components::pre_component::{PreComponent};
use crate::agent_components::post_component::{PostComponent};

pub mod connection_1x;

pub struct FwdPreS1 {
    pub msg_gen: i32
}

pub struct FwdPostS1 {
    pub msg_gen: i32,
    pub msg_prop: i32,
}

// pub struct BkwdPreS1 {
//     pub msg_gen: i32
// }

// pub struct BkwdPostS1 {
//     pub msg_gen: i32,
//     pub msg_prop: i32,
// }

// pub trait S1PassivePropagator: PassiveConnection + S1Propagator {}

// pub trait S1Propagator {
// }

pub trait S1Generator {
    fn add_out_passive_s1 (&mut self, connection: Weak<Mutex<dyn S1PassivePropagator + Send>>);
    // fn add_out_active<T: 'static + ActivePropagator + Send> (&mut self, connection: Weak<Mutex<T>>, channel: CCSender<Signal1Gen>);
}

pub trait S1Acceptor {
    fn add_in_s1 (&mut self, connection: Weak<Mutex<dyn S1PassivePropagator + Send>>);
}

pub type PreAgentComponentS1 = PreComponent<dyn S1PassivePropagator + Send, FwdPreS1>;
pub type PostAgentComponentS1 = PostComponent<dyn S1PassivePropagator + Send, FwdPostS1>;
pub type ConnectionComponentS1 = ConnectionComponent<'static + S1Generator + Send, 'static + S1Acceptor + Send, FwdPreS1, FwdPostS1>;
