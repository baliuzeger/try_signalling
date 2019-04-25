
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
    pub channels: DeviceMode<ChannelsOutFFW<S>>,
    pub linker: Arc<Mutex<Linker<S>>>,
}

impl<C, S> OutSet<C, S>
where C: Acceptor<S> + Send + ?Sized,
      S: Send,
{
    pub fn config_run(&mut self, mode: RunMode) {
        self.linker.lock().unwrap().pre_mode = mode;
    }

    pub fn config_channels(&mut self) {
        let mut lnkr = self.linker.lock().unwrap();
        self.channels = link.make_pre();
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
    pub pre_mode: RunMode,
    pub post_mode: RunMode,
    channels: DeviceMode<TmpFFW<S>>,
}

impl<S: Send> Linker<S> {
    fn channels_mode(&self) -> RunMode {
        RunMode::mode_from_device(self.channels)
    }

    fn make_pre(&mut self) -> DeviceMode<ChannelsOutFFW<S>> {
        match self.pre_mode() {
            RunMode::Idle => {
                self.config_idle();
                DeviceMode::Idle
            },
            pre_m => match self.post_mode {
                RunMode::Idle => {
                    self.config_idle();
                    DeviceMode::Idle
                },

                pre_m => match self.channels_mode() {
                    RunMode::Idle => match pre_m {
                        RunMode::Idle => panic!("pre_m can't be Idle."),
                        RunMode::Feedforward => {
                            let (tx, rx) = crossbeam_channel::unbounded();
                            self.channels = DeviceMode::Feedforward(TmpFFW {
                                pre: None,
                                post: Some(rx),
                            });
                            DeviceMode::Feedforward(
                                ChannelsOutFFW {
                                    ch_ffw: tx
                                }
                            )                            
                        }
                    },

                    pre_m => DeviceMode::Feedforward(
                        ChannelsOutFFW {
                            ch_ffw: match &mut self.config {
                                DeviceMode::Idle => panic!("Linker is idle when take_pre!"),
                                DeviceMode::Feedforward(lnks) => lnks.pre.take(),
                            }    
                        }
                    ),
                    _ => panic!("lnker's channels configed into different mode from pre/post!"),
                },
                _ => panic!("pre/post of linker configed into different modes!"),
            },
        }
    }

    fn make_post(&mut self) -> DeviceMode<ChannelsInFFW<S>> {
        match self.post_mode() {
            RunMode::Idle => {
                self.config_idle();
                DeviceMode::Idle
            },
            post_m => match self.pre_mode {
                RunMode::Idle => {
                    self.config_idle();
                    DeviceMode::Idle
                },

                post_m => match self.channels_mode() {
                    RunMode::Idle => match post_m {
                        RunMode::Idle => panic!("post_m can't be Idle."),
                        RunMode::Feedforward => {
                            let (tx, rx) = crossbeam_channel::unbounded();
                            self.channels = DeviceMode::Feedforward(TmpFFW {
                                pre: Some(tx),
                                post: None,
                            });
                            DeviceMode::Feedforward(
                                ChannelsInFFW {
                                    ch_ffw: rx
                                }
                            )                            
                        }
                    },

                    pre_m => DeviceMode::Feedforward(
                        ChannelsInFFW {
                            ch_ffw: match &mut self.config {
                                DeviceMode::Idle => panic!("Linker is idle when take_pre!"),
                                DeviceMode::Feedforward(lnks) => lnks.post.take(),
                            }    
                        }
                    ),
                    _ => panic!("lnker's channels configed into different mode from pre/post!"),
                },
                _ => panic!("pre/post of linker configed into different modes!"),
            },
        }
    }

    fn config_idle(&mut self) {
        self.config = DeviceMode::Idle;
    }
}

struct TmpFFW<S: Send> {
    pub pre: Option<CCSender<S>>,
    pub post: Option<CCReceiver<S>>,
}

struct ChannelsOutFFW<S: Send> {
    pub ch_ffw: CCSender<S>,
}

struct ChannelsInFFW<S: Send> {
    pub ch_ffw: CCReceiver<S>,
}
