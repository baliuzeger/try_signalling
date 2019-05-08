extern crate crossbeam_channel;
use crate::random_sleep;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crate::operation::{RunningSet, Fired, Broadcast, RunMode, Runnable, Configurable};

pub trait FiringActivePopulation: Configurable{}
pub trait ConsecutiveActivePopulation: Configurable{}
pub trait SilentActivePopulation: Configurable{}
pub trait PassivePopulation: Configurable{}

impl<T: FiringActivePopulation> Runnable for T {
    type Report = Fired;
    
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

                Broadcast::Evolve => {
                    random_sleep();
                    devices_with_event.clear();
                    for r_device in &running_devices {
                        r_device.confirm.send(Broadcast::Evolve).unwrap();
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
                    // println!("pp finished Evolve.");
                },

                Broadcast::Respond => {
                    random_sleep();
                    for device_e in &devices_with_event {
                        device_e.0.send(Broadcast::Respond).unwrap();
                    }
                    // println!("pp waiting device report Respond.");
                    for device_e in &devices_with_event {
                        match device_e.1.recv().unwrap() {
                            Fired::N => (),
                            Fired::Y => panic!("device report Event after Respond!")
                        }
                    }
                    // println!("pp get report from device of Respond.")
                    tx_report.send(Fired::N).unwrap();
                }
            }
        }
    }
}

impl<T: ConsecutiveActivePopulation> Runnable for T {
    type Report = ();

    fn run(&mut self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<()>) {
        // this version make all connections (only passive supported) into threads controlled by pre-devices.
        let running_devices = self.running_devices();
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

                Broadcast::Evolve => {
                    for r_device in &running_devices {
                        r_device.confirm.send(Broadcast::Evolve).unwrap();
                    }
                    for r_device in &running_devices {
                        r_device.report.recv().unwrap();
                    }
                    tx_report.send(()).unwrap();
                },

                Broadcast::Respond => {
                    for r_device in &running_devices {
                        r_device.confirm.send(Broadcast::Respond).unwrap();
                    }
                    for r_device in &running_devices {
                        r_device.report.recv().unwrap();
                    }
                    tx_report.send(()).unwrap();
                }
            }
        }
    }
}

impl<T: SilentActivePopulation> Runnable for T {
    type Report = ();

    fn run(&mut self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<()>) {
        // this version make all connections (only passive supported) into threads controlled by pre-devices.
        let running_devices = self.running_devices();
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
                
                Broadcast::Evolve => {
                    for r_device in &running_devices {
                        r_device.confirm.send(Broadcast::Evolve).unwrap();
                    }
                    for r_device in &running_devices {
                        r_device.report.recv().unwrap();
                    }
                    tx_report.send(()).unwrap();
                },

                Broadcast::Respond => panic!("SilentActivePopulation should not recv Finishcycle!"),
            }
        }
    }    
}
