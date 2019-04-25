use std::sync::{Mutex, Arc};
use crate::operation::Runnable;

pub mod simple_neuron_population;
pub mod simple_passive_population;

pub trait HoldDevices {
    type Device: Runnable + Send;
    fn device_by_id(&self, n: usize) -> Arc<Mutex<Self::Device>>;
}


