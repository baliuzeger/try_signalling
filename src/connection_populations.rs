use std::sync::{Arc, Mutex};
use crate::connections::PassiveConnection;

pub struct SimplePassiveConnectionPopulation<T: PassiveConnection> {
    connections: Vec<Arc<Mutex<T>>>,
}


