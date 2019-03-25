extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Mutex, Arc, Weak};
use crate::connections::{PassiveConnection};
use crate::connections::signal_1::{ConnectionModuleS1, S1PassivePropagator, S1Propagator};
use crate::connections::signal_1::{FwdPreS1, FwdPostS1};
use crate::supervisor::RunMode;


pub struct Model<G: S1Generator + Send, A: S1Acceptor + Send> {
    module: ConnectionModuleS1<G, A>
    value: i32,
}

impl<G: S1Generator + Send, A: S1Acceptor + Send> S1PassivePropagator for Model<G, A> {}

impl<G: S1Generator + Send, A: S1Acceptor + Send> S1Propagator for Model<G, A> {}

impl<G: S1Generator + Send, A: S1Acceptor + Send> PassiveConnection for Model<G, A> {
    fn mode(&self) -> RunMode {
        RunMode::variant(self.module.mode());
    }

    fn config_run(&mut self, mode: RunMode<bool, bool>) {
        self.module.config_run(mode);
    }
    
    fn config_idle(&mut self) {
        self.module.config_idle();
    }
    
    fn propagate(&self) {
        self.module.export(self.refine(self.module.import()));
    }
}

impl<G: S1Generator + Send, A: S1Acceptor + Send> Model<G, A> {
    pub fn new(pre: Weak<Mutex<G>>, post: Weak<Mutex<A>>, value: i32) -> Arc<Mutex<Model<G, A>>>
    where G:'static + S1Generator + Send,
          A:'static + S1Acceptor + Send
    {
        Arc::new(Mutex::new(Model {
            module: ConnectionModuleS1::new(pre, post),
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
