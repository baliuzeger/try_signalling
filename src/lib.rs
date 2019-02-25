#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let x = Rc::new(Agent_a::new());
        let y = Rc::new(Agent_a::new());
        let cn1 = Rc::new(Channel::new(&x, &y, Signal_1::make_ref()));
        let cn2 = Rc::new(Channel::new(&x, &y, Signal_2::make_ref()));
    }
}

struct Generator_1 {
    name: "g1",
}

impl Generator_1 {
    fn generate(&self) -> Signal_1 {
        Signal_1 {name: self.name,}
    }
    fn check_ref(s: Signal_1) -> Signal_1 {
        Signal_1 {name: "ref g1."}
    }
}

struct Generator_2 {
    name: "g2",
}
nn
impl Generator_2 {
    fn generate(&self) -> Signal_2 {
        Signal_2 {self.name,}
    }
    fn check_ref(s: Signal_2) -> Signal_2 {
        Signal_2 {name: "ref g2."}
    }
}

struct Processor_1 {
  n  name: "p1",
}

impl Processor_1 {
    fn process(&self, s: Signal_1) {
        println!("{} and {}", s.message, self.name)
    }
    fn check_ref(s: Signal_1) -> Signal_1 {
        Signal_1 {name: "ref p1."}
    }
}

struct Processor_2 {
    name: "p2",
}

impl Processor_2 {
    fn process(&self, s: Signal_2) {
        println!("{} and {}", s.message, self.name)
    }
    fn check_ref(s: Signal_2) -> Signal_2 {
        Signal_2 {name: "ref p2."}
    }
}

struct Signal_1 {
    pub message: &str,
}

impl Signal_1 {
    fn make_ref() -> Signal_1 {
        Signal_1 {name: "ref s1."}
    }
}

struct Signal_2 {
    pub message: &str,
}

impl Signal_2 {
    fn make_ref() -> Signal_2 {
        Signal_2 {name: "ref s2."}
    }
}

struct Channel<T, U, V> {
    sender: Rc<T>,
    receiver: Rc<U>,
    signal_ref: V,
}

impl<T, U, V> Channel<T, U, V> {
    fn new (sender: Rc<T>, receiver: Rc<U>, signal_ref: V) {
        Channel {
            sender: Rc::clone(&sender),
            receiver: Rc::clone(&receiver),
            signal_ref: receiver.check_ref_process(sender.check_ref_generate(signal_ref)),
        }
    }
}

struct Agent_a {
    name: "Agent a!",
    gn1: Generator_1,
    gn2: Generator_2,
    pc1: Processor_1,
    pc2: Processor_2,
}

impl Agent_a {
    fn new() -> Agent_a {
        gn1: Generator_1,
        gn2: Generator_2,
        pc1: Processor_1,
        pc2: Processor_2,
        in_channels: Vec<Rc<Channel>>,
        out_channels: Vec<Rc<Channel>>,
    }

    fn process(&self, s: Signal_1) {
        self.pc1.process(s);
    }

    fn process(&self, s1: Signal_2) {
        self.pc2.process(s);
    }

    fn event(&self) {
        for cn in self.channels {
            cn.receiver.process(cn.sender.generate(cn.signal_ref))
        }
    }

    fn generate(&self, _s: Signal_1) -> Signal_1 {
        self.gn1.generate()
    }

    fn generate(&self, _s: Signal_2) -> Signal_2 {
        self.gn2.generate()
    }

    fn check_ref_generate(&self, s: Signal_1) -> Signal_1 {
        self.gn1.check_ref(s)
    }

    fn check_ref_generate(&self, s: Signal_2) -> Signal_2 {
        self.gn2.check_ref(s)
    }

    fn check_ref_process(&self, s: Signal_1) -> Signal_1 {
        self.pc1.check_ref(s)
    }

    fn check_ref_process(&self, s: Signal_2) -> Signal_2 {
        self.pc2.check_ref(s)
    }
}
