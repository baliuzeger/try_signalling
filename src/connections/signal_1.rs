extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Mutex, Arc, Weak};
use crate::supervisor::RunMode;
use crate::agents::{AgentIdleModule, PreAgentFwdModule, PostAgentFwdModule};
use crate::connections::{ConnectionIdleModule, ConnectionFwdModule};

pub mod connection_1x;

pub struct FwdPreS1 {
    pub msg_gen: i32
}

pub struct FwdPostS1 {
    pub msg_gen: i32,
    pub msg_prop: i32,
}

pub struct BkwdPreS1 {
    pub msg_gen: i32
}

pub struct BkwdPostS1 {
    pub msg_gen: i32,
    pub msg_prop: i32,
}

pub struct PreAgentModuleS1 {
    config: RunMode<AgentIdleModule<dyn Propagate1 + Send>,
                    PreAgentFwdModule<dyn Propagate1 + Send, FwdPreS1>>
}

pub struct PostAgentModuleS1 {
    config: RunMode<AgentIdleModule<dyn Propagate1 + Send>,
                    PostAgentFwdModule<dyn Propagate1 + Send, FwdPostS1>>
}

pub struct ConnectionModuleS1<G: S1Generator + Send, A: S1Acceptor + Send> {
    config: RunMode<ConnectionIdleModule<G, A>,
                    ConnectionFwdModule<G, A, FwdPreS1, FwdPostS1>>
}

impl PreAgentModuleS1 {
    fn feedforward(&self, s: FwdPostS1) {
        match self {
            RunMode::FeedForward(v) => {
                v.iter().map(|set| set.send(s)).collect();
            }
            _ => panic!("PreAgentmodules1 is not Feedforward when feedforward called!");
        }
    }
}


pub enum ConnectionModuleS1<S: Generate1 + Send, R: Process1 + Send> {
    Idle{
        pre: Arc<Mutex<S>>,
        post: Arc<Mutex<R>>,
    },
    FeedForward{
        pre: FwdInSet<FwdPreS1, Arc<Mutex<S>>>,
        post: FwdOutSet<FwdPostS1, Arc<Mutex<R>>>
    },
}



pub trait PassivePropagator: PassiveConnection + Propagator {}

pub trait S1Generator {
    fn generate_s1(&self);
    fn add_out_passive<T: 'static + PassivePropagator + Send> (&mut self, connection: Weak<Mutex<T>>, channel: CCSender<Signal1Gen>);
    // fn add_out_active<T: 'static + ActivePropagator + Send> (&mut self, connection: Weak<Mutex<T>>, channel: CCSender<Signal1Gen>);
}

pub trait S1Propagator {
    fn refine(&self, s: Signal1Gen) -> Signal1Prop;
    fn propagate(&self, s: Signal1Prop);
}

pub trait S1Acceptor {
    fn process_s1(&self);
    fn add_in<T: 'static + Propagate1 + Send> (&mut self, connection: Weak<Mutex<T>>, channel: CCReceiver<Signal1Prop>);
}
