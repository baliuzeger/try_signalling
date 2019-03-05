use std::rc::{Rc, Weak};


struct Signal_1 {
    pub message: i32,
}

trait Propogate_1 {
    fn refine(&self, Signal_1) -> Signal_1;
    fn propogate(&self, Signal_1);
}

trait Process_1 {
    fn process_1(&self, Signal_1);
    fn add_in_1<T: Propogate_1> (&self, &T);
}

trait Generate_1 {
    fn generate_1 (&self) -> Signal_1;
    fn add_out_1<T: Propogate_1> (&self, &T);
}

struct Channel_1 {
    sender: Weak<Generate_1>,
    receiver: Weak<Process_1>,
    value: i32,
}

impl Propogate_1 for Channel_1 {
    fn refine(&self, s: Signal_1) -> Signal_1 {
        Signal_1 {
            message: self.name + s.message,
        }
    }
    
    fn propogate(&self, s: Signal_1) {
        self.receiver.process(self.refine(s));
    }
}

impl Channel_1 {
    fn new(s: Rc<Generate_1>, r: Rc<Process_1>) -> Channel_1 {
        let ch = Channel_1 {
            sender: Rc::downgrade(&s),
            receiver: Rc::downgrade(&r),
            value: 10,
        };
        s.add_out_1(ch);
        r.add_in_1(ch);
        ch
    }
}
