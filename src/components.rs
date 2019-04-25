
use std::sync::{Arc, Mutex, Weak};
extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crate::operation::{RunMode, DeviceMode};
use crate::connectivity::{Acceptor, Generator};

pub mod multi_in_component;
pub mod multi_out_component;
pub mod single_in_component;
pub mod single_out_component;

pub struct OutSet<C, S>
where C: Acceptor<S> + Send + ?Sized,
      S: Send,
{
    pub target: Weak<Mutex<C>>,
    pub config: DeviceMode<ChannelsOutFFW<S>>,
    pub linker: Arc<Mutex<Linker<S>>>,
}

impl<C, S> OutSet<C, S>
where C: Acceptor<S> + Send + ?Sized,
      S: Send,
{
    pub fn config_channels(&mut self, mode: RunMode) {
        self.config = match self.target.upgrade().unwrap().mode() {
            RunMode::Idle => DeviceMode::Idle,
            RunMode::Feedforward => {
                let mut lnkr = self.linker.lock().unwrap();
                DeviceMode::Feedforward(
                    ChannelsOutFFW {
                        ch_ffw: match lnkr.mode() {
                            RunMode::Idle => lnkr.gen_pre(),
                            RunMode::Feedforward => lnkr.take_pre(),
                        }
                    }
                )
            },
        }
    }

    pub fn config_idle(&mut self) {
        self.config = DeviceMode::Idle;
        self.linker.lock().unwrap().config_idle();
    }
}

pub struct InSet<C, S>
where C: Generator<S> + Send + ?Sized,
      S: Send,
{
    pub target: Weak<Mutex<C>>,
    pub config: DeviceMode<ChannelsInFFW<S>>,
    pub linker: Arc<Mutex<Linker<S>>>,
}

impl<C, S> InSet<C, S>
where C: Generator<S> + Send + ?Sized,
      S: Send,
{
    pub fn config_channels(&mut self, mode: RunMode) {
        self.config = match self.target.upgrade().unwrap().mode() {
            RunMode::Idle => DeviceMode::Idle,
            RunMode::Feedforward => {
                let mut lnkr = self.linker.lock().unwrap();
                DeviceMode::Feedforward(
                    ChannelsOutFFW {
                        ch_ffw: match lnkr.mode() {
                            RunMode::Idle => lnkr.gen_post(),
                            RunMode::Feedforward => lnkr.take_post(),
                        }
                    }
                )
            },
        }
    }

    pub fn config_idle(&mut self) {
        self.config = DeviceMode::Idle;
        self.linker.lock().unwrap().config_idle();
    }
}

struct Linker<S: Send> {
    config: DeviceMode<LinksFFW<S>>,
}

impl<S: Send> Linker<S> {
    fn mode(&self) -> RunMode {
        RunMode::mode_from_device(self.config)
    }

    fn gen_pre(&mut self) -> CCSender<S> {
        let (tx, rx) = crossbeam_channel::unbounded();
        self.config = DeviceMode::Feedforward(LinksFFW {
            pre: None,
            post: Some(rx),
        });
        tx
    }

    fn take_pre(&mut self) -> CCSender<S> {
        match &mut self.config {
            DeviceMode::Idle => panic!("Linker is idle when take_pre!"),
            DeviceMode::Feedforward(lnks) => lnks.pre.take(),
        }
    }

    fn gen_post(&mut self) -> CCReceiver<S> {
        let (tx, rx) = crossbeam_channel::unbounded();
        self.config = DeviceMode::Feedforward(LinksFFW {
            pre: Some(tx),
            post: None,
        });
        rx
    }

    fn take_post(&mut self) -> CCReceiver<S> {
        match &mut self.config {
            DeviceMode::Idle => panic!("Linker is idle when take_post!"),
            DeviceMode::Feedforward(lnks) => lnks.post.take(),
        }   
    }

    fn config_idle(&mut self) {
        self.config = DeviceMode::Idle;
    }
    
}

struct LinksFFW<S: Send> {
    pub pre: Option<CCSender<S>>,
    pub post: Option<CCReceiver<S>>,
}

struct ChannelsOutFFW<S: Send> {
    pub ch_ffw: CCSender<S>,
}

struct ChannelsInFFW<S: Send> {
    pub ch_ffw: CCReceiver<S>,
}
