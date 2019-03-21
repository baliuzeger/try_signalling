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

pub struct BkwdPreS1 {
    pub msg_gen: i32
}

pub struct BkwdPostS1 {
    pub msg_gen: i32,
    pub msg_prop: i32,
}

pub struct PreAgentModuleS1 {
    config: RunMode<AgentModuleIdle<dyn Propagate1 + Send>,
                    PreAgentModuleFFW<dyn Propagate1 + Send, FwdPreS1>>
}

pub struct PostAgentModuleS1 {
    config: RunMode<AgentModuleIdle<dyn Propagate1 + Send>,
                    PostAgentModuleFFW<dyn Propagate1 + Send, FwdPostS1>>
}

pub struct ConnectionModuleS1<G: S1Generator + Send, A: S1Acceptor + Send> {
    config: RunMode<ConnectionModuleIdle<G, A>,
                    ConnectionModuleFFW<G, A, FwdPreS1, FwdPostS1>>
}

impl<G, A, R, S> ConnectionModuleS1<G, A>
where G: S1Generator + Send,
      A: S1Acceptor + Send,
{

    fn config_ffw(&mut self, , post_channel: ) {
        match &self.config {
            RunMode::Idle(m) => self.config = RunMode::Feedforward(m.make_ffw(pre_channel, post_channel)),
            _ => panic!("call fn config_feedforward when not RunMode::Idle!"),
        }
    }

    fn set_pre_ffw(&mut self, pre_channel: Option<CCReceiver<FwdPreS1>>) {
        match &self.config {
            RunMode::Feedforward(m) => m.set_pre_channel(pre_channel),
            _ => panic!("call fn set_pre_ffw when not RunMode::Feedforward!")
        }
    }

    fn set_post_ffw(&mut self, post_channel: Option<CCSender<FwdPostS1>>) {
        match &self.config {
            RunMode::Feedforward(m) => m.set_post_channel(post_channel),
            _ => panic!("call fn set_post_ffw when not RunMode::Feedforward!")
        }
    }
    
    fn config_idle(&mut self) {
        match &self.config {
            RunMode::Feedforward(m) => self.config = RunMode::Idle(m.make_idle()),
            RunMode => panic!("call fn config_idle when RunMode::Idle!"),
        }
    }

    fn import(&mut self) {
        match &self.config {
            RunMode::Feedforward(m) => m.import();
            RunMode => panic!("call fn import when RunMode::Idle!"),
        }
    }

    fn export(&self, s: FwdPostS1) {
        match &self.config {
            RunMode::Feedforward(m) => m.export();
            RunMode => panic!("call fn export when RunMode::Idle!"),
        }
    }    
}

impl PreAgentModuleS1 {
    fn config_feedforward(&mut self) {
        match &self.config {
            RunMode::Idle(m) => self.config = RunMode::Feedforward(),
            _ => panic!("call fn config_feedforward when not RunMode::Idle!"),
        }
    }
    
    fn config_idle(&mut self) {
        match &self.config {
            RunMode::Feedforward(m) => self.config = RunMode::Idle(),
            RunMode => panic!("call fn config_idle when RunMode::Idle!"),
        }
    }

    fn feedforward(&self, s: FwdPostS1) {
        match self {
            RunMode::FeedForward(v) => {
                v.iter().map(|set| set.send(s)).collect();
            }
            _ => panic!("PreAgentmodules1 is not Feedforward when feedforward called!");
        }
    }
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
