use std::sync::{Weak, Mutex, Arc};
use crate::operation::RunMode;
use crate::connectivity::PassiveAcceptor;
use crate::components::{OutSet, Linker};

pub struct SingleOutComponent<A, S>
where A: PassiveAcceptor<S> + Send + ?Sized,
      S: Send,
{
    mode: RunMode,
    out_set: Option<OutSet<A, S>>,
}

impl<A, S> SingleOutComponent<A, S>
where A: PassiveAcceptor<S> + Send + ?Sized,
      S: Send,
{
    pub fn new() -> SingleOutComponent<A, S> {
        SingleOutComponent {
            mode: RunMode::Idle,
            out_set: None,
        }
    }


    pub fn add_target(&mut self, target: Weak<Mutex<A>>, linker: Arc<Mutex<Linker<S>>>) {
        match &mut self.mode {
            RunMode::Idle => match self.out_set {
                None => self.out_set = Some(OutSet::new(target, linker)),
                Some(_) => println!("SingleOutComponent already connected!"),
            }
            _ => panic!("SingleOutComponent can only add_conntion when DeviceMode::Idle!"),
        }
    }
    
    pub fn mode(&self) -> RunMode {
        self.mode
    }

    pub fn config_mode(&mut self, mode: RunMode) {
        match (mode, &self.mode) {
            (RunMode::Idle, RunMode::Idle) => println!("SingleOutComponent config_mode from Idle to Idle, no effect."),
            (RunMode::Idle, _) => self.config_mode_to(mode),
            (_, RunMode::Idle) => self.config_mode_to(mode),
            (_, _) => panic!("unhandled config_mode: from {:?} to {:?}.", self.mode(), mode),
        }
    }

    fn config_mode_to(&mut self, mode: RunMode) {
        self.mode = mode;
        match &mut self.out_set {
            None => (),
            Some(set) => set.config_mode(mode),
        }
    }
    
    pub fn config_channels(&mut self) {
        match &mut self.out_set {
            None => (),
            Some(set) => set.config_channels(),
        }
    }

    pub fn feedforward(&self, s: S) {
        match &self.mode {
            RunMode::Feedforward => match &self.out_set {
                None => (),
                Some(set) => set.feedforward(s),
            }
            _ => panic!("PreAgentmodules1 is not Feedforward when feedforward called!"),
        }
    }
    
}
