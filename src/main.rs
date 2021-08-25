use std::time::{Duration, Instant};

use lander::{
    inertia::Inertia,
    render::{render_target::RenderTarget, scene::Scene},
    ship::Ship,
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
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

    fn integrate(&mut self, ship: &mut Ship) {
        while self.clock < Instant::now() {
            ship.integrate();
            self.clock += Self::step();
        }
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
        .try_init()
        .unwrap_or_else(|err| println!("env_logger::init() failed: {}", err));

    let event_loop = EventLoop::new();

    let window = winit::window::WindowBuilder::new()
        .with_title("Lander")
        .with_inner_size(PhysicalSize::new(800, 800))
        .build(&event_loop)
        .unwrap();

    let mut target = RenderTarget::new(&window);
    let mut scene: Scene = target.new_scene();
    let mut ship = Ship::new();
    let mut integration = IntegrationController::new();

    event_loop.run(move |event, _loop_target, control_flow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(33));
        window.request_redraw();
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                target.resize(size.width, size.height);
            }
            Event::RedrawRequested(_) => {
                integration.integrate(&mut ship);
                scene.set_position(ship.origin(), ship.direction());
                target.render_one(&mut scene);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
