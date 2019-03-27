use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use crate::connections::PassiveConnection;
use crate::supervisor::{RunMode};

pub trait ConnectionPopulation {
    fn config_run(&mut self, mode: RunMode);
    fn config_idle(&mut self);
}

pub struct SimplePassiveConnectionPopulation<T, S0, S1>
where T: PassiveConnection<S0, S1>,
      S0: Send,
      S1: Send,
{
    connections: Vec<Arc<Mutex<T>>>,
    phantom: PhantomData<(S0, S1)>
}

impl<T, S0, S1> ConnectionPopulation for SimplePassiveConnectionPopulation<T, S0, S1>
where T: PassiveConnection<S0, S1>,
      S0: Send,
      S1: Send,
{
    fn config_run(&mut self, mode: RunMode) {
        self.connections.iter().map(|conn| conn.lock().unwrap().config_run(mode)).collect();
    }
    
    fn config_idle(&mut self) {
        self.connections.iter().map(|conn| conn.lock().unwrap().config_idle()).collect();
    }
}

impl<T, S0, S1>  SimplePassiveConnectionPopulation<T, S0, S1>
where T: PassiveConnection<S0, S1>,
      S0: Send,
      S1: Send,
{
    pub fn new() -> Arc<Mutex<SimplePassiveConnectionPopulation<T, S0, S1>>> {
        Arc::new(Mutex::new(SimplePassiveConnectionPopulation{
            connections: Vec::new(),
            phantom: PhantomData {},
        }))
    }

    pub fn add_connection(&mut self, conn: Arc<Mutex<T>>) {
        self.connections.push(conn);
    }

    
    pub fn connection_by_id(&self, n: usize) -> Arc<Mutex<T>> {
        Arc::clone(&self.connections[n])
    }    
}
