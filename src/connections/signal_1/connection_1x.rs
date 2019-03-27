extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Mutex, Arc, Weak};
use crate::connections::{PassiveConnection};
use crate::connections::signal_1::{S1Acceptor, S1Generator};
use crate::connections::signal_1::{FwdPreS1, FwdPostS1};
use crate::connections::signal_1::{ConnectionComponentS1};
use crate::supervisor::{RunMode};
use crate::agent_populations::{AgentPopulation, HoldAgents};

pub struct Model<G, A>
where G: S1Generator + Send,
      A: S1Acceptor + Send
{
    module: ConnectionComponentS1<G, A>,
    value: i32,
}

impl<G, A> PassiveConnection<FwdPreS1, FwdPostS1> for Model<G, A>
where G: S1Generator + Send,
      A: S1Acceptor + Send
{
    fn mode(&self) -> RunMode {
        // println!("connection1x mode: {:?}.", self.module.mode());
        self.module.mode()
    }

    fn config_run(&mut self, mode: RunMode) {
        // println!("connection_1x config_run.");
        self.module.config_run(mode);
    }
    
    fn config_idle(&mut self) {
        self.module.config_idle();
    }
    
    fn propagate(&self) {
        self.module.export(self.refine(self.module.import()));
    }

    fn set_pre_channel_ffw(&mut self, channel: Option<CCReceiver<FwdPreS1>>) {
        // println!("connection_1x setting pre_channel.");
        self.module.set_pre_channel_ffw(channel);
    }
    
    fn set_post_channel_ffw(&mut self, channel: Option<CCSender<FwdPostS1>>) {
        // println!("connection_1x setting post_channel.");
        self.module.set_post_channel_ffw(channel);        
    }
}

impl<G: S1Generator + Send, A: S1Acceptor + Send> Model<G, A> {
    pub fn new(pre: Weak<Mutex<G>>, post: Weak<Mutex<A>>, value: i32) -> Arc<Mutex<Model<G, A>>>
    where G:'static + S1Generator + Send,
          A:'static + S1Acceptor + Send
    {
        let conn = Arc::new(Mutex::new(Model {
            module: ConnectionComponentS1::new(pre.clone(), post.clone()),
            value,
        }));
        pre.upgrade().unwrap().lock().unwrap().add_out_passive_s1(Arc::downgrade(&conn));
        post.upgrade().unwrap().lock().unwrap().add_in_s1(Arc::downgrade(&conn));
        conn
    }

    pub fn new_on_populations<P1, P2>(value: i32, p1: &Arc<Mutex<P1>>, n1: usize, p2: &Arc<Mutex<P2>>, n2: usize) -> Arc<Mutex<Model<G, A>>>
    where G:'static + S1Generator + Send,
          A:'static + S1Acceptor + Send,
          P1: HoldAgents<G>,
          P2: HoldAgents<A>,
    {
        let ag1 = Arc::downgrade(&p1.lock().unwrap().agent_by_id(n1));
        let ag2 = Arc::downgrade(&p2.lock().unwrap().agent_by_id(n2));
        Model::new(ag1, ag2, value)
    }

    fn refine(&self, s: FwdPreS1) -> FwdPostS1 {
        FwdPostS1 {
            msg_gen: s.msg_gen,
            msg_prop: self.value,
        }
    }
}
