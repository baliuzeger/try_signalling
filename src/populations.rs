use std::sync::{Mutex, Arc};

pub mod active_popoulations;
pub mod passibe_populations;

pub trait HoldDevices<T: Device + Send> {
    fn device_by_id(&self, n: usize) -> Arc<Mutex<T>>;
}

    
