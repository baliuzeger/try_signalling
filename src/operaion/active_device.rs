/// used by Population.runningdevices()
pub trait ActiveDevice: Device {
    fn running_passive_devices(&self) -> Vec<RunningDevice<(), Broadcast>>;
    fn end(&mut self);
    fn evolve(&mut self) -> AgentEvent;

    // fn run_f(&mut self) -> Box<dyn FnMut(CCReceiver<Broadcast>, CCSender<AgentEvent>)> {
    //     Box::new(|rx_confirm, tx_report| self.run(rx_confirm, tx_report))
    // }
    
    fn run(&mut self, rx_confirm: CCReceiver<Broadcast>, tx_report: CCSender<AgentEvent>) {
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
                    // for r_cn in &running_active_connections {
                    //     r_cn.confirm.send(Broadcast::NewCycle).unwrap();
                    // }
                    match self.evolve() {
                        AgentEvent::N => tx_report.send(AgentEvent::N).unwrap(),
                        AgentEvent::Y => {
                            random_sleep();
                            tx_report.send(AgentEvent::Y).unwrap();
                            // println!("agnt waiting pp confirm FinishCycle.");
                            match rx_confirm.recv().unwrap() {
                                Broadcast::FinishCycle => {
                                    for r_cn in &running_devices {
                                        r_cn.confirm.send(Broadcast::FinishCycle).unwrap();
                                    }
                                    // println!("agnt waiting conn report finish Prop.");
                                    for r_cn in &running_devices {
                                        r_cn.report.recv().unwrap();
                                    }
                                    // println!("agnt get conn report finish Prop.");
                                    tx_report.send(AgentEvent::N).unwrap();
                                },
                                _ => panic!("sp not confirm by FinishCycle before finish cycle!"),
                            }
                        }
                    }
                },

                _ => panic!("agent should only get Exit or NewCycle at beginning of cycle!")
            }
        }
    }    
}
