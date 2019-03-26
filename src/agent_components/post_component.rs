use std::sync::{Weak, Mutex};
use crate::agent_components::{ComponentIdle, PostComponentFFW};
use crate::supervisor::{RunMode, DeviceMode};
use crate::connections::PassiveConnection;

pub struct PostComponent<C: PassiveConnection + Send + ?Sized, S: Send> {
    config: DeviceMode<ComponentIdle<C>,
                       PostComponentFFW<C, S>>
}

impl<C: PassiveConnection + Send, S: Send> PostComponent<C, S> {
    fn new() -> PostComponent<C, S> {
        PostComponent {
            config: DeviceMode::Idle(ComponentIdle::new()),
        }
    }

    pub fn mode(&self) -> RunMode {
        DeviceMode::variant(self.config)
    }
    
    pub fn ffw_accepted(&self) -> Vec<S> {
        match &mut self {
            DeviceMode::Feedforward(m) => m.accepted(),
            DeviceMode::Idle(_) => panic!("PostComponent is Idle when .accepted called!"),
        }
    }
    
    pub fn add_connection(&mut self, connection: Weak<Mutex<S>>) {
        match &mut self.config {
            DeviceMode::Idle(m) => m.add_conection(connection), 
            _ => panic!("can only add_conntion when DeviceMode::Idle!"),
        }
    }

    pub fn config_run(&mut self, mode: RunMode) {
        match (mode, &self.config) {
            (DeviceMode::Idle(_), _) => println!("config_run for mode Idle, no effect."),
            (mi, DeviceMode::Idle(ms)) => self.config = DeviceMode::Feedforward(ms.make_ffw_post()),
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
