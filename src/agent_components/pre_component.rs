

pub struct PreComponent {
    config: DeviceMode<AgentModuleIdle<dyn S1PassivePropagator + Send>,
                    PreAgentModuleFFW<dyn S1PassivePropagator + Send, FwdPreS1>>
}

impl PreComponent {
    pub fn new() -> PreComponent {
        PreComponent {
            config: DeviceMode::Idle(AgentModuleIdle::<dyn S1PassivePropagator + Send>:new()),
        }
    }

    pub fn mode(&self) -> RunMode {
        DeviceMode::variant(self.config)
    }
    
    pub fn add_connection(&mut self, connection: Weak<Mutex<dyn S1PassivePropagator + Send>>) {
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
            
        }
        
        self.out_connections_1.iter().map(|cn| RunningPassiveConnection::new(cn.connection.upgrade().unwrap())).collect()
    }
    
    pub fn config_idle(&mut self) {
        match &self.config {
            DeviceMode::Feedforward(m) => self.config = DeviceMode::Idle(m.make_idle()),
            DeviceMode::Idle(_) => println!("call fn config_idle when Idle, no effect."),
        }
    }

    pub fn feedforward(&self, s: FwdPostS1) {
        match &self {
            DeviceMode::FeedForward(m) => m.feeddorward(s),
            _ => panic!("PreAgentmodules1 is not Feedforward when feedforward called!");
        }
    }
}