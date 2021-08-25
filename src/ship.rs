use crate::{
    geom::{Point, Vector},
    inertia::Inertia,
};

#[derive(Debug)]
pub struct Ship {
    bottom: (Inertia, Inertia),
    top: Inertia,
}
impl Ship {
    pub fn new() -> Ship {
        let bottom = (
            Inertia::new(Point(-1.0, 0.0)),
            Inertia::new(Point(1.0, 0.0)),
        );
        let top = Inertia::new(Point(0.0, 10.0));
        Ship { bottom, top }
    }

    pub fn direction(&self) -> Vector {
        let origin = (self.bottom.0.position + self.bottom.1.position) * 0.5;
        let dir1 = (self.top.position - origin).unit();
        let dir2 = (self.bottom.0.position - self.bottom.1.position)
            .unit()
            .rot90();
        (dir1 + dir2) * 0.5
    }

    pub fn integrate(&mut self) {
        self.bottom.0.force(Self::gravity());
        // self.top.force(Self::gravity());
        self.bottom.0.integrate();
        self.bottom.1.integrate();
        self.top.integrate();
        self.fix_points_equidistance();
    }

    fn gravity() -> Vector {
        Point(0.0, -1.62)
    }

    fn fix_points_equidistance(&mut self) {
        let bottom = (self.bottom.0.position + self.bottom.1.position) * 0.5;
        let center = (bottom + self.top.position) * 0.5;
        let direction = self.direction();

        self.top.position = center + (direction * 5.0);
        let bottom = center - (direction * 5.0);
        let direction = direction.rot90();
        self.bottom.0.position = bottom - direction;
        self.bottom.1.position = bottom + direction;
    }
}
