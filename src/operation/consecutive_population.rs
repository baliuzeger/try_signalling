extern crate crossbeam_channel;
use crate::random_sleep;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crate::operation::{RunningSet, Broadcast, RunMode};

pub trait ConsecutivePopulation {
    fn config_mode(&mut self, mode: RunMode);
    fn config_channels(&mut self);
    fn running_devices(&self) -> Vec<RunningSet<Broadcast, ()>>;

    // fn run_f(&mut self) -> fn(CCReceiver<Broadcast>, CCSender<Fired>) {}
    // fn run_f(&mut self) -> Box<dyn FnMut(CCReceiver<Broadcast>, CCSender<Fired>) + Send> {
    //     let f = |rx_confirm, tx_report| self.run(rx_confirm, tx_report);
    //     Box::new(f)
    // }
    
    fn run(&mut self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<()>) {
        // this version make all connections (only passive supported) into threads controlled by pre-neurons.
        let running_neurons = self.running_devices();
        loop {
            random_sleep();
            match rx_confirm.recv().unwrap() {

                Broadcast::Exit => {
                    for r_neuron in &running_neurons {
                        r_neuron.confirm.send(Broadcast::Exit).unwrap();
                    }
                    for r_neuron in running_neurons {
                        r_neuron.instance.join().expect("connection join error!");
                    }
                    break;
                },

                Broadcast::NewCycle => {
                    for r_neuron in &running_neurons {
                        r_neuron.confirm.send(Broadcast::NewCycle).unwrap();
                    }
                    for r_neuron in &running_neurons {
                        r_neuron.report.recv().unwrap();
                    }
                    tx_report.send(()).unwrap();
                },

                Broadcast::FinishCycle => {
                    for r_neuron in &running_neurons {
                        r_neuron.confirm.send(Broadcast::FinishCycle).unwrap();
                    }
                    for r_neuron in &running_neurons {
                        r_neuron.report.recv().unwrap();
                    }
                    tx_report.send(()).unwrap();
                }
            }
        }
    }
}
