use std::sync::{Weak, Mutex};
use crate::agent_components::{ComponentIdle, PostComponentFFW};
use crate::supervisor::{RunMode, DeviceMode};
use crate::connections::PassiveConnection;

pub struct PostComponent<C, S0, S1>
where C: 'static + PassiveConnection<S0, S1> + Send + ?Sized,
      S0: Send + Copy,
      S1: Send + Copy,
{
    config: DeviceMode<ComponentIdle<C, S0, S1>,
                       PostComponentFFW<C, S0, S1>>
}

impl<C, S0, S1> PostComponent<C, S0, S1>
where C: 'static + PassiveConnection<S0, S1> + Send + ?Sized,
      S0: Send + Copy,
      S1: Send + Copy,
{
    pub fn new() -> PostComponent<C, S0, S1> {
        PostComponent {
            config: DeviceMode::Idle(ComponentIdle::new()),
        }
    }

    pub fn mode(&self) -> RunMode {
        RunMode::mode_from_device(&self.config)
    }
    
    pub fn ffw_accepted(&self) -> Vec<S1> {
        match &self.config {
            DeviceMode::Feedforward(m) => m.accepted(),
            DeviceMode::Idle(_) => panic!("PostComponent is Idle when .accepted called!"),
        }
    }
    
    pub fn add_connection(&mut self, connection: Weak<Mutex<C>>) {
        match &mut self.config {
            DeviceMode::Idle(m) => m.add_connection(connection), 
            _ => panic!("can only add_conntion when DeviceMode::Idle!"),
        }
    }

    pub fn config_run(&mut self, mode: RunMode) {
        match (mode, &self.config) {
            (RunMode::Idle, _) => println!("config_run for mode Idle, no effect."),
            (_, DeviceMode::Idle(ms)) => self.config = DeviceMode::Feedforward(ms.make_ffw_post::<S1>()),
            (_, _) => panic!("call fn config_run when not DeviceMode::Idle!"),
        }
    }

    pub fn config_idle(&mut self) {
        match &self.config {
            DeviceMode::Feedforward(m) => self.config = DeviceMode::Idle(m.make_idle()),
            DeviceMode::Idle(_) => println!("call fn config_idle when Idle, no effect."),
        }
    }
}
