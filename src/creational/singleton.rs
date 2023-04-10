use once_cell::sync::Lazy;
use rand::Rng;
use std::sync::{Arc, Mutex};

pub struct Singleton {
    data: u32,
}

impl Singleton {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Singleton {
            data: rng.gen_range(0..10) as u32,
        }
    }

    pub fn instance() -> Arc<Mutex<Self>> {
        // Use once_cell's Lazy type to initialize the instance only once
        static INSTANCE: Lazy<Arc<Mutex<Singleton>>> =
            Lazy::new(|| Arc::new(Mutex::new(Singleton::new())));
        INSTANCE.clone()
    }

    pub fn get_data(&self) -> u32 {
        self.data.clone()
    }
}
