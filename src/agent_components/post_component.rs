pub struct PostAgentModuleS1 {
    config: RunMode<AgentModuleIdle<dyn S1PassivePropagator + Send>,
                    PostAgentModuleFFW<dyn S1PassivePropagator + Send, FwdPostS1>>
}

impl PostAgentModuleS1 {
    fn new() -> PostAgentModuleS1 {
        PostAgentModuleS1 {
            config: RunMode::Idle(AgentModuleIdle::<dyn S1Propagator + Send>:new()),
        }
    }

    pub fn mode(&self) -> RunMode<bool, bool> {
        RunMode::variant(self.config)
    }
    
    pub fn ffw_accepted(&self) -> Vec<FwdPreS1> {
        match &mut self {
            RunMode::Feedforward(m) => m.accepted(),
            RunMode::Idle(_) => panic!("PostAgentModuleS1 is Idle when .accepted called!"),
        }
    }
    
    pub fn add_connection(&mut self, connection: Weak<Mutex<dyn S1Propagator + Send>>) {
        match &mut self.config {
            RunMode::Idle(m) => m.add_conection(connection), 
            _ => panic!("can only add_conntion when RunMode::Idle!"),
        }
    }

    pub fn config_run(&mut self, mode: RunMode<bool, bool>) {
        match (mode, &self.config) {
            (RunMode::Idle(_), _) => println!("config_run for mode Idle, no effect."),
            (mi, RunMode::Idle(ms)) => self.config = RunMode::Feedforward(ms.make_ffw_post()),
            (_, _) => panic!("call fn config_run when not RunMode::Idle!"),
        }
    }

    pub fn config_idle(&mut self) {
        match &self.config {
            RunMode::Feedforward(m) => self.config = RunMode::Idle(m.make_idle()),
            RunMode::Idle(_) => println!("call fn config_idle when Idle, no effect."),
        }
    }
}
