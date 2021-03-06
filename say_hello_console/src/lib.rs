use isay_hello::ISayHelloService;

#[no_mangle]
pub fn new() -> Box<dyn ISayHelloService> {
    Box::new(PluginSayHello::new())
}

pub struct PluginSayHello {
    id: String,
}

impl PluginSayHello {
    fn new() -> PluginSayHello {
        let id = format!("{:08x}", rand::random::<u32>());
        println!("[{}] Created instance!", id);
        PluginSayHello { id }
    }
}

impl ISayHelloService for PluginSayHello {
    fn say_hello(&self) {
        println!("[{}] Hello from plugin!", self.id);
    }
}

impl Drop for PluginSayHello {
    fn drop(&mut self) {
        println!("[{}] Destroyed instance!", self.id);
    }
}
