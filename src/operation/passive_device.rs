use crate::operation::{RunMode, Runnable, Broadcast};
extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crate::random_sleep;

/// used by Components.runningdevices()
pub trait PassiveDevice: Runnable {
    fn mode(&self) -> RunMode;
    fn respond(&self);

    // fn run_f(&mut self) -> Box<dyn FnMut(CCReceiver<Broadcast>, CCSender<()>)> {
    //     Box::new(|rx_confirm, tx_report| self.run(rx_confirm, tx_report))
    // }

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
