use std::sync::{Weak, Mutex};
use crate::operation::{RunMode};
use crate::components::InSet;
use crate::connectivity::Generator;

pub struct SingleInComponent<G, R>
where G: Generator<R> + Send + ?Sized,
      R: Send,
{
    mode: RunMode,
    in_set: InSet<G, R>,
}

impl<G, R> SingleInComponent<G, R>
where G: Generator<R> + Send + ?Sized,
      R: Send,
{
    pub fn new(target: Weak<Mutex<G>>) -> SingleInComponent<G, R> {
        SingleInComponent {
            mode: RunMode::Idle,
            in_set: InSet::new(target),
        }
    }

    pub fn mode(&self) -> RunMode {
        self.mode
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
        self.in_set.config_mode(mode);
    }
    
    pub fn config_channels(&mut self) {
        self.in_set.config_channels();
    }
    
    pub fn ffw_accepted(&self) -> Vec<R> {
        match &self.mode {
            RunMode::Feedforward => {
                self.in_set.ffw_accepted_iter()
                    .map_or(Vec::with_capacity(0), |iter| iter.collect())
            },
            RunMode::Idle => panic!("SingleInComponent is Idle when accepted() called!"),
        }
    }
}
