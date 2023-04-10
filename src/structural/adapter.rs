pub struct ConcreteRoundHole {
    radius: f64,
}

impl ConcreteRoundHole {
    pub fn new(radius: f64) -> Self {
        ConcreteRoundHole { radius }
    }

    pub fn fits<T: RoundPeg>(&self, peg: &T) -> bool {
        self.radius >= peg.get_radius()
    }
}

pub trait RoundPeg {
    fn get_radius(&self) -> f64;
}

pub trait SquarePeg {
    fn get_width(&self) -> f64;
}

pub struct ConcreteRoundPeg {
    radius: f64,
}

impl RoundPeg for ConcreteRoundPeg {
    fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl ConcreteRoundPeg {
    pub fn new(radius: f64) -> ConcreteRoundPeg {
        ConcreteRoundPeg { radius: radius }
    }
}

pub struct ConcreteSquarePeg {
    width: f64,
}

impl SquarePeg for ConcreteSquarePeg {
    fn get_width(&self) -> f64 {
        self.width
    }
}

impl ConcreteSquarePeg {
    pub fn new(width: f64) -> ConcreteSquarePeg {
        ConcreteSquarePeg { width: width }
    }
}

pub struct SquarePegAdapter {
    square_peg: ConcreteSquarePeg,
}

impl RoundPeg for SquarePegAdapter {
    fn get_radius(&self) -> f64 {
        self.square_peg.get_width() * (2.0 as f64).sqrt() / 2.0
    }
}

impl SquarePegAdapter {
    pub fn new(square_peg: ConcreteSquarePeg) -> Self {
        SquarePegAdapter { square_peg }
    }
}
