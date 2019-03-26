extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::sync::{Weak, Mutex};
use crate::supervisor::{DeviceMode, RunMode};

pub struct ConnectionComponent<G: Send, A: Send, R: Send, S: Send> {
    config: DeviceMode<ComponentIdle<G, A>,
                       ComponentFFW<G, A, R, S>>
}

impl<G: Send, A: Send, R: Send, S: Send> ConnectionComponent<G, A, R, S> {
    fn new(pre: Weak<Mutex<G>>, post: Weak<Mutex<A>>) -> ConnectionComponent<G, A, R, S> {
        ConnectionComponent {
            config: DeviceMode::Idle(ComponentIdle::new(pre, post)),
        }
    }

    pub fn mode(&self) -> RunMode {
        RunMode::mode_from_device(self.config)
    }

    fn config_run(&mut self, mode: RunMode) {
        match (mode, &self.config) {
            (DeviceMode::Idle(_), _) => println!("config_run for mode Idle, no effect."),
            (mi, DeviceMode::Idle(ms)) => self.config = DeviceMode::Feedforward(ms.make_ffw()),
            (_, _) => panic!("call fn config_run when not DeviceMode::Idle!"),
        }
    }
    
    fn config_idle(&mut self) {
        match &self.config {
            DeviceMode::Feedforward(m) => self.config = DeviceMode::Idle(m.make_idle()),
            DeviceMode => panic!("call fn config_idle when DeviceMode::Idle!"),
        }
    }
    
    fn set_pre_ffw(&mut self, pre_channel: Option<CCReceiver<R>>) {
        match &self.config {
            DeviceMode::Feedforward(m) => m.set_pre_channel(pre_channel),
            _ => panic!("call fn set_pre_ffw when not DeviceMode::Feedforward!")
        }
    }

    fn set_post_ffw(&mut self, post_channel: Option<CCSender<S>>) {
        match &self.config {
            DeviceMode::Feedforward(m) => m.set_post_channel(post_channel),
            _ => panic!("call fn set_post_ffw when not DeviceMode::Feedforward!")
        }
    }
    
    pub fn import(&mut self) {
        match &self.config {
            DeviceMode::Feedforward(m) => m.import(),
            DeviceMode => panic!("call fn import when DeviceMode::Idle!"),
        }
    }

    pub fn export(&self, s: S) {
        match &self.config {
            DeviceMode::Feedforward(m) => m.export(),
            DeviceMode => panic!("call fn export when DeviceMode::Idle!"),
        }
    }    
}

pub struct ComponentIdle<G: Send, A: Send> {
    pre: Weak<Mutex<G>>,
    post: Weak<Mutex<A>>,
}

impl<G: Send, A: Send> ComponentIdle<G, A> {
    fn new(pre: Weak<Mutex<G>>, post: Weak<Mutex<A>>) -> ComponentIdle<G, A> {
        ComponentIdle {
            pre,
            post,
        }
    }

    fn make_ffw<R, S>(&self) -> ComponentFFW<G, A, R, S>
    where R: Send,
          S: Send
    {
        ComponentFFW {
            pre: self.pre.clone(),
            post: self.post.clone(),
            pre_channel: None,
            post_channel: None,
         }
    }
}

pub struct ComponentFFW<G: Send, A: Send, R: Send, S: Send> {
    pre: Weak<Mutex<G>>,
    post: Weak<Mutex<A>>,
    pre_channel: Option<CCReceiver<R>>,
    post_channel: Option<CCSender<S>>,
}

impl<G: Send, A: Send, R: Send, S: Send> ComponentFFW<G, A, R, S> {
    fn make_idle(&self) -> ComponentIdle<G, A> {
        ComponentIdle {
            pre: Weak::clone(self.pre),
            post: Weak::clone(self.post),
        }
    }

    fn set_pre_channel(&mut self, pre_channel: Option<CCReceiver<R>>) {
        self.pre_channel = pre_channel;
    }

    fn set_post_channel(&mut self, post_channel: Option<CCSender<S>>) {
        self.post_channel = post_channel;
    }
    
    fn import(&mut self) -> R {
        self.pre_channel.expect("FFW connection has no pre_channel!").recv().unwrap();
    }

    fn export(&self, s: S) {
        self.post_channel.expect("FFW connection has no post_channel!").send(s).unwrap();
    }
}
