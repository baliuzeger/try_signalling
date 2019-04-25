use std::sync::{Mutex, Weak};
use crate::operation::{RunningSet, RunMode, DeviceMode};
use crate::operation::passive_device::PassiveDevice;
use crate::connectivity::{PassiveAcceptor};
use crate::components::OutSet;

pub struct PreComponent<C, S>
where C: 'static + PassiveAcceptor<S> + Send + ?Sized,
      S: Send
{
    mode: RunMode,
    connections: Vec<OutSet<C, S>>,
}

impl<C, S> PreComponent<C, S>
where C: 'static + PassiveAcceptor<S> + Send + ?Sized,
      S: Send,
{
    pub fn new() -> PreComponent<C, S> {
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
            RunMode::Idle => self.connections.push(OutSet {
                connection,
                config: DeviceMode::Idle,
            }), 
            _ => panic!("can only add_conntion when DeviceMode::Idle!"),
        }
    }

    pub fn config_run(&mut self, mode: RunMode) {
        match (mode, &self.mode) {
            (RunMode::Idle, _) => println!("config_run for mode Idle, no effect."),
            (_, RunMode::Idle) => {
                self.mode = mode;
                for set in &mut self.connections {
                    set.config_run(mode);
                }
            }
            (_, _) => panic!("call fn config_run when not RunMode::Idle!"),
        }
    }

    pub fn running_connections(&self) -> Vec<RunningSet> {
        match &self.mode {
            RunMode::Idle => panic!("call running_connections when agent Idle!"),
            RunMode::Feedforward => self.connections.iter().filter_map(|set| {
                match &set.config {
                    DeviceMode::Idle => None,
                    DeviceMode::Feedforward(chs) => Some(RunningSet::new(set.connection.upgrade().unwrap())),
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

    pub fn feedforward(&self, s: S) {
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
