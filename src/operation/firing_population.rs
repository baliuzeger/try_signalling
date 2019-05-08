extern crate crossbeam_channel;
use crate::random_sleep;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crate::operation::{RunningSet, Fired, Broadcast, RunMode};

pub trait FiringPopulation {
    fn config_mode(&mut self, mode: RunMode);
    fn config_channels(&mut self);
    fn running_devices(&self) -> Vec<RunningSet<Broadcast, Fired>>;
    // fn run_f(&mut self) -> Box<dyn FnMut(CCReceiver<Broadcast>, CCSender<Fired>) + Send> {
    //     let f = |rx_confirm, tx_report| self.run(rx_confirm, tx_report);
    //     Box::new(f)
    // }
    
    fn run(&mut self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<Fired>) {
        // this version make all connections (only passive supported) into threads controlled by pre-devices.
        let running_devices = self.running_devices();

        let mut devices_with_event = Vec::new();
        loop {
            random_sleep();
            match rx_confirm.recv().unwrap() {

                Broadcast::Exit => {
                    for r_device in &running_devices {
                        r_device.confirm.send(Broadcast::Exit).unwrap();
                    }
                    for r_device in running_devices {
                        r_device.instance.join().expect("connection join error!");
                    }
                    break;
                },

                Broadcast::NewCycle => {
                    random_sleep();
                    devices_with_event.clear();
                    for r_device in &running_devices {
                        r_device.confirm.send(Broadcast::NewCycle).unwrap();
                    }
                    for r_device in &running_devices {
                        if let Fired::Y = r_device.report.recv().unwrap() {
                            devices_with_event.push((r_device.confirm.clone(), r_device.report.clone()));
                        }
                    }

                    match devices_with_event.len() {
                        0 => tx_report.send(Fired::N).unwrap(),
                        _ => tx_report.send(Fired::Y).unwrap(),
                    }
                    // println!("pp finished NewCycle.");
                },

                Broadcast::FinishCycle => {
                    random_sleep();
                    for device_e in &devices_with_event {
                        device_e.0.send(Broadcast::FinishCycle).unwrap();
                    }
                    // println!("pp waiting device report FinishCycle.");
                    for device_e in &devices_with_event {
                        match device_e.1.recv().unwrap() {
                            Fired::N => (),
                            Fired::Y => panic!("device report Event after FinishCycle!")
                        }
                    }
                    // println!("pp get report from device of FinishCycle.")
                    tx_report.send(Fired::N).unwrap();
                }
            }
        }
    }
}
