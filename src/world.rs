use instant::{Duration, Instant};

use crate::{
    inertia::Inertia,
    render::{render_target::RenderTarget, scene::Scene},
    ship::{Land, Ship, Throttle},
};

struct IntegrationController {
    clock: Instant,
}
impl IntegrationController {
    fn new() -> IntegrationController {
        IntegrationController {
            clock: Instant::now(),
        }
    }

    fn step() -> Duration {
        Duration::from_millis((Inertia::step() * 1000.0) as u64)
    }

    fn integrate(&mut self, ship: &mut Ship, land: &mut Land) -> bool {
        let now = Instant::now();
        while self.clock < now {
            ship.integrate(land);
            self.clock += Self::step();
        }
        true
    }
}

pub struct World<T>
where
    T: RenderTarget,
{
    target: T,
    scene: T::RenderScene<Scene>,
    ship: Ship,
    land: Land,
    integration: IntegrationController,
    prev_zoom: f32,
}
impl<T> From<T> for World<T>
where
    T: RenderTarget,
{
    fn from(mut target: T) -> Self {
        let scene = target.new_scene(Scene::new());

        World {
            target,
            scene,
            ship: Ship::new(),
            land: Land::new(),
            integration: IntegrationController::new(),
            prev_zoom: 0.0002,
        }
    }
}
#[cfg(feature = "wgpu_render")]
impl<T: raw_window_handle::HasRawWindowHandle> From<&T>
    for World<crate::render::wgpu::target::WgpuRenderTarget>
{
    fn from(window: &T) -> Self {
        crate::render::wgpu::target::WgpuRenderTarget::new(window).into()
    }
}
impl<T> World<T>
where
    T: RenderTarget,
{
    pub fn resize(&mut self, width: u32, height: u32) {
        self.target.resize(width, height);
    }

    pub fn redraw(&mut self) {
        let origin = self.ship.origin();
        let ground = self.land.get(origin);
        let ground = ground.0 + ground.direction() * (origin - ground.0).0;
        let distance = (origin - ground).len() + 30.0;
        let zoom = (1.0 / distance).min(0.02);
        let zoom = zoom * 0.01 + self.prev_zoom * 0.99;
        self.prev_zoom = zoom;

        let size = self.target.get_size();

        self.integration.integrate(&mut self.ship, &mut self.land);
        self.scene.set_throttles(&self.ship.active_throttles());
        self.scene
            .set_position(self.ship.origin(), self.ship.direction());
        self.scene.set_zoom(zoom);
        self.scene.set_window_size(size.0, size.1);
        self.scene.set_land(self.land.all());
        self.target.render_one(&mut self.scene, &());
    }

    pub fn control(&mut self, throttle: Throttle, activate: bool) {
        self.ship.throttle(throttle, activate);
    }
}
