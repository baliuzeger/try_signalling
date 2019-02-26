use std::rc::Rc;
use crate::generators::{Generator_1, Generator_2};
use crate::processors::{Processor_1, Processor_2};
use crate::signals::{Channel, Signal_1, Signal_2};

struct Agent_a<T, U, V_i, V_o> {
    name: String,
    gn1: Generator_1,
    gn2: Generator_2,
    pc1: Processor_1,
    pc2: Processor_2,
    in_channels: Vec<Rc<Channel<T, U, V_i>>>,
    out_channels: Vec<Rc<Channel<T, U, V_o>>>,
}

impl<T, U, V_i, V_o> Agent_a<T, U, V_i, V_o> {
    
    pub fn new() -> Agent_a<T, U, V_i, V_o> {
        Agent_a {
            name: String::from("Agent a!"),
            gn1: Generator_1::new(),
            gn2: Generator_2::new(),
            pc1: Processor_1::new(),
            pc2: Processor_2::new(),
            in_channels: Vec::new(),
            out_channels: Vec::new(),
        }
    }

    fn process(&self, s: Signal_1) {
        self.pc1.process(s);
    }

    fn process(&self, s: Signal_2) {
        self.pc2.process(s);
    }

    fn event(&self) {
        for cn in self.channels {
            cn.receiver.process(cn.sender.generate(cn.signal_sample));
        }
    }

    
    
    fn generate_s1(&self, _s: Signal_1) -> Signal_1 {
        self.gn1.generate()
    }

    fn generate(&self, _s: Signal_2) -> Signal_2 {
        self.gn2.generate()
    }

    fn check_ref_generate(&self, s: Signal_1) -> Signal_1 {
        self.gn1.check_sample(s)
    }

    fn check_ref_generate(&self, s: Signal_2) -> Signal_2 {
        self.gn2.check_sample(s)
    }

    fn check_ref_process(&self, s: Signal_1) -> Signal_1 {
        self.pc1.check_sample(s)
    }

    fn check_ref_process(&self, s: Signal_2) -> Signal_2 {
        self.pc2.check_sample(s)
    }

    fn add_in_channel(&self, in_ch: Rc<Channel<T, U, V_i>>) {
        self.in_channels.push(Rc::clone(&in_ch));
    }

    fn add_out_channel(&self, out_ch: Rc<Channel<T, U, V_o>>) {
        self.out_channels.push(Rc::clone(&out_ch));
    }
}



// connection {
//     spike_sample: spike_event
// }

// neuron.generate(spike_event) {
//     spike_event.generate(&self)
// }

// neuron_A.generate(connection_a.sample);
// neuron_A.generate(connection_b.sample);


    

// impl spike_event {
//     fn generate(neron)
// }
    

