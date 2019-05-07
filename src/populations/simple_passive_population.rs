use std::sync::{Arc, Mutex};
use crate::operation::{RunMode};
use crate::operation::passive_device::PassiveDevice;
use crate::operation::passive_population::PassivePopulation;


pub struct SimplePassivePopulation<T>
where T: PassiveDevice,
{
    devices: Vec<Arc<Mutex<T>>>,
}

impl<T> PassivePopulation for SimplePassivePopulation<T>
where T: PassiveDevice,
{
    fn config_mode(&mut self, mode: RunMode) {
        // println!("SimplePassiveconnectionpopulation config_run.");
        for device in &self.devices {
            device.lock().unwrap().config_mode(mode);
        }
    }

    fn config_channels(&mut self) {
        for device in &self.devices {
            device.lock().unwrap().config_channels();
        }
    }
}

impl<T>  SimplePassivePopulation<T>
where T: PassiveDevice,
{
    pub fn new() -> Arc<Mutex<SimplePassivePopulation<T>>> {
        Arc::new(Mutex::new(SimplePassivePopulation{
            devices: Vec::new(),
        }))
    }

    pub fn add(&mut self, device: Arc<Mutex<T>>) {
        self.devices.push(device);
    }

    
    pub fn device_by_id(&self, n: usize) -> Arc<Mutex<T>> {
        Arc::clone(&self.devices[n])
    }    
}
