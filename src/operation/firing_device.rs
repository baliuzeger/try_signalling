/// used by Population.runningdevices()

extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crate::operation::{RunningSet, Broadcast, Fired, RunMode, ActiveDevice};
use crate::random_sleep;

pub trait FiringDevice: ActiveDevice {
    fn config_mode(&mut self, mode: RunMode);
    fn config_channels(&mut self);
    fn running_passive_devices(&self) -> Vec<RunningSet<Broadcast, ()>>;
    fn end(&mut self);
    fn evolve(&mut self) -> Fired;
    
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
