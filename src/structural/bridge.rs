pub trait Device {
    fn is_enabled(&self) -> bool;
    fn enable(&mut self);
    fn disable(&mut self);
    fn get_volume(&self) -> i32;
    fn set_volume(&mut self, volume: i32);
}

// Concrete Implementations
pub struct Radio {
    enabled: bool,
    volume: i32,
}

impl Radio {
    pub fn new() -> Self {
        Radio {
            enabled: false,
            volume: 0,
        }
    }
}

pub struct Tv {
    enabled: bool,
    volume: i32,
}

impl Tv {
    pub fn new() -> Self {
        Tv {
            enabled: false,
            volume: 0,
        }
    }
}

impl Device for Radio {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn enable(&mut self) {
        self.enabled = true;
    }

    fn disable(&mut self) {
        self.enabled = false;
    }

    fn get_volume(&self) -> i32 {
        self.volume
    }

    fn set_volume(&mut self, volume: i32) {
        self.volume = volume;
    }
}

impl Device for Tv {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn enable(&mut self) {
        self.enabled = true;
    }

    fn disable(&mut self) {
        self.enabled = false;
    }

    fn get_volume(&self) -> i32 {
        self.volume
    }

    fn set_volume(&mut self, volume: i32) {
        self.volume = volume;
    }
}

// Bridge: Remote Control
pub struct RemoteControl {
    device: Box<dyn Device>,
}

impl RemoteControl {
    pub fn new(device: Box<dyn Device>) -> Self {
        RemoteControl { device }
    }

    pub fn toggle_power(&mut self) {
        if self.device.is_enabled() {
            self.device.disable();
        } else {
            self.device.enable();
        }
    }

    pub fn volume_up(&mut self) {
        let new_volume = self.device.get_volume() + 1;
        self.device.set_volume(new_volume);
    }

    pub fn volume_down(&mut self) {
        let new_volume = self.device.get_volume() - 1;
        self.device.set_volume(new_volume);
    }
}
