use std::{array, collections::HashSet};
use wasm_bindgen::prelude::wasm_bindgen;

use rand::prelude::Distribution;

use crate::{
    geom::{Line, Mat3, Point, Vector},
    inertia::Inertia,
    render::scene::Drawable,
};

#[wasm_bindgen]
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

    pub fn drawable(&self) -> impl Drawable + '_ {
        ShipDrawable(self, Some(Mat3::scale(3.0, 10.0)).into_iter())
    }

    pub fn active_throttles(&self) -> impl Drawable + '_ {
        let triangles = [
            self.throttle_drawing(Throttle::Left),
            self.throttle_drawing(Throttle::Bottom),
            self.throttle_drawing(Throttle::Right),
        ];

        ShipDrawable(self, array::IntoIter::new(triangles))
    }

    fn throttle_drawing(&self, thruster: Throttle) -> Mat3 {
        let size = if self.throttle.contains(&thruster) {
            let mut rng = rand::thread_rng();
            let between = rand::distributions::Uniform::from(100..300);
            (between.sample(&mut rng) as f32) / 100.0
        } else {
            0.0
        };

        let pos = (thruster as i32) - 1;

        Mat3::translate((pos as f32) * 3.0, 0.0) * Mat3::scale(0.5, -size)
    }
}

pub struct ShipDrawable<'a, T: Iterator<Item = Mat3>>(&'a Ship, T);
impl<'a, T: Iterator<Item = Mat3>> Drawable for ShipDrawable<'a, T> {
    fn position(&self) -> crate::geom::Mat3 {
        let origin = self.0.origin();

        Mat3::translate(origin.0, origin.1) * Mat3::rotate_y_to(self.0.direction())
    }

    fn triangles<'b>(&'b mut self) -> &'b mut (dyn Iterator<Item = Mat3> + 'b) {
        &mut self.1
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

    pub fn apply_gravity(&mut self, point: &mut Inertia) {
        point.force(Ship::gravity());
    }

    pub fn handle_collision(&mut self, pos: &mut Point) {
        let ground_line = self.get(*pos);
        let ground_hit = ground_line.projection(*pos);
        if pos.1 < ground_hit.1 {
            *pos = ground_hit;
        }
    }

    pub fn get(&mut self, pos: Point) -> Line {
        let x = pos.0;
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

    pub fn drawable(&self) -> impl Iterator<Item = Mat3> + '_ {
        self.all().map(|line| {
            let pos = line.center();
            let direction = line.direction().rot90() * -1.0;

            let transform = Mat3::translate(pos.0, pos.1)
                * Mat3::rotate_y_to(direction)
                * Mat3::scale(line.len() * 0.52, -1.0);

            transform
        })
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
