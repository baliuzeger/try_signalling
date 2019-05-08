use crate::operation::{RunMode, Broadcast};
extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crate::random_sleep;
// use crate::connectivity::Acceptor;

/// used by Components.runningdevices()
pub trait PassiveDevice {
    fn config_mode(&mut self, mode: RunMode);
    fn config_channels(&mut self);
    fn respond(&self);

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