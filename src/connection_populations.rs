use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use crate::connections::PassiveConnection;
use crate::supervisor::{RunMode};

pub trait ConnectionPopulation {
    fn config_run(&mut self, mode: RunMode);
    fn config_idle(&mut self);
}

pub struct SimplePassiveConnectionPopulation<T>
where T: PassiveConnection<S0, S1>,
{
    connections: Vec<Arc<Mutex<T>>>,
}

impl<T> ConnectionPopulation for SimplePassiveConnectionPopulation<T>
where T: PassiveConnection<S0, S1>,
{
    fn config_run(&mut self, mode: RunMode) {
        // println!("SimplePassiveconnectionpopulation config_run.");
        for conn in &self.connections {
            conn.lock().unwrap().config_run(mode);
        }
    }
    
    fn config_idle(&mut self) {
        for conn in &self.connections {
            conn.lock().unwrap().config_idle();
        }
    }
}

impl<T>  SimplePassiveConnectionPopulation<T>
where T: PassiveConnection<S0, S1>,
{
    pub fn new() -> Arc<Mutex<SimplePassiveConnectionPopulation<T>>> {
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

pub struct MixedSimplePassiveConnectionPopulation<T>
where T: PassiveConnection
{
    connections: Vec<Arc<Mutex<T>>>,
}
