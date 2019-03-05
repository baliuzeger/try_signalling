pub Agent;

struct Agent {
    gen_value: i32,
    proc_value: i32,
    out_channels_1: Vec<Rc<dyn Propogate_1>>,
//    out_channels_2: Vec<Rc<dyn Propogate_2>>,
    in_channels_1: Vec<Rc<dyn Propogate_1>>,
//    in_channels_2: Vec<Rc<dyn Propogate_2>>,
}

impl Process_1 for Agent {
    fn process_1(&self, s: Signal_1) {
        println!("{}", self.proc_value + s.message);
    }

    fn add_in_1<T: Propogate_1> (&self, ch: &T) {
        self.in_channels_1.push(Rc::new(ch));
    }
}

impl Generate_1 for Agent {
    fn generate_1(&self) -> Signal_1 {
        Signal_1 {
            message: self.gen_value,
        }
    }

    fn add_out_1<T: Propogate_1> (&self, ch: &T) {
        self.out_channels_1.push(Rc::new(ch));
    }
}

impl Agent {
    pub fn new() -> Agent {
        Agent {
            gen_value: 1,
            proc_value: 100,
            out_channels_1: Vec::new();
            // out_channels_2: Vec::new();
            in_channels_1: Vec::new();
            // in_channels_2: Vec::new();
        }
    }

    fn event(&self) {
        let a_sgnl_1 = self.generate_1();
        for cn in self.out_channels_1.iter() {
            ch.propagate(a_sgnl_1);
        }        
    }
}
