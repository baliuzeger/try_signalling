use std::sync::{Mutex, Weak};
use crate::operation::{RunningSet, RunMode, DeviceMode};
use crate::connectivity::Generator;
use crate::components::{InSet};

pub struct InConnectionSet<C: Send + ?Sized>
where C: PassiveExporter + Send + ?Sized,
{
    pub connection: Weak<Mutex<C>>,
    pub config: DeviceMode<ChannelsInFFW<C::Signal>>,
}

impl<C: Send + ?Sized> InConnectionSet<C> {
    pub fn config_run(&mut self, mode: RunMode) {
        let arc = self.connection.upgrade().unwrap();
        let mut unlocked_conn = arc.lock().unwrap();
        self.config = match unlocked_conn.mode() {
            RunMode::Idle => DeviceMode::Idle,
            RunMode::Feedforward => {
                let (tx, rx) = crossbeam_channel::bounded(1);
                unlocked_conn.set_pre_channel_ffw(Some(tx));
                DeviceMode::Feedforward(
                    ChannelsOutFFW {
                        ch_ffw: rx
                    }
                )
            },
        }
    }

    pub fn config_idle(&mut self) {
        self.config = DeviceMode::Idle;
    }
}

struct ChannelsInFFW<S: Send> {
    ch_ffw: CCReceiver<S>,
}

pub struct PostComponent<C>
where C: 'static + PassiveExporter + Send + ?Sized,
{
    mode: RunMode,
    connections: Vec<InConnectionSet<C>>,
}

impl<C> PostComponent<C>
where C: 'static + PassiveExporter + Send + ?Sized
{
    pub fn new() -> PostComponent<C> {
        PostComponent {
            mode: RunMode::Idle,
            connections: Vec::new(),
        }
    }

    pub fn mode(&self) -> RunMode {
        self.mode
    }
    
    pub fn ffw_accepted(&self) -> Vec<S1> {
        match &self.mode {
            RunMode::Feedforward => {
                self.connections.iter()
                    .filter_map(|set| {
                        match &set.config {
                            DeviceMode::Idle => None,
                            DeviceMode::Feedforward(chs_in_ffw) => chs_in_ffw.ch_ffw.try_iter()
                        }
                    }).flatten().collect()
            },
            RunMode::Idle => panic!("PostComponent is Idle when accepted() called!"),
        }
    }
    
    pub fn add_connection(&mut self, connection: Weak<Mutex<C>>) {
        match &mut self.mode {
            RunMode::Idle(m) => self.connections.push(InConnectionSet {
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
            (_, _) => panic!("call fn config_run when not DeviceMode::Idle!"),
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
}
