/// functionality needed:
/// 1. a channel should not connect from/to an identical agent.
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use crate::random_sleep;
use crate::supervisor::{RunMode, Broadcast};

pub mod signal_1;
// pub mod signal_2;

pub struct RunningPassiveConnection {
    pub instance: JoinHandle<()>,
    pub report: CCReceiver<bool>,
    pub confirm: CCSender<Broadcast>,
}

impl RunningPassiveConnection {
    pub fn new<T>(device: Arc<Mutex<T>>) -> RunningPassiveConnection
    where T: 'static + PassiveConnection + Send + ?Sized
    {
        // for strict ordering of agent-connection_prop, bounded(1) is chosen.
        let (tx_report, rx_report) = crossbeam_channel::bounded(1);
        let (tx_confirm, rx_confirm) = crossbeam_channel::bounded(1);
        RunningPassiveConnection {
            instance: thread::spawn(move || {device.lock().unwrap().run(rx_confirm, tx_report)}),
            report: rx_report,
            confirm: tx_confirm,
        }
    }    
}

pub trait PassiveConnection {
    fn mode(&self) -> RunMode;

    fn propagate(&self);
    
    fn run(&self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<bool>){
        loop {
            random_sleep();
            match rx_confirm.recv().unwrap() {
                Broadcast::Exit => break,
                Broadcast::NewCycle => panic!("agent confirm by NewCycle!"),
                Broadcast::FinishCycle => {
                    // println!("conn wait recv signal.");
                    self.propagate();
                    // println!("conn got & propagated signal.");
                    tx_report.send(true).unwrap();
                }
            }
        }
    }
}

pub struct ConnectionModuleIdle<G: Send, A: Send> {
    pre: Weak<Mutex<G>>,
    post: Weak<Mutex<A>>,
}

impl<G: Send, A: Send> ConnectionModuleIdle<G, A> {
    fn make_ffw<R, S>(&self) -> ConnectionModuleFFW<G, A, R, S>
    where R: Send,
          S: Send
    {
        ConnectionModuleFFW {
            pre: Weak::clone(self.pre),
            post: Weak::clone(self.post),
            pre_channel: None,
            post_channel: None,
            buffer: Vec::new(),
        }
    }
}

pub struct ConnectionModuleFFW<G: Send, A: Send, R: Send, S: Send> {
    pre: Weak<Mutex<G>>,
    post: Weak<Mutex<A>>,
    pre_channel: Option<CCReceiver<R>>,
    post_channel: Option<CCSender<S>>,
}

impl<G: Send, A: Send, R, S> ConnectionModuleFFW<G, A, R, S> {
    fn make_idle(&self) -> ConnectionModuleIdle<G, A> {
        ConnectionModuleIdle {
            pre: Weak::clone(self.pre),
            post: Weak::clone(self.post),
        }
    }

    fn set_pre_channel(&mut self, pre_channel: Option<CCReceiver<R>>) {
        self.pre_channel = pre_channel;
    }

    fn set_post_channel(&mut self, post_channel: Option<CCSender<S>>) {
        self.post_channel = post_channel;
    }
    
    fn import(&mut self) {
        self.buffer.push(m.pre_channel.unwrap().recv().unwrap());
    }

    fn export(&self, s: S) {
        self.post_channel.uwrap().send(s).unwrap(),
    }
}
