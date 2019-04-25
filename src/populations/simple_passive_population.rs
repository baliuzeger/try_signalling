use std::sync::{Arc, Mutex};
use crate::operation::{RunMode};
use crate::operation::passive_device::PassiveDevice;
use crate::operation::passive_population::PassivePopulation;


pub struct SimplePassiveConnectionPopulation<T>
where T: PassiveDevice,
{
    connections: Vec<Arc<Mutex<T>>>,
}

impl<T> PassivePopulation for SimplePassiveConnectionPopulation<T>
where T: PassiveDevice,
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
where T: PassiveDevice,
{
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
