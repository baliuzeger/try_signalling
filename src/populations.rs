use std::sync::{Mutex, Arc};

// pub mod simple_consecutive_population;
pub mod simple_firing_population;
pub use simple_firing_population::SimpleFiringPopulation;
// pub mod simple_silent_population;
pub mod simple_passive_population;
pub use simple_passive_population::SimplePassivePopulation;

pub trait HoldDevices {
    type Device: Send;
    fn device_by_id(&self, n: usize) -> Arc<Mutex<Self::Device>>;
}


