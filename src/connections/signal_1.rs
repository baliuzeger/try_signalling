extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Mutex, Arc, Weak};
use crate::supervisor::RunMode;
use crate::agents::{AgentModuleIdle, PreAgentModuleFFW, PostAgentModuleFFW};
use crate::connections::{ConnectionModuleIdle, ConnectionModuleFFW};

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

pub trait S1PassivePropagator: PassiveConnection + S1Propagator {}

pub trait S1Generator {
    fn add_out_passive_s1<T: 'static + S1PassivePropagator + Send> (&mut self, connection: Weak<Mutex<T>>, channel: CCSender<Signal1Gen>);
    // fn add_out_active<T: 'static + ActivePropagator + Send> (&mut self, connection: Weak<Mutex<T>>, channel: CCSender<Signal1Gen>);
}

pub trait S1Propagator {
}

pub trait S1Acceptor {
    fn add_in_s1<T: 'static + S1PassivePropagator + Send> (&mut self, connection: Weak<Mutex<T>>, channel: CCReceiver<Signal1Prop>);
}

type PreAgentComponentS1 = PreComponent<dyn S1PassivePropagator + Send, FwdPreS1>;
type PostAgentComponentS1 = PostComponent<dyn S1PassivePropagator + Send, FwdPostS1>;
type ConnectionComponentS1<'a, 'b> = ConnectionComponent<'a S1Generator + Send, 'b S1Acceptor + Send>
