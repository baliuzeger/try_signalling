use std::sync::{Mutex, Arc};
use crate::operation::{RunMode, RunningSet, Broadcast, Fired};
use crate::operation::firing_device::FiringDevice;
use crate::operation::firing_population::FiringPopulation;
use crate::populations::HoldDevices;

pub struct SimplePopulation<T: FiringDevice> {
    neurons: Vec<Arc<Mutex<T>>>,
}

impl<T: 'static + FiringDevice + Send> FiringPopulation for SimplePopulation<T> {
    fn config_run(&mut self, mode: RunMode) {
        for neuron in &self.neurons {
            neuron.lock().unwrap().config_run(mode);
        }
    }

    fn config_idle(&mut self) {
        for neuron in &self.neurons {
            neuron.lock().unwrap().config_idle();
        }
    }

    fn running_devices(&self) -> Vec<RunningSet<Broadcast, Fired>> {
        self.neurons.iter().map(|neuron| RunningSet::new(Arc::clone(&neuron))).collect()
    }
}

impl<T: FiringDevice + Send> HoldDevices for SimplePopulation<T> {
    type Device = T;
    fn device_by_id(&self, n: usize) -> Arc<Mutex<T>> {
        Arc::clone(&self.neurons[n])
    }    
}

impl<T: 'static + FiringDevice + Send>  SimplePopulation<T> {
    pub fn new() -> Arc<Mutex<SimplePopulation<T>>> {
        Arc::new(Mutex::new(SimplePopulation{
            neurons: Vec::new(),
        }))
    }

    pub fn add_agent(&mut self, neuron: Arc<Mutex<T>>) {
        self.neurons.push(neuron);
    }
}
