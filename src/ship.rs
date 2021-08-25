use std::collections::HashSet;

use crate::{
    geom::{Point, Vector},
    inertia::Inertia,
};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Throttle {
    Left,
    Bottom,
    Right,
}

#[derive(Debug)]
pub struct Ship {
    bottom: (Inertia, Inertia),
    top: Inertia,
    throttle: HashSet<Throttle>,
}
impl Ship {
    pub fn new() -> Ship {
        Ship {
            bottom: (
                Inertia::new(Point(-3.0, 0.0)),
                Inertia::new(Point(3.0, 0.0)),
            ),
            top: Inertia::new(Point(0.0, 10.0)),
            throttle: Default::default(),
        }
    }

    pub fn origin(&self) -> Point {
        (self.bottom.0.position + self.bottom.1.position) * 0.5
    }

    pub fn direction(&self) -> Vector {
        let origin = self.origin();
        let dir1 = (self.top.position - origin).unit();
        let dir2 = (self.bottom.0.position - self.bottom.1.position)
            .unit()
            .rot90();
        (dir1 + dir2) * 0.5
    }

    pub fn integrate(&mut self) {
        self.bottom.0.force(Self::gravity());
        self.bottom.1.force(Self::gravity());
        self.top.force(Self::gravity());

        if self.throttle.contains(&Throttle::Left) {
            self.bottom.0.force(self.throttle_force());
        }
        if self.throttle.contains(&Throttle::Bottom) {
            self.bottom.0.force(self.throttle_force());
            self.bottom.1.force(self.throttle_force());
        }
        if self.throttle.contains(&Throttle::Right) {
            self.bottom.1.force(self.throttle_force());
        }

        self.bottom.0.integrate();
        self.bottom.1.integrate();
        self.top.integrate();
        self.fix_points_equidistance();
    }

    fn gravity() -> Vector {
        Point(0.0, -0.32)
    }

    fn throttle_force(&self) -> Vector {
        let force = Self::gravity().len() * 3.0;
        self.direction() * force
    }

    fn fix_points_equidistance(&mut self) {
        let bottom = (self.bottom.0.position + self.bottom.1.position) * 0.5;
        let center = (bottom + self.top.position) * 0.5;
        let direction = self.direction();

        self.top.position = center + (direction * 5.0);
        let bottom = center - (direction * 5.0);
        let direction = direction.rot90();
        self.bottom.0.position = bottom - direction * 3.0;
        self.bottom.1.position = bottom + direction * 3.0;
    }

    pub fn throttle(&mut self, throttle: Throttle, activate: bool) {
        if activate {
            self.throttle.insert(throttle);
        } else {
            self.throttle.remove(&throttle);
        }
    }

    pub fn active_throttles(&self) -> Vec<i32> {
        [
            (Throttle::Left, -1),
            (Throttle::Bottom, 0),
            (Throttle::Right, 1),
        ]
        .iter()
        .filter(|(x, _)| self.throttle.contains(x))
        .map(|(_, pos)| *pos)
        .collect()
    }
}
