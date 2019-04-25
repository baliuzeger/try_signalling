use std::sync::{Mutex, Arc};

pub mod simple_neuron_population;
pub mod simple_passive_population;

pub trait HoldDevices {
    type Device: Send;
    fn device_by_id(&self, n: usize) -> Arc<Mutex<Self::Device>>;
}


