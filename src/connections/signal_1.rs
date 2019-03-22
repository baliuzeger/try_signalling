extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crossbeam_channel::TryIter as CCTryIter;
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
    config: RunMode<AgentModuleIdle<dyn S1PassivePropagator + Send>,
                    PreAgentModuleFFW<dyn S1PassivePropagator + Send, FwdPreS1>>
}

pub struct PostAgentModuleS1 {
    config: RunMode<AgentModuleIdle<dyn S1PassivePropagator + Send>,
                    PostAgentModuleFFW<dyn S1PassivePropagator + Send, FwdPostS1>>
}

pub struct ConnectionModuleS1<G: S1Generator + Send, A: S1Acceptor + Send> {
    config: RunMode<ConnectionModuleIdle<G, A>,
                    ConnectionModuleFFW<G, A, FwdPreS1, FwdPostS1>>
}

impl<G, A, R, S> ConnectionModuleS1<G, A>
where G: S1Generator + Send,
      A: S1Acceptor + Send,
{
    fn mode(&self) -> RunMode<bool, bool> {
        match self.config {
            RunMode::Idle(_) => RunMode::Idle(True),
            RunMode::Feedforward(_) => RunMode::Feedforward(True),
        }
    }
    
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
    fn new() -> PreAgentModuleS1 {
        PreAgentModuleS1 {
            config: RunMode::Idle(AgentModuleIdle::<dyn S1PassivePropagator + Send>:new()),
        }
    }
    
    fn add_connection(&mut self, connection: Weak<Mutex<dyn S1PassivePropagator + Send>>) {
        match &mut self.config {
            RunMode::Idle(m) => m.add_conection(connection), 
            _ => panic!("can only add_conntion when RunMode::Idle!"),
        }
    }

    fn config_run(&mut self, mode: RunMode<bool, bool>) {
        match (mode, &self.config) {
            (RunMode::Idle(_), _) => println!("config_run for mode Idle, no effect."),
            (mi, RunMode::Idle(ms)) => self.config = RunMode::Feedforward(ms.make_ffw_pre()),
            (_, _) => panic!("call fn config_run when not RunMode::Idle!"),
        }
    }
    
    fn config_idle(&mut self) {
        match &self.config {
            RunMode::Feedforward(m) => self.config = RunMode::Idle(),
            RunMode => panic!("call fn config_idle when RunMode::Idle!"),
        }
    }

    fn feedforward(&self, s: FwdPostS1) {
        match &self {
            RunMode::FeedForward(m) => m.feeddorward(s),
            _ => panic!("PreAgentmodules1 is not Feedforward when feedforward called!");
        }
    }
}

impl PostAgentModuleS1 {
    fn new() -> PostAgentModuleS1 {
        PostAgentModuleS1 {
            config: RunMode::Idle(AgentModuleIdle::<dyn S1Propagator + Send>:new()),
        }
    }

    pub fn ffw_accepted(&self) -> Vec<FwdPreS1> {
        match &mut self {
            RunMode::Feedforward(m) => m.accepted(),
            RunMode::Idle(_) => panic!("PostAgentModuleS1 is Idle when .accepted called!"),
        }
    }
    
    pub fn add_connection(&mut self, connection: Weak<Mutex<dyn S1Propagator + Send>>) {
        match &mut self.config {
            RunMode::Idle(m) => m.add_conection(connection), 
            _ => panic!("can only add_conntion when RunMode::Idle!"),
        }
    }

    pub fn config_run(&mut self, mode: RunMode<bool, bool>) {
        match (mode, &self.config) {
            (RunMode::Idle(_), _) => println!("config_run for mode Idle, no effect."),
            (mi, RunMode::Idle(ms)) => self.config = RunMode::Feedforward(ms.make_ffw_post()),
            (_, _) => panic!("call fn config_run when not RunMode::Idle!"),
        }
    }
}

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
