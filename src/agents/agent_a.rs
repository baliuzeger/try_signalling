use std::rc::Rc;
use crate::generators::{Generator_1, Generator_2};
use crate::processors::{Processor_1, Processor_2};
use crate::signals::{Sender, Receiver, Signal, Channel, Signal_1, Signal_2};
use crate::agents::{Process_1, Process_2, Is_sender, Is_receiver};

pub struct Agent<'a> {
    name: String,
    gn1: Generator_1,
    out_channels_1: Vec<Rc<Channel<'a>>>,
    gn2: Generator_2,
    out_channels_2: Vec<Rc<Channel<'a>>>,
    pc1: Processor_1,
    in_channels_1: Vec<Rc<Channel<'a>>>,
    pc2: Processor_2,
    in_channels_2: Vec<Rc<Channel<'a>>>,
}

impl<'a> Process_1 for &mut Agent<'a> {
    fn process_1(&mut self, s: Signal_1) {
        self.pc1.process(s);
    }
}

impl<'a> Process_2 for &mut Agent<'a> {
    fn process_2(&mut self, s: Signal_2) {
        self.pc2.process(s);
    }    
}

impl<'a> Is_sender<'a> for Agent<'a> {
    fn wrap_sender(self) -> Sender<'a> {
        Sender::Agent_a_(Rc::new(self))
    }


    fn add_out_channel(&mut self, ch: Channel) {
        match ch.signal_sample {
            Signal::Signal_1_ => {
                self.out_channels_1.push(Rc::new(ch));
            },
            Signal::Signal_2_ => {
                self.out_channels_2.push(Rc::new(ch));
            },
        }
    }
}

impl<'a> Is_receiver<'a> for Agent<'a> {
    fn wrap_receiver(self) -> Receiver<'a> {
        Receiver::Agent_a_(Rc::new(self))
    }

    fn add_in_channel(&mut self, ch: Channel) {
        match ch.signal_sample {
            Signal::Signal_1_ => {
                self.in_channels_1.push(Rc::new(ch));
            },
            Signal::Signal_2_ => {
                self.in_channels_2.push(Rc::new(ch));
            },
        }
    }
}

impl<'a> Agent<'a> {
    pub fn new() -> Agent<'static> {
        Agent {
            name: String::from("Agent a!"),
            gn1: Generator_1::new(),
            out_channels_1: Vec::new(),
            gn2: Generator_2::new(),
            out_channels_2: Vec::new(),
            pc1: Processor_1::new(),
            in_channels_1: Vec::new(),
            pc2: Processor_2::new(),
            in_channels_2: Vec::new(),
        }
    }
    
    pub fn make_event(&self) {
        for cn in self.out_channels_1.iter() {
            //            self.event_1::<Process_1>(cn.receiver.unwrap());
            self.event_1(
                match Rc::get_mut(&mut cn.receiver).unwrap() {
                    Receiver::Agent_a_(rc_agnt) => Rc::get_mut(&mut rc_agnt).unwrap(),
                }
            )
        };
        for cn in self.out_channels_2.iter() {
            self.event_2(
                match Rc::get_mut(&mut cn.receiver).unwrap() {
                    Receiver::Agent_a_(rc_agnt) => Rc::get_mut(&mut rc_agnt).unwrap(),
                }
            )
        };
    }
    
    fn generate_1(&self) -> Signal_1 {
        self.gn1.generate()
    }

    fn generate_2(&self) -> Signal_2 {
        self.gn2.generate()
    }

    fn event_1<T: Process_1> (&self, rcvr: T) {        
        rcvr.process_1(self.generate_1());
    }

    fn event_2<T: Process_2> (&self, rcvr: T) {
        rcvr.process_2(self.generate_2());
    }
    
}
