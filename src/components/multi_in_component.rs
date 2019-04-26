use std::sync::{Mutex, Weak};
use crate::operation::{RunMode, DeviceMode};
use crate::connectivity::Generator;
use crate::components::{InSet};

pub struct MultiInComponent<C, S>
where C: 'static + Generator<S> + Send + ?Sized,
      S: Send,
{
    mode: RunMode,
    target_sets: Vec<InSet<C, S>>,
}

impl<C, S> MultiInComponent<C, S>
where C: 'static + Generator<S> + Send + ?Sized,
      S: Send,
{
    pub fn new() -> MultiInComponent<C, S> {
        MultiInComponent {
            mode: RunMode::Idle,
            target_sets: Vec::new(),
        }
    }

    pub fn mode(&self) -> RunMode {
        self.mode
    }
    
    pub fn ffw_accepted(&self) -> Vec<S> {
        match &self.mode {
            RunMode::Feedforward => {
                self.target_sets.iter()
                    .filter_map(|set| set.ffw_accepted_iter()).flatten().collect()
            },
            RunMode::Idle => panic!("PostComponent is Idle when accepted() called!"),
        }
    }
    
    pub fn add_target(&mut self, target: Weak<Mutex<C>>) {
        match &mut self.mode {
            RunMode::Idle => self.target_sets.push(InSet::new(target)), 
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
        for set in &mut self.target_sets {
            set.config_mode(mode);
        }
    }

    pub fn config_channels(&mut self) {
        for set in &mut self.target_sets {
            set.config_channels();
        }
    }
}
