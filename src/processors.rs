use crate::signals::{Signal_1, Signal_2};

pub struct Processor_1 {
    name: String,
}

impl Processor_1 {
    pub fn new() -> Processor_1 {
        Processor_1 {
            name: String::from("p1")
        }
    }

    pub fn process(&self, s: Signal_1) {
        println!("{} and {}", s.message, self.name);
    }
    
    fn check_sample(s: Signal_1) -> Signal_1 {
        Signal_1 {message: String::from("ref p1.")}
    }
}

pub struct Processor_2 {
    name: String,
}

impl Processor_2 {
    pub fn new() -> Processor_2 {
        Processor_2 {
            name: String::from("p2")
        }
    }

    pub fn process(&self, s: Signal_2) {
        println!("{} and {}", s.message, self.name);
    }
    
    fn check_sample(s: Signal_2) -> Signal_2 {
        Signal_2 {message: String::from("sample p2.")}
    }
}
