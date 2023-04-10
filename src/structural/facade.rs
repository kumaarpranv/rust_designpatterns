pub struct Thermostat {
    temperature: f32,
}

impl Thermostat {
    pub fn new() -> Self {
        Self { temperature: 22.0 }
    }

    pub fn set_temperature(&mut self, temperature: f32) {
        self.temperature = temperature;
        println!("Thermostat set to {}°C.", self.temperature);
    }
}

pub struct SecuritySystem {
    pub locked: bool,
}

impl SecuritySystem {
    pub fn new() -> Self {
        Self { locked: true }
    }

    pub fn lock(&mut self) {
        self.locked = true;
        println!("Security system locked.");
    }

    pub fn unlock(&mut self) {
        self.locked = false;
        println!("Security system unlocked.");
    }
}

pub struct LightingSystem {
    on: bool,
}

impl LightingSystem {
    pub fn new() -> Self {
        Self { on: false }
    }

    pub fn turn_on(&mut self) {
        self.on = true;
        println!("Lighting system turned on.");
    }

    pub fn turn_off(&mut self) {
        self.on = false;
        println!("Lighting system turned off.");
    }
}

pub struct SmartHomeFacade {
    thermostat: Thermostat,
    pub security_system: SecuritySystem,
    lighting_system: LightingSystem,
}

impl SmartHomeFacade {
    pub fn new() -> Self {
        Self {
            thermostat: Thermostat::new(),
            security_system: SecuritySystem::new(),
            lighting_system: LightingSystem::new(),
        }
    }

    pub fn going_out(&mut self, temperature: f32) {
        self.thermostat.set_temperature(temperature);
        self.security_system.lock();
        self.lighting_system.turn_off();
    }

    pub fn going_in(&mut self, temperature: f32) {
        self.thermostat.set_temperature(temperature);
        self.security_system.unlock();
        self.lighting_system.turn_on();
    }
}
