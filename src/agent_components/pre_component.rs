use std::sync::{Mutex, Weak};
use crate::agent_components::{ComponentIdle, PreComponentFFW};
use crate::supervisor::{RunMode, DeviceMode};
use crate::connections::{RunningPassiveConnection, PassiveConnection};

pub struct PreComponent<C, S0, S1>
where C: 'static + PassiveConnection<S0, S1> + Send + ?Sized,
      S0: Send,
      S1: Send,
{
    config: DeviceMode<ComponentIdle<C, S0, S1>,
                       PreComponentFFW<C, S0, S1>>
}

impl<C, S0, S1> PreComponent<C, S0, S1>
where C: 'static +PassiveConnection<S0, S1> + Send + ?Sized,
      S0: Send,
      S1: Send,
{
    pub fn new() -> PreComponent<C, S0, S1> {
        PreComponent {
            config: DeviceMode::Idle(ComponentIdle::new()),
        }
    }

    pub fn mode(&self) -> RunMode {
        RunMode::mode_from_device(self.config)
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
            (_, DeviceMode::Idle(ms)) => self.config = DeviceMode::Feedforward(ms.make_ffw_pre::<S0>()),
            (_, _) => panic!("call fn config_run when not DeviceMode::Idle!"),
        }
    }

    pub fn running_connections(&self) -> Vec<RunningPassiveConnection> {
        match &self.config {
            DeviceMode::Idle(_) => panic!("call running_connections when agent Idle!"),
            DeviceMode::Feedforward(m) => m.running_connections(),
        }
    }
    
    pub fn config_idle(&mut self) {
        match &self.config {
            DeviceMode::Feedforward(m) => self.config = DeviceMode::Idle(m.make_idle()),
            DeviceMode::Idle(_) => println!("call fn config_idle when Idle, no effect."),
        }
    }

    pub fn feedforward(&self, s: S0) {
        match &self.config {
            DeviceMode::Feedforward(m) => m.feedforward(s),
            _ => panic!("PreAgentmodules1 is not Feedforward when feedforward called!"),
        }
    }
}
