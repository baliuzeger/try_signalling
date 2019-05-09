use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Mutex, Arc, Weak};
use crate::connectivity::{Generator, Acceptor, PassiveAcceptor, ActiveAcceptor};
use crate::connectivity::s1_pre::{SingleInComponentS1Pre, FwdPreS1};
use crate::connectivity::s1_post::{SingleOutComponentS1Post, FwdPostS1};
use crate::operation::{Configurable, Runnable, RunningSet, Broadcast, PassiveDevice, RunMode};
use crate::operation::op_device::ConsecutivePassiveDevice;
use crate::components::Linker;

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
        RunMode::eq_mode(self.in_s1_pre.mode(),self.out_s1_post.mode())
    }
}

impl PassiveDevice for ConnectionS1 {}

impl Runnable for ConnectionS1 {
    type Confirm = Broadcast;
    type Report = ();

    fn run(&mut self, rx_confirm: CCReceiver<<Self as Runnable>::Confirm>, tx_report: CCSender<<Self as Runnable>::Report>) {
        <Self as ConsecutivePassiveDevice>::run(self, rx_confirm, tx_report);
    }
}

impl ConsecutivePassiveDevice for ConnectionS1 {
    fn respond(&self) {
        self.in_s1_pre.ffw_accepted().into_iter().for_each(|s| self.out_s1_post.feedforward(self.refine(s)));
    }
    
    fn running_passive_devices(&self) -> Vec<RunningSet<Broadcast, ()>> {
        self.out_s1_post.running_passive_devices()
    }
}

impl Acceptor<FwdPreS1> for ConnectionS1 {
    fn add(&mut self, pre: Weak<Mutex<dyn Generator<FwdPreS1>>>, linker: Arc<Mutex<Linker<FwdPreS1>>>) {
        self.in_s1_pre.add_target(pre, linker);
    }
}

impl Generator<FwdPostS1> for ConnectionS1 {
    fn add_active(&mut self, post: Weak<Mutex<dyn ActiveAcceptor<FwdPostS1>>>, linker: Arc<Mutex<Linker<FwdPostS1>>>) {
        self.out_s1_post.add_active_target(post, linker);
    }
    
    fn add_passive(&mut self, post: Weak<Mutex<dyn PassiveAcceptor<FwdPostS1>>>, linker: Arc<Mutex<Linker<FwdPostS1>>>) {
        self.out_s1_post.add_passive_target(post, linker);
    }
}

impl ConnectionS1 {
    pub fn new(value: i32) -> Arc<Mutex<ConnectionS1>> {
        Arc::new(Mutex::new(ConnectionS1 {
            in_s1_pre: SingleInComponentS1Pre::new(),
            out_s1_post: SingleOutComponentS1Post::new(),
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
