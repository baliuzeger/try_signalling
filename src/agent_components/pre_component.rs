

pub struct PreComponent {
    config: RunMode<AgentModuleIdle<dyn S1PassivePropagator + Send>,
                    PreAgentModuleFFW<dyn S1PassivePropagator + Send, FwdPreS1>>
}

impl PreComponent {
    pub fn new() -> PreComponent {
        PreComponent {
            config: RunMode::Idle(AgentModuleIdle::<dyn S1PassivePropagator + Send>:new()),
        }
    }

    pub fn mode(&self) -> RunMode<bool, bool> {
        RunMode::variant(self.config)
    }
    
    pub fn add_connection(&mut self, connection: Weak<Mutex<dyn S1PassivePropagator + Send>>) {
        match &mut self.config {
            RunMode::Idle(m) => m.add_conection(connection), 
            _ => panic!("can only add_conntion when RunMode::Idle!"),
        }
    }

    pub fn config_run(&mut self, mode: RunMode<bool, bool>) {
        match (mode, &self.config) {
            (RunMode::Idle(_), _) => println!("config_run for mode Idle, no effect."),
            (mi, RunMode::Idle(ms)) => self.config = RunMode::Feedforward(ms.make_ffw_pre()),
            (_, _) => panic!("call fn config_run when not RunMode::Idle!"),
        }
    }
    
    pub fn config_idle(&mut self) {
        match &self.config {
            RunMode::Feedforward(m) => self.config = RunMode::Idle(m.make_idle()),
            RunMode::Idle(_) => println!("call fn config_idle when Idle, no effect."),
        }
    }

    pub fn feedforward(&self, s: FwdPostS1) {
        match &self {
            RunMode::FeedForward(m) => m.feeddorward(s),
            _ => panic!("PreAgentmodules1 is not Feedforward when feedforward called!");
        }
    }
}
