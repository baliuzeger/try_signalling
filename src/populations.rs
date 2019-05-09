use std::sync::{Mutex, Arc};

// pub mod simple_consecutive_population;
pub mod simple_firing_population;
// pub mod simple_silent_population;
pub mod simple_passive_population;

pub trait HoldDevices {
    type Device: Send;
    fn device_by_id(&self, n: usize) -> Arc<Mutex<Self::Device>>;
}


