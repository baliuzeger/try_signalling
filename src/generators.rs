use crate::signals::{Signal_1, Signal_2};

pub struct Generator_1 {
    name: String,
}

impl Generator_1 {
    fn new() -> Generator_1 {
        Generator_1 {
            name: String::from("g1"),
        }
    }
    
    fn generate(&self) -> Signal_1 {
        Signal_1 {name: self.name,}
    }

    fn check_sample(s: Signal_1) -> Signal_1 {
        Signal_1 {name: String::from("ref g1."),}
    }
}

pub struct Generator_2 {
    name: String,
}

impl Generator_2 {
    fn new() -> Generator_2 {
        Generator_2 {
            name: String::from("g2"),
        }
    }

    fn generate(&self) -> Signal_2 {
        Signal_2 {name: self.name,}
    }

    fn check_sample(s: Signal_2) -> Signal_2 {
        Signal_2 {name: String::from("ref g2."),}
    }
}
