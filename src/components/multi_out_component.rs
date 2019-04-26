use std::sync::{Mutex, Weak};
use crate::operation::{RunningSet, RunMode, DeviceMode, Broadcast};
use crate::connectivity::{PassiveAcceptor};
use crate::components::OutSet;

pub struct MultiOutComponent<C, S>
where C: 'static + PassiveAcceptor<S> + Send + ?Sized,
      S: Send
      //CA: 'static + ActiveAcceptor<S> + Send + ?Sized,
{
    mode: RunMode,
    passive_out_sets: Vec<OutSet<C, S>>,
    // active_taegets: Vec<OutSet<CA, S>>
}

impl<C, S> MultiOutComponent<C, S>
where C: 'static + PassiveAcceptor<S> + Send + ?Sized,
      S: Send + Copy,
{
    pub fn new() -> MultiOutComponent<C, S> {
        MultiOutComponent {
            mode: RunMode::Idle,
            passive_out_sets: Vec::new(),
        }
    }

    pub fn mode(&self) -> RunMode {
        self.mode
    }
    
    pub fn add_passive_target(&mut self, target: Weak<Mutex<C>>) {
        match &mut self.mode {
            RunMode::Idle => self.passive_out_sets.push(OutSet::new(target)), 
            _ => panic!("can only add_conntion when DeviceMode::Idle!"),
        }
    }

    pub fn config_mode(&mut self, mode: RunMode) {
        match (mode, &self.mode) {
            (RunMode::Idle, RunMode::Idle) => println!("config_mode from Idle to Idle, no effect."),
            (RunMode::Idle, _) => self.config_mode_to(mode),
            (_, RunMode::Idle) => self.config_mode_to(mode),
            (_, _) => panic!("unhandled config_mode: from {:?} to {:?}.", self.mode(), mode),
        }
    }

    fn config_mode_to(&mut self, mode: RunMode) {
        self.mode = mode;
        for set in &mut self.passive_out_sets {
            set.config_mode(mode);
        }
    }
    
    pub fn config_channels(&mut self) {
        for set in &mut self.passive_out_sets {
            set.config_channels();
        }
    }

    pub fn running_passive_targets(&self) -> Vec<RunningSet<Broadcast, ()>> {
        match &self.mode {
            RunMode::Idle => panic!("call running_passive_targets when agent Idle!"),
            RunMode::Feedforward => {
                self.passive_out_sets.iter()
                    .filter_map(|set| match set.channels {
                        DeviceMode::Idle => None,
                        DeviceMode::Feedforward(_) => Some(RunningSet::<Broadcast, ()>::new_passive_device(set.target.upgrade().unwrap()))
                    }).collect()                
            }
        }
    }
    
    pub fn config_idle(&mut self) {
        match &self.mode {
            RunMode::Feedforward => {
                self.mode = RunMode::Idle;
                for set in &mut self.passive_out_sets {
                    set.config_idle();
                }
            }
            RunMode::Idle => println!("call fn config_idle when Idle, no effect."),
        }
    }

    pub fn feedforward(&self, s: S) {
        match &self.mode {
            RunMode::Feedforward => for set in &self.passive_out_sets {
                set.feedforward(s);
            },
            _ => panic!("PreAgentmodules1 is not Feedforward when feedforward called!"),
        }
    }
}
