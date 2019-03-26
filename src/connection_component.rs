
pub struct ConnectionComponent<G: Send, A: Send> {
    config: DeviceMode<ComponentIdle<G, A>,
                    ComponentFFW<G, A, FwdPreS1, FwdPostS1>>
}

impl<G, A, R, S> ConnectionComponent<G, A>
where G: S1Generator + Send,
      A: S1Acceptor + Send,
{
    fn new<G, A>(pre: Weak<Mutex<G>>, post: Weak<Mutex<A>>) -> ConnectionComponent<G, A> {
        ConnectionComponent {
            config: DeviceMode::Idle(ComponentIdle::new(pre, post)),
        }
    }

    pub fn mode(&self) -> RunMode {
        DeviceMode::variant(self.config)
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
    
    fn set_pre_ffw(&mut self, pre_channel: Option<CCReceiver<FwdPreS1>>) {
        match &self.config {
            DeviceMode::Feedforward(m) => m.set_pre_channel(pre_channel),
            _ => panic!("call fn set_pre_ffw when not DeviceMode::Feedforward!")
        }
    }

    fn set_post_ffw(&mut self, post_channel: Option<CCSender<FwdPostS1>>) {
        match &self.config {
            DeviceMode::Feedforward(m) => m.set_post_channel(post_channel),
            _ => panic!("call fn set_post_ffw when not DeviceMode::Feedforward!")
        }
    }
    
    pub fn import(&mut self) {
        match &self.config {
            DeviceMode::Feedforward(m) => m.import();
            DeviceMode => panic!("call fn import when DeviceMode::Idle!"),
        }
    }

    pub fn export(&self, s: FwdPostS1) {
        match &self.config {
            DeviceMode::Feedforward(m) => m.export();
            DeviceMode => panic!("call fn export when DeviceMode::Idle!"),
        }
    }    
}

pub struct ComponentIdle<G: Send, A: Send> {
    pre: Weak<Mutex<G>>,
    post: Weak<Mutex<A>>,
}

impl<G: Send, A: Send> ComponentIdle<G, A> {
    fn new(post: Weak<Mutex<G>>, pre: Weak<Mutex<A>>) -> ComponentIdle {
        pre,
        post,
    }

    fn make_ffw<R, S>(&self) -> ComponentFFW<G, A, R, S>
    where R: Send,
          S: Send
    {
        ComponentFFW {
            pre: Weak::clone(self.pre),
            post: Weak::clone(self.post),
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

impl<G: Send, A: Send, R, S> ComponentFFW<G, A, R, S> {
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
        self.post_channel.expect("FFW connection has no post_channel!").send(s).unwrap(),
    }
}
