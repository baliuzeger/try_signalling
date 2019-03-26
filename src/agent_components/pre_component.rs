use std::sync::{Mutex, Weak};
use crate::agent_components::{ComponentIdle, PreComponentFFW};
use crate::supervisor::{RunMode, DeviceMode};
use crate::connections::{RunningPassiveConnection, PassiveConnection};

pub struct PreComponent<C: PassiveConnection + Send, S: Send> {
    config: DeviceMode<ComponentIdle<C>,
                       PreComponentFFW<C, S>>
}

impl<C: PassiveConnection + Send, S: Send> PreComponent<C, S> {
    pub fn new() -> PreComponent<C, S> {
        PreComponent {
            config: DeviceMode::Idle(ComponentIdle::new()),
        }
    }

    pub fn mode(&self) -> RunMode {
        DeviceMode::variant(self.config)
    }
    
    pub fn add_connection(&mut self, connection: Weak<Mutex<C>>) {
        match &mut self.config {
            DeviceMode::Idle(m) => m.add_conection(connection), 
            _ => panic!("can only add_conntion when DeviceMode::Idle!"),
        }
    }

    pub fn config_run(&mut self, mode: RunMode) {
        match (mode, &self.config) {
            (DeviceMode::Idle(_), _) => println!("config_run for mode Idle, no effect."),
            (mi, DeviceMode::Idle(ms)) => self.config = DeviceMode::Feedforward(ms.make_ffw_pre()),
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

    pub fn feedforward(&self, s: S) {
        match &self {
            DeviceMode::FeedForward(m) => m.feeddorward(s),
            _ => panic!("PreAgentmodules1 is not Feedforward when feedforward called!"),
        }
    }
}
