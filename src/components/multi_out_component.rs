use std::sync::{Mutex, Weak};
use crate::operation::{RunningSet, RunMode, DeviceMode, Broadcast};
use crate::operation::passive_device::PassiveDevice;
use crate::connectivity::{PassiveAcceptor};
use crate::components::OutSet;

pub struct MultiOutComponent<C, S>
where C: 'static + PassiveAcceptor<S> + Send + ?Sized,
      S: Send
      //CA: 'static + ActiveAcceptor<S> + Send + ?Sized,
{
    mode: RunMode,
    passive_targets: Vec<OutSet<C, S>>,
    // active_taegets: Vec<OutSet<CA, S>>
}

impl<C, S> MultiOutComponent<C, S>
where C: 'static + PassiveAcceptor<S> + Send + ?Sized,
      S: Send,
{
    pub fn new() -> MultiOutComponent<C, S> {
        MultiOutComponent {
            mode: RunMode::Idle,
            passive_targets: Vec::new(),
        }
    }

    pub fn mode(&self) -> RunMode {
        self.mode
    }
    
    pub fn add_passive_target(&mut self, target: Weak<Mutex<C>>) {
        match &mut self.mode {
            RunMode::Idle => self.passive_targets.push(OutSet {
                target,
                config: DeviceMode::Idle,
            }), 
            _ => panic!("can only add_conntion when DeviceMode::Idle!"),
        }
    }

    pub fn config_run(&mut self, mode: RunMode) {
        match (mode, &self.mode) {
            (RunMode::Idle, _) => println!("config_run for mode Idle, no effect."),
            (_, RunMode::Idle) => self.mode = mode,
            (_, _) => panic!("call fn config_run when not RunMode::Idle!"),
        }
    }
    
    pub fn config_channels(&mut self) {
        for set in &mut self.passive_targets {
            set.config_channels(self.mode());
        }
    }

    pub fn running_passive_targets(&self) -> Vec<RunningSet<Broadcast, ()>> {
        match &self.mode {
            RunMode::Idle => panic!("call running_passive_targets when agent Idle!"),
            RunMode::Feedforward => self.passive_targets.iter().filter_map(|set| {
                match &set.config {
                    DeviceMode::Idle => None,
                    DeviceMode::Feedforward(chs) => Some(RunningSet::new(set.target.upgrade().unwrap())),
                }
            }).collect()
        }
    }
    
    pub fn config_idle(&mut self) {
        match &self.mode {
            RunMode::Feedforward => {
                self.mode = RunMode::Idle;
                for set in &mut self.passive_targets {
                    set.config_idle();
                }
            }
            RunMode::Idle => println!("call fn config_idle when Idle, no effect."),
        }
    }

    pub fn feedforward(&self, s: S) {
        match &self.mode {
            RunMode::Feedforward => for set in &self.passive_targets {
                match &set.config {
                    DeviceMode::Idle => (),
                    DeviceMode::Feedforward(chs) => chs.ch_ffw.send(s).unwrap(),
                }
            }
            _ => panic!("PreAgentmodules1 is not Feedforward when feedforward called!"),
        }
    }
}
