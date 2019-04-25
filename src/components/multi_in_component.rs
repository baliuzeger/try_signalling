use std::sync::{Mutex, Weak};
use crate::operation::{RunMode, DeviceMode};
use crate::connectivity::Generator;
use crate::components::{InSet};

pub struct MultiInComponent<C, S>
where C: 'static + Generator<S> + Send + ?Sized,
      S: Send,
{
    mode: RunMode,
    targets: Vec<InSet<C, S>>,
}

impl<C, S> MultiInComponent<C, S>
where C: 'static + Generator<S> + Send + ?Sized,
      S: Send,
{
    pub fn new() -> MultiInComponent<C, S> {
        MultiInComponent {
            mode: RunMode::Idle,
            targets: Vec::new(),
        }
    }

    pub fn mode(&self) -> RunMode {
        self.mode
    }
    
    pub fn ffw_accepted(&self) -> Vec<S> {
        match &self.mode {
            RunMode::Feedforward => {
                self.targets.iter()
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
    
    pub fn add_target(&mut self, target: Weak<Mutex<C>>) {
        match &mut self.mode {
            RunMode::Idle => self.targets.push(InSet {
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
        for set in &mut self.targets {
            set.config_channels(self.mode());
        }
    }

    pub fn config_idle(&mut self) {
        match &self.mode {
            RunMode::Feedforward => {
                self.mode = RunMode::Idle;
                for set in &mut self.targets {
                    set.config_idle();
                }
            }
            RunMode::Idle => println!("call fn config_idle when Idle, no effect."),
        }
    }
}
