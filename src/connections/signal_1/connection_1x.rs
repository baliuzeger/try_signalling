extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Mutex, Arc, Weak};
use crate::connections::{PassiveConnection};
use crate::connections::signal_1::{S1Acceptor, S1Generator};
use crate::connections::signal_1::{FwdPreS1, FwdPostS1};
use crate::connections::signal_1::{ConnectionComponentS1};
use crate::supervisor::{RunMode};

pub struct Model {
    module: ConnectionComponentS1,
    value: i32,
}

impl PassiveConnection<FwdPreS1, FwdPostS1> for Model {
    fn mode(&self) -> RunMode {
        self.module.mode()
    }

    fn config_run(&mut self, mode: RunMode) {
        self.module.config_run(mode);
    }
    
    fn config_idle(&mut self) {
        self.module.config_idle();
    }
    
    fn propagate(&self) {
        self.module.export(self.refine(self.module.import()));
    }

    fn set_pre_channel_ffw(&mut self, channel: Option<CCReceiver<FwdPreS1>>) {
        self.module.set_pre_channel_ffw(channel);
    }
    
    fn set_post_channel_ffw(&mut self, channel: Option<CCSender<FwdPostS1>>) {
        self.module.set_post_channel_ffw(channel);        
    }
}

impl Model {
    pub fn new<G, A>(pre: Weak<Mutex<G>>, post: Weak<Mutex<A>>, value: i32) -> Arc<Mutex<Model>>
    where G:'static + S1Generator + Send,
          A:'static + S1Acceptor + Send
    {
        Arc::new(Mutex::new(Model {
            module: ConnectionComponentS1::new(pre, post),
            value,
        }))
    }

    fn refine(&self, s: FwdPreS1) -> FwdPostS1 {
        FwdPostS1 {
            msg_gen: s.msg_gen,
            msg_prop: self.value,
        }
    }
}
