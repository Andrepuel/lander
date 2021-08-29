use std::collections::HashSet;

use rand::prelude::Distribution;

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
            let ground_line = land.get(point.position.0);
            let ground_hit = ground_line.projection(point.position);
            if point.position.1 < ground_hit.1 {
                point.position = ground_hit;
            }
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
    heights: Vec<Point>,
}
impl Land {
    pub fn new() -> Land {
        Land {
            heights: vec![Point(-15.0, -30.0), Point(15.0, -30.0)],
        }
    }

    pub fn get(&mut self, x: f32) -> Line {
        self.expand_min(x - 500.0);
        self.expand_max(x + 500.0);
        self.shrink_min(x - 800.0);
        self.shrink_max(x + 800.0);

        let points = Self::binary_search(&self.heights, x);
        Line(points[0], points[1])
    }

    pub fn all(&self) -> impl Iterator<Item = Line> + '_ {
        (1..self.heights.len()).map(move |idx| Line(self.heights[idx - 1], self.heights[idx]))
    }

    fn expand_min(&mut self, min: f32) {
        while self.heights[0].0 > min {
            let mut new_height = Self::new_height();
            new_height.0 = self.heights[0].0 - new_height.0;
            new_height.1 = self.heights[0].1 + new_height.1;
            self.heights.insert(0, new_height);
        }
    }

    fn expand_max(&mut self, max: f32) {
        while self.heights.last().unwrap().0 < max {
            let mut new_height = Self::new_height();
            new_height.0 = self.heights.last().unwrap().0 + new_height.0;
            new_height.1 = self.heights.last().unwrap().1 + new_height.1;
            self.heights.push(new_height);
        }
    }

    fn shrink_min(&mut self, min: f32) {
        while self.heights[0].0 < min {
            self.heights.remove(0);
        }
    }

    fn shrink_max(&mut self, max: f32) {
        while self.heights.last().unwrap().0 > max {
            self.heights.pop();
        }
    }

    fn new_height() -> Point {
        let mut rng = rand::thread_rng();
        let x_between = rand::distributions::Uniform::from(1000..3000);
        let y_between = rand::distributions::Uniform::from(-3000..3000);

        Point(
            (x_between.sample(&mut rng) as f32) / 100.0,
            (y_between.sample(&mut rng) as f32) / 100.0,
        )
    }

    fn binary_search(heights: &[Point], x: f32) -> &[Point] {
        if heights.len() == 2 {
            return heights;
        }
        assert!(heights.len() > 2);
        let middle_idx = heights.len() / 2;
        let middle = heights[middle_idx].0;
        if middle < x {
            Self::binary_search(&heights[middle_idx..], x)
        } else {
            Self::binary_search(&heights[0..(middle_idx + 1)], x)
        }
    }
}
