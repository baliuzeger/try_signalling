pub struct PostAgentModuleS1 {
    config: DeviceMode<AgentModuleIdle<dyn S1PassivePropagator + Send>,
                    PostAgentModuleFFW<dyn S1PassivePropagator + Send, FwdPostS1>>
}

impl PostAgentModuleS1 {
    fn new() -> PostAgentModuleS1 {
        PostAgentModuleS1 {
            config: DeviceMode::Idle(AgentModuleIdle::<dyn S1Propagator + Send>:new()),
        }
    }

    pub fn mode(&self) -> RunMode {
        DeviceMode::variant(self.config)
    }
    
    pub fn ffw_accepted(&self) -> Vec<FwdPreS1> {
        match &mut self {
            DeviceMode::Feedforward(m) => m.accepted(),
            DeviceMode::Idle(_) => panic!("PostAgentModuleS1 is Idle when .accepted called!"),
        }
    }
    
    pub fn add_connection(&mut self, connection: Weak<Mutex<dyn S1Propagator + Send>>) {
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
