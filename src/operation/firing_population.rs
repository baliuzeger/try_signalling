extern crate crossbeam_channel;
use crate::random_sleep;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crate::operation::{Runnable, RunningSet, Fired, Broadcast};

pub trait FiringPopulation: Runnable {
    fn running_devices(&self) -> Vec<RunningSet<Broadcast, ()>>;

    fn run(&mut self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<Fired>) {
        // this version make all connections (only passive supported) into threads controlled by pre-neurons.
        let running_neurons = self.running_neurons();

        let mut neurons_with_event = Vec::new();
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
                    neurons_with_event.clear();
                    for r_neuron in &running_neurons {
                        r_neuron.confirm.send(Broadcast::NewCycle).unwrap();
                    }
                    for r_neuron in &running_neurons {
                        if let Fired::Y = r_neuron.report.recv().unwrap() {
                            neurons_with_event.push((r_neuron.confirm.clone(), r_neuron.report.clone()));
                        }
                    }

                    match neurons_with_event.len() {
                        0 => tx_report.send(Fired::N).unwrap(),
                        _ => {
                            random_sleep();
                            tx_report.send(Fired::Y).unwrap();
                            // println!("pp waiting sp confirm to Finishcycle.");
                            match rx_confirm.recv().unwrap() {
                                Broadcast::FinishCycle => {
                                    for neuron_e in &neurons_with_event {
                                        neuron_e.0.send(Broadcast::FinishCycle).unwrap();
                                    }
                                    // println!("pp waiting neuron report FinishCycle.");
                                    for neuron_e in &neurons_with_event {
                                        match neuron_e.1.recv().unwrap() {
                                            Fired::N => (),
                                            Fired::Y => panic!("neuron report Event after FinishCycle!")
                                        }
                                    }
                                    // println!("pp get report from neuron of FinishCycle.")
                                },
                                _ => panic!("sp not confirm by FinishCycle before finish cycle!"),
                            }
                            tx_report.send(Fired::N).unwrap();
                        }
                    }
                },
                _ => panic!("pp should only recv confirm of NewCycle or Exit!")
            }
        }
    }
}
