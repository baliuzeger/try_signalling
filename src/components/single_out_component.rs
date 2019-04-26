
use std::sync::{Weak, Mutex};
use crate::operation::RunMode;
use crate::connectivity::PassiveAcceptor;
use crate::components::OutSet;

pub struct SingleOutComponent<A, S>
where A: PassiveAcceptor<S> + Send + ?Sized,
      S: Send,
{
    mode: RunMode,
    out_set: OutSet<A, S>,
}

impl<A, S> SingleOutComponent<A, S>
where A: PassiveAcceptor<S> + Send + ?Sized,
      S: Send,
{
    pub fn new(target: Weak<Mutex<A>>) -> SingleOutComponent<A, S> {
        SingleOutComponent {
            mode: RunMode::Idle,
            out_set: OutSet::new(target),
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
        self.out_set.config_mode(mode);
    }
    
    pub fn config_channels(&mut self) {
        self.out_set.config_channels();
    }

    pub fn feedforward(&self, s: S) {
        match &self.mode {
            RunMode::Feedforward => self.out_set.feedforward(s),
            _ => panic!("PreAgentmodules1 is not Feedforward when feedforward called!"),
        }
    }
    
}
