/// used by Population.running_devices() or OutComponents.running_devices()

extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crate::operation::{RunningSet, Broadcast, Fired, RunMode, ActiveDevice, Configurable, Runnable};
use crate::random_sleep;

pub trait ConsecutivePassiveDevice: Configurable {
    fn respond(&self);
    fn running_passive_devices(&self) -> Vec<RunningSet<Broadcast, ()>>;
}

pub trait FiringPassiveDevice: Configurable {
    fn respond(&self) -> Fired;
    fn running_passive_devices(&self) -> Vec<RunningSet<Broadcast, ()>>;
}

pub trait SilentPassiveDevice: Configurable {
    fn respond(&self);
}

pub trait ConsecutiveActiveDevice: Configurable {
    fn end(&mut self);
    fn evolve(&mut self);
    fn running_passive_devices(&self) -> Vec<RunningSet<Broadcast, ()>>;
}

pub trait FiringActiveDevice: Configurable {
    fn end(&mut self);
    fn evolve(&mut self) -> Fired;
    fn running_passive_devices(&self) -> Vec<RunningSet<Broadcast, ()>>;
}

pub trait SilentActiveDevice: Configurable {
    fn end(&mut self);
    fn evolve(&mut self);
}

impl<T> Runnable for T: FiringActiveDevice {
    type Report = Fired;
    
    fn run(&mut self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<Fired>) {
        let running_devices = self.running_passive_devices();
        let mut last_result = Fired::N;
        
        loop {
            random_sleep();
            match rx_confirm.recv().unwrap() {

                Broadcast::Exit => {
                    self.end();
                    for r_cn in &running_devices {
                        r_cn.confirm.send(Broadcast::Exit).unwrap();
                    }
                    for r_cn in running_devices {
                        r_cn.instance.join().expect("connection join error!");
                    }
                    break;
                },

                Broadcast::NewCycle => {
                    match self.evolve() {
                        Fired::N => tx_report.send(Fired::N).unwrap(),
                        Fired::Y => {
                            random_sleep();
                            last_result = Fired::Y;
                            tx_report.send(Fired::Y).unwrap();
                            // println!("agnt finished NewCycle.");
                        }
                    }
                },

                Broadcast::FinishCycle => {
                    random_sleep();
                    match &mut last_result {
                        Fired::N => (),
                        Fired::Y => {
                            for r_cn in &running_devices {
                                r_cn.confirm.send(Broadcast::FinishCycle).unwrap();
                            }
                            // println!("agnt waiting conn report finish Prop.");
                            for r_cn in &running_devices {
                                r_cn.report.recv().unwrap();
                            }
                            // println!("agnt get conn report finish Prop.");
                            tx_report.send(Fired::N).unwrap();
                        }
                    }
                    last_result = Fired::N;
                }
            }
        }
    }
}

impl<T> Runnable for T: ConsecutiveActiveDevice {
    type Report = ();

    fn run(&mut self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<()>) {
        let running_devices = self.running_passive_devices();

        loop {
            random_sleep();
            match rx_confirm.recv().unwrap() {

                Broadcast::Exit => {
                    self.end();
                    for r_cn in &running_devices {
                        r_cn.confirm.send(Broadcast::Exit).unwrap();
                    }
                    for r_cn in running_devices {
                        r_cn.instance.join().expect("connection join error!");
                    }
                    break;
                },

                Broadcast::NewCycle => {
                    self.evolve();
                    tx_report.send(()).unwrap();
                },

                Broadcast::FinishCycle => {
                    for r_cn in &running_devices {
                        r_cn.confirm.send(Broadcast::FinishCycle).unwrap();
                    }
                    // println!("agnt waiting conn report finish Prop.");
                    for r_cn in &running_devices {
                        r_cn.report.recv().unwrap();
                    }
                    // println!("agnt get conn report finish Prop.");
                    tx_report.send(()).unwrap();
                }
            }
        }
    }
}

impl<T> Runnable for T: SilentActiveDevice {
    type Report = ();

    fn run(&mut self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<()>) {
        loop {
            random_sleep();
            match rx_confirm.recv().unwrap() {
                Broadcast::Exit => {
                    self.end();
                    break;
                },
                Broadcast::NewCycle => {
                    self.evolve();
                    tx_report.send(()).unwrap();
                },
                Broadcast::FinishCycle => panic!("SilentActivePopulation should not recv Finishcycle!"),
            }
        }
    }
}

impl<T> Runnable for T: ConsecutivePassiveDevice{
    type Report = ();

    fn run(&self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<()>){
        let running_devices = self.running_passive_devices();
        let mut last_result = Fired::N;
        loop {
            random_sleep();
            match rx_confirm.recv().unwrap() {
                Broadcast::Exit => {
                    for r_cn in &running_devices {
                        r_cn.confirm.send(Broadcast::Exit).unwrap();
                    }
                    for r_cn in running_devices {
                        r_cn.instance.join().expect("connection join error!");
                    }
                    break;
                },
                Broadcast::NewCycle => panic!("ConsecutivePassivedevice confirmed by NewCycle!"),

                Broadcast::FinishCycle => {
                    // println!("conn wait recv signal.");
                    self.respond();
                    for r_cn in &running_devices {
                        r_cn.confirm.send(Broadcast::FinishCycle).unwrap();
                    }
                    // println!("agnt waiting conn report finish Prop.");
                    for r_cn in &running_devices {
                        r_cn.report.recv().unwrap();
                    }
                    // println!("agnt get conn report finish Prop.");
                    tx_report.send(()).unwrap();
                }
            }
        }
    }    
}

impl<T> Runnable for T: FiringPassiveDevice{
    type Report = ();

    fn run(&self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<()>){
        let running_devices = self.running_passive_devices();
        let mut last_result = Fired::N;
        loop {
            random_sleep();
            match rx_confirm.recv().unwrap() {
                Broadcast::Exit => {
                    for r_cn in &running_devices {
                        r_cn.confirm.send(Broadcast::Exit).unwrap();
                    }
                    for r_cn in running_devices {
                        r_cn.instance.join().expect("connection join error!");
                    }
                    break;
                },
                Broadcast::NewCycle => panic!("FiringPassivedevice confirmed by NewCycle!"),

                Broadcast::FinishCycle => {
                    random_sleep();
                    // println!("conn wait recv signal.");
                    match self.respond() {
                        Fired::N => (),
                        Fired::Y => {
                            for r_cn in &running_devices {
                                r_cn.confirm.send(Broadcast::FinishCycle).unwrap();
                            }
                            // println!("agnt waiting conn report finish Prop.");
                            for r_cn in &running_devices {
                                r_cn.report.recv().unwrap();
                            }
                        },
                    }
                    tx_report.send(()).unwrap();
                }
            }
        }
    }
}

impl<T> Runnable for T: SilentPassiveDevice{
    type Report = ();

    fn run(&self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<()>){
        loop {
            random_sleep();
            match rx_confirm.recv().unwrap() {
                Broadcast::Exit => break,
                Broadcast::NewCycle => panic!("Passivedevice confirmed by NewCycle!"),
                Broadcast::FinishCycle => {
                    // println!("conn wait recv signal.");
                    self.respond();
                    // println!("conn got & propagated signal.");
                    tx_report.send(()).unwrap();
                }
            }
        }
    }    
}
