use std::sync::{Mutex, Arc};

pub struct SimplePopulation<T: Agent> {
    agents: Vec<Arc<Mutex<T>>>,
}

impl<T: 'static + Agent + Send> ActivePopulation for SimplePopulation<T> {
    fn config_run(&self, mode: RunMode) {
        for agnt in &self.agents {
            agnt.lock().unwrap().config_run(mode);
        }
    }

    fn config_idle(&mut self) {
        for agnt in &self.agents {
            agnt.lock().unwrap().config_idle();
        }
    }

    fn running_devices(&self) -> Vec<RunningAgent> {
        self.agents.iter().map(|agnt| RunningAgent::new(Arc::clone(&agnt))).collect()
    }
}

impl<T: Agent + Send> HoldAgents<T> for SimplePopulation<T> {
    fn agent_by_id(&self, n: usize) -> Arc<Mutex<T>> {
        Arc::clone(&self.agents[n])
    }    
}

impl<T: 'static + Agent + Send>  SimplePopulation<T> {
    pub fn new() -> Arc<Mutex<SimplePopulation<T>>> {
        Arc::new(Mutex::new(SimplePopulation{
            agents: Vec::new(),
        }))
    }

    pub fn add_agent(&mut self, agnt: Arc<Mutex<T>>) {
        self.agents.push(agnt);
    }
}
