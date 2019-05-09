use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Mutex, Arc, Weak};
use crate::connectivity::s1_pre::SingleInComponentS1Pre;
use crate::connectivity::s1_post::SingleOutComponentS1Post;
use crate::operation::{Configurable, Runnable};

pub struct ConnectionS1 {
    in_s1_pre: SingleInComponentS1Pre,
    out_s1_post: SingleOutComponentS1Post,
    value: i32,
}

impl Configurable

impl<G, A> PassiveConnection<FwdPreS1, FwdPostS1> for ConnectionS1<G, A>
where G: Generator<FwdPreS1, FwdPostS1> + Send,
      A: Acceptor<FwdPreS1, FwdPostS1> + Send
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

impl<G: Generator<FwdPreS1, FwdPostS1> + Send, A: Acceptor<FwdPreS1, FwdPostS1> + Send> ConnectionS1<G, A> {
    pub fn new(pre: Weak<Mutex<G>>, post: Weak<Mutex<A>>, value: i32) -> Arc<Mutex<ConnectionS1<G, A>>>
    where G:'static + Generator<FwdPreS1, FwdPostS1> + Send,
          A:'static + Acceptor<FwdPreS1, FwdPostS1> + Send
    {
        let conn = Arc::new(Mutex::new(ConnectionS1 {
            module: ConnectionComponentS1::new(pre.clone(), post.clone()),
            value,
        }));
        pre.upgrade().unwrap().lock().unwrap().add_out_passive(Arc::downgrade(&conn));
        post.upgrade().unwrap().lock().unwrap().add_in(Arc::downgrade(&conn));
        conn
    }

    pub fn new_on_populations<P1, P2>(value: i32, p1: &Arc<Mutex<P1>>, n1: usize, p2: &Arc<Mutex<P2>>, n2: usize) -> Arc<Mutex<ConnectionS1<G, A>>>
    where G:'static + Generator<FwdPreS1, FwdPostS1> + Send,
          A:'static + Acceptor<FwdPreS1, FwdPostS1> + Send,
          P1: HoldAgents<G>,
          P2: HoldAgents<A>,
    {
        let ag1 = Arc::downgrade(&p1.lock().unwrap().agent_by_id(n1));
        let ag2 = Arc::downgrade(&p2.lock().unwrap().agent_by_id(n2));
        ConnectionS1::new(ag1, ag2, value)
    }

    fn refine(&self, s: FwdPreS1) -> FwdPostS1 {
        FwdPostS1 {
            msg_gen: s.msg_gen,
            msg_prop: self.value,
        }
    }
}
