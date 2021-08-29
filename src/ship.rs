use std::collections::HashSet;

use crate::{
    geom::{Line, Point, Vector},
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

    pub fn integrate(&mut self, land: &mut Land) {
        self.all_points().for_each(|point| {
            land.apply_gravity(point);
        });

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
        self.ground_collision(land);
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

    fn all_points(&mut self) -> impl Iterator<Item = &mut Inertia> {
        vec![&mut self.bottom.0, &mut self.bottom.1, &mut self.top].into_iter()
    }

    fn ground_collision(&mut self, land: &mut Land) {
        self.all_points().for_each(|point| {
            land.handle_collision(&mut point.position);
        });
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

pub struct Land {
    center: Point,
    radius: f32,
}
impl Land {
    pub fn new() -> Land {
        Land {
            center: Point(0.0, -1000.0),
            radius: 1000.0,
        }
    }

    pub fn apply_gravity(&mut self, point: &mut Inertia) {
        let force = Ship::gravity().len();
        let direction = (self.center - point.position).unit();
        point.force(direction * force);
    }

    pub fn handle_collision(&mut self, pos: &mut Point) {
        let direction = *pos - self.center;
        let distance = direction.len();
        if distance < self.radius {
            *pos = self.center + direction.unit() * self.radius;
        }
    }

    pub fn get(&mut self, pos: Point) -> Line {
        let direction = pos - self.center;
        let intersect = self.center + direction.unit() * self.radius;

        Line(intersect, intersect + Point(1.0, 1.0))
    }

    pub fn all(&self) -> impl Iterator<Item = Line> + '_ {
        (0..361)
            .map(move |angle| {
                let angle = (angle as f32) / 180.0 * std::f32::consts::PI;
                Point(angle.cos(), angle.sin()) * self.radius + self.center
            })
            .two_items()
            .map(|x| Line(x.0, x.1))
    }
}

struct TwoItems<T: Iterator>(T, Option<T::Item>, Option<T::Item>)
where
    T::Item: Copy;
impl<T: Iterator> TwoItems<T>
where
    T::Item: Copy,
{
    fn new(it: T) -> TwoItems<T> {
        TwoItems(it, None, None)
    }
}
impl<T: Iterator> Iterator for TwoItems<T>
where
    T::Item: Copy,
{
    type Item = (T::Item, T::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let under = self.0.next();
        self.2 = self.1;
        self.1 = under;
        match self.1 {
            Some(a) => match self.2 {
                Some(b) => Some((a, b)),
                None => self.next(),
            },
            None => None,
        }
    }
}

impl<T: Iterator> TwoItemsEx for T where T::Item: Copy {}
trait TwoItemsEx: Iterator + Sized
where
    Self::Item: Copy,
{
    fn two_items(self) -> TwoItems<Self> {
        TwoItems::new(self)
    }
}
