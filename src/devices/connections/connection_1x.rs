use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Mutex, Arc, Weak};
use crate::connectivity::s1_pre::SingleInComponentS1Pre;
use crate::connectivity::s1_post::SingleOutComponentS1Post;
use crate::operation::{Configurable, Runnable, RunningSet, Broadcast};
use crate::operation::op_device::ConsecutivePassiveDevice;

pub struct ConnectionS1 {
    in_s1_pre: SingleInComponentS1Pre,
    out_s1_post: SingleOutComponentS1Post,
    value: i32,
}

impl Configurable for ConnectionS1 {
    fn config_mode(&mut self, mode: RunMode) {
        self.in_s1_pre.config_mode(mode);
        self.out_s1_post.config_mode(mode);
    }
    
    fn config_channels(&mut self) {
        self.in_s1_pre.config_channels();
        self.out_s1_post.config_channels();   
    }

    fn mode(&self) -> RunMode {
        RunMode::eq_mode(self.in_s1_post.mode(),self.out_s1_pre.mode())
    }
}

impl ConsecutivePassiveDevice for ConnectionS1 {
    fn respond(&self) {
        self.out_s1_post.feedforward(self.refine(self.in_s1_pre.ffw_accepted()));
    }
    
    fn running_passive_devices(&self) -> Vec<RunningSet<Broadcast, ()>> {
        
    }
}

impl<G, A> PassiveConnection<FwdPreS1, FwdPostS1> for ConnectionS1<G, A>
where G: Generator<FwdPreS1, FwdPostS1> + Send,
      A: Acceptor<FwdPreS1, FwdPostS1> + Send
{

    
    fn propagate(&self) {
        self.module.export(self.refine(self.module.import()));
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
