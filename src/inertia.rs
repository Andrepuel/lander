use crate::geom::{Point, Vector};

pub struct Inertia {
    pub position: Point,
    pub prev: Point,
}
impl Inertia {
    pub fn step() -> f32 {
        0.1
    }

    pub fn new(position: Point) -> Inertia {
        let prev = position;
        Inertia { position, prev }
    }

    pub fn integrate(&mut self) {
        let inertia = self.inertia();
        self.prev = self.position;
        self.position = self.position + inertia
    }

    pub fn inertia(&self) -> Point {
        self.position - self.prev
    }

    pub fn force(&mut self, force: Vector) {
        self.prev = self.prev - (force * Self::step());
    }
}
impl std::fmt::Debug for Inertia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}->{:?}", self.position, self.inertia())
    }
}
