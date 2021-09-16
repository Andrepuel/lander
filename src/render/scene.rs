use crate::geom::{Line, Mat3, Point, Vector};
use rand::prelude::Distribution;

use super::render_target::RenderScene;

pub struct Scene {
    camera: Mat3,
    position: Mat3,
    throttles: Vec<i32>,
    land: Vec<Line>,
}
impl Scene {
    pub fn new() -> Scene {
        Scene {
            camera: Mat3::identity(),
            position: Mat3::identity(),
            throttles: vec![-1],
            land: vec![],
        }
    }

    pub fn set_position(&mut self, bottom: Point, direction: Vector) {
        self.position = Mat3::translate(bottom.0, bottom.1) * Mat3::rotate_y_to(direction);
        self.camera = Mat3::translate(-bottom.0, -bottom.1);
    }

    pub fn set_zoom(&mut self, scale: f32) {
        self.camera = Mat3::scale(scale, scale) * self.camera;
    }

    pub fn set_window_size(&mut self, width: u32, height: u32) {
        let aspect = (height as f32) / (width as f32);
        self.camera = Mat3::scale(aspect, 1.0) * self.camera;
    }

    pub fn set_throttles(&mut self, throttles: &[i32]) {
        self.throttles = throttles.to_owned();
    }

    pub fn set_land<L: Iterator<Item = Line>>(&mut self, land: L) {
        self.land = land.map(|x| x).collect();
    }

    fn ship(&self) -> Mat3 {
        let transform = self.position * Mat3::scale(3.0, 10.0);
        transform
    }

    fn throttles<'a>(&'a self) -> impl Iterator<Item = Mat3> + 'a {
        let mut rng = rand::thread_rng();
        let between = rand::distributions::Uniform::from(100..300);

        self.throttles.iter().map(move |pos| {
            let throttle_size = (between.sample(&mut rng) as f32) / 100.0;
            let transform = self.position
                * Mat3::translate((*pos as f32) * 3.0, 0.0)
                * Mat3::scale(0.5, -throttle_size);

            transform
        })
    }

    fn ground<'a>(&'a self) -> impl Iterator<Item = Mat3> + 'a {
        self.land.iter().map(move |line| {
            let pos = line.center();
            let direction = line.direction().rot90() * -1.0;

            let transform = Mat3::translate(pos.0, pos.1)
                * Mat3::rotate_y_to(direction)
                * Mat3::scale(line.len() * 0.52, -1.0);

            transform
        })
    }
}
impl RenderScene for Scene {
    type Context<'a> = &'a ();

    fn triangles(&self, _context: Self::Context<'_>) -> Box<dyn Iterator<Item = Mat3> + '_> {
        let r = (0..1)
            .map(move |_| self.ship())
            .chain(self.ground())
            .chain(self.throttles())
            .map(move |x| self.camera * x);

        Box::new(r)
    }
}
