#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let x = Rc::new(Agent_a::new());
        let y = Rc::new(Agent_a::new());
        let cn1 = Rc::new(Channel::new(&x, &y, Signal_1::sample()));
        let cn2 = Rc::new(Channel::new(&x, &y, Signal_2::sample()));
        
    }
}

mod agent_a;
mod generators;
mod processors;
mod signals;
