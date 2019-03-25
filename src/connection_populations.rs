use std::sync::{Arc, Mutex};
use crate::connections::PassiveConnection;
use crate::supervisor::RunMode;

pub trait ConnectionPopulation {
    fn config_run(&mut self, mode: RunMode);
    fn config_idle(&mut self);
}

pub struct SimplePassiveConnectionPopulation<T: PassiveConnection> {
    connections: Vec<Arc<Mutex<T>>>,
}

impl<T: 'static + PassiveConnection + Send>  SimplePassiveConnectionPopulation<T> {
    pub fn new() -> Arc<Mutex<SimplePassiveConnectionPopulation<T>>> {
        Arc::new(Mutex::new(SimplePassiveConnectionPopulation{
            connections: Vec::new(),
        }))
    }

    pub fn add_connection(&mut self, conn: Arc<Mutex<T>>) {
        self.connections.push(conn);
    }

    
    pub fn connection_by_id(&self, n: usize) -> Arc<Mutex<T>> {
        Arc::clone(&self.connections[n])
    }    
}
