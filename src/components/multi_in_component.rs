extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Mutex, Weak};
use crate::supervisor::{RunMode, DeviceMode};
use crate::connections::{RunningPassiveConnection, PassiveImporter};

pub struct OutConnectionSet<C: Send + ?Sized>
where C: PassiveImporter + Send + ?Sized,
{
    pub connection: Weak<Mutex<C>>,
    pub config: DeviceMode<ChannelsOutFFW<C::Signal>>,
}

impl<C: Send + ?Sized> OutConnectionSet<C> {
    pub fn config_run(&mut self, mode: RunMode) {
        let arc = self.connection.upgrade().unwrap();
        let mut unlocked_conn = arc.lock().unwrap();
        self.config = match unlocked_conn.mode() {
            RunMode::Idle => DeviceMode::Idle,
            RunMode::Feedforward => {
                let (tx, rx) = crossbeam_channel::bounded(1);
                unlocked_conn.set_pre_channel_ffw(Some(rx));
                DeviceMode::Feedforward(
                    ChannelsOutFFW {
                        ch_ffw: tx
                    }
                )
            },
        }
    }

    pub fn config_idle(&mut self) {
        self.config = DeviceMode::Idle;
    }
}

struct ChannelsOutFFW<S: Send> {
    pub ch_ffw: CCSender<S>,
}

pub struct PreComponent<C>
where C: 'static + PassiveImporter + Send + ?Sized,
{
    mode: RunMode,
    connections: Vec<OutConnectionSet<C>>,
}

impl<C> PreComponent<C>
where C: 'static + PassiveImporter + Send + ?Sized,
{
    pub fn new() -> PreComponent<C> {
        PreComponent {
            mode: RunMode::Idle,
            connections: Vec::new(),
        }
    }

    pub fn mode(&self) -> RunMode {
        self.mode
    }
    
    pub fn add_connection(&mut self, connection: Weak<Mutex<C>>) {
        match &mut self.mode {
            RunMode::Idle => self.connections.push(OutConnectionSet {
                connection,
                config: DeviceMode::Idle,
            }), 
            _ => panic!("can only add_conntion when DeviceMode::Idle!"),
        }
    }

    pub fn config_run(&mut self, mode: RunMode) {
        match (mode, &self.mode) {
            (RunMode::Idle, _) => println!("config_run for mode Idle, no effect."),
            (_, RunMode::Idle(ms)) => {
                self.mode = mode;
                for set in &mut self.connections {
                    set.config_run(mode);
                }
            }
            (_, _) => panic!("call fn config_run when not RunMode::Idle!"),
        }
    }

    pub fn running_connections(&self) -> Vec<RunningPassiveConnection> {
        match &self.mode {
            RunMode::Idle => panic!("call running_connections when agent Idle!"),
            RunMode::Feedforward => self.connections.iter().filter_map(|set| {
                match &set.config {
                    DeviceMode::Idle => None,
                    DeviceMode::Feedforward(chs) => Some(RunningPassiveConnection::new(set.connection.upgrade().unwrap())),
                }
            }).collect()
        }
    }
    
    pub fn config_idle(&mut self) {
        match &self.mode {
            RunMode::Feedforward => {
                self.mode = RunMode::Idle;
                for set in &mut self.connections {
                    set.config_idle();
                }
            }
            RunMode::Idle => println!("call fn config_idle when Idle, no effect."),
        }
    }

    pub fn feedforward(&self, s: S0) {
        match &self.mode {
            RunMode::Feedforward => for set in &self.connections {
                match &set.config {
                    DeviceMode::Idle => (),
                    DeviceMode::Feedforward(chs) => chs.ch_ffw.send(s).unwrap(),
                }
            }
            _ => panic!("PreAgentmodules1 is not Feedforward when feedforward called!"),
        }
    }
}
