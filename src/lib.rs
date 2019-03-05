

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let x = Rc::new(Agent_a::Agent::new());
        let y = Rc::new(Agent_a::Agent::new());
        let cn1 = Rc::new(Channel::new(x, y));
//        let cn2 = Rc::new(Channel::new(&x, &y, Signal::Signal_2_));
        x.event();
    }
}

mod agents;
mod signals;
