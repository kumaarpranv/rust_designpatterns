pub trait Department {
    fn set_next(&mut self, next: Option<Box<dyn Department>>);
    fn handle_request(&self, request: &str);
}

pub struct Reception {
    next: Option<Box<dyn Department>>,
}

impl Reception {
    pub fn new() -> Self {
        Reception { next: None }
    }
}

impl Department for Reception {
    fn set_next(&mut self, next: Option<Box<dyn Department>>) {
        self.next = next;
    }

    fn handle_request(&self, request: &str) {
        println!("Reception: Checking in patient...");
        if let Some(ref next) = self.next {
            next.handle_request(request);
        }
    }
}

pub struct Doctor {
    next: Option<Box<dyn Department>>,
}

impl Doctor {
    pub fn new() -> Self {
        Doctor { next: None }
    }
}

impl Department for Doctor {
    fn set_next(&mut self, next: Option<Box<dyn Department>>) {
        self.next = next;
    }

    fn handle_request(&self, request: &str) {
        println!("Doctor: Diagnosing and treating patient...");
        if let Some(ref next) = self.next {
            next.handle_request(request);
        }
    }
}

pub struct Cashier {
    next: Option<Box<dyn Department>>,
}

impl Cashier {
    pub fn new() -> Self {
        Cashier { next: None }
    }
}

impl Department for Cashier {
    fn set_next(&mut self, next: Option<Box<dyn Department>>) {
        self.next = next;
    }

    fn handle_request(&self, request: &str) {
        println!("Cashier: Processing payment...");
        if let Some(ref next) = self.next {
            next.handle_request(request);
        }
    }
}
