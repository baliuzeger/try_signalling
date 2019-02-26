use std::rc::Rc;

pub struct Signal_1 {
    pub message: String,
}

impl Signal_1 {
    fn sample() -> Signal_1 {
        Signal_1 {name: String::from("sample s1.")}
    }
}

pub struct Signal_2 {
    pub message: String,
}

impl Signal_2 {
    fn sample() -> Signal_2 {
        Signal_2 {name: String::from("ref s2.")}
    }
}

pub struct Channel<T, U, V> {
    sender: Rc<T>,
    receiver: Rc<U>,
    signal_sample: V,
}

impl<T, U, V> Channel<T, U, V> {
    fn new (sender: Rc<T>, receiver: Rc<U>, signal_sample: V) -> Channel<T, U, V> {
        let ch = Channel {
            sender: Rc::clone(&sender),
            receiver: Rc::clone(&receiver),
            signal_sample: receiver.check_sample_process(sender.check_sample_generate(signal_sample)),
        };
        receiver.add_in_channel(ch);
        sender.add_out_channel(ch);
        ch
    }
}
