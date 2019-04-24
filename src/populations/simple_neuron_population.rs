use std::sync::{Mutex, Arc};
use crate::operation::{RunMode, RunningSet};
use crate::operation::neuron::Neuron;
use crate::populations::HoldDevices;

pub struct SimplePopulation<T: Neuron> {
    neurons: Vec<Arc<Mutex<T>>>,
}

impl<T: 'static + Agent + Send> ActivePopulation for SimplePopulation<T> {
    fn config_run(&self, mode: RunMode) {
        for neuron in &self.neurons {
            neuron.lock().unwrap().config_run(mode);
        }
    }

    fn config_idle(&mut self) {
        for neuron in &self.neurons {
            neuron.lock().unwrap().config_idle();
        }
    }

    fn running_devices(&self) -> Vec<RunningSet> {
        self.neurons.iter().map(|neuron| RunningSet::new(Arc::clone(&neuron))).collect()
    }
}

impl<T: Agent + Send> HoldDevices for SimplePopulation<T> {
    type Device = T;
    fn agent_by_id(&self, n: usize) -> Arc<Mutex<T>> {
        Arc::clone(&self.neurons[n])
    }    
}

impl<T: 'static + Agent + Send>  SimplePopulation<T> {
    pub fn new() -> Arc<Mutex<SimplePopulation<T>>> {
        Arc::new(Mutex::new(SimplePopulation{
            neurons: Vec::new(),
        }))
    }

    pub fn add_agent(&mut self, neuron: Arc<Mutex<T>>) {
        self.neurons.push(neuron);
    }
}
