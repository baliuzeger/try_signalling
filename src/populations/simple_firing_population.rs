use std::sync::{Mutex, Arc};
use crate::operation::{RunMode, RunningSet, Broadcast, Fired};
use crate::operation::firing_device::FiringDevice;
use crate::operation::firing_population::FiringPopulation;
use crate::populations::HoldDevices;

pub struct SimpleFiringPopulation<T: FiringDevice> {
    devices: Vec<Arc<Mutex<T>>>,
}

impl<T: 'static + FiringDevice + Send> FiringPopulation for SimpleFiringPopulation<T> {
    fn config_mode(&mut self, mode: RunMode) {
        for device in &self.devices {
            device.lock().unwrap().config_run(mode);
        }
    }

    fn config_channels(&mut self) {
        for device in &self.devices {
            device.lock().unwrap().config_channels();
        }
    }

    fn running_devices(&self) -> Vec<RunningSet<Broadcast, Fired>> {
        self.devices.iter().map(|device| RunningSet::<Broadcast, Fired>::new_firing_device(Arc::clone(&device))).collect()
    }
}

impl<T: FiringDevice + Send> HoldDevices for SimpleFiringPopulation<T> {
    type Device = T;
    fn device_by_id(&self, n: usize) -> Arc<Mutex<T>> {
        Arc::clone(&self.devices[n])
    }    
}

impl<T: 'static + FiringDevice + Send>  SimpleFiringPopulation<T> {
    pub fn new() -> Arc<Mutex<SimpleFiringPopulation<T>>> {
        Arc::new(Mutex::new(SimpleFiringPopulation{
            devices: Vec::new(),
        }))
    }

    pub fn add(&mut self, device: Arc<Mutex<T>>) {
        self.devices.push(device);
    }
}
