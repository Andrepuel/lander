use std::time::{Duration, Instant};

use lander::{
    inertia::Inertia,
    render::{render_target::RenderTarget, scene::Scene},
    ship::{Land, Ship, Throttle},
};
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, Event, KeyboardInput, WindowEvent},
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

    fn integrate(&mut self, ship: &mut Ship, land: &mut Land) -> bool {
        let now = Instant::now();
        while self.clock < now {
            ship.integrate(land);
            self.clock += Self::step();
        }
        true
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
    let mut land = Land::new();
    let mut integration = IntegrationController::new();
    let mut window_size = PhysicalSize {
        width: 1,
        height: 1,
    };
    let mut prev_zoom = 0.0002;

    event_loop.run(move |event, _loop_target, control_flow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(33));
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                window_size = size;
                target.resize(size.width, size.height);
            }
            Event::RedrawRequested(_) => {
                let origin = ship.origin();
                let ground = land.get(origin.0);
                let ground = ground.0 + ground.direction() * (origin - ground.0).0;
                let distance = (origin - ground).len() + 30.0;
                let zoom = (1.0 / distance).min(0.02);
                let zoom = zoom * 0.01 + prev_zoom * 0.99;
                prev_zoom = zoom;

                integration.integrate(&mut ship, &mut land);
                scene.set_throttles(&ship.active_throttles());
                scene.set_position(ship.origin(), ship.direction());
                scene.set_zoom(zoom);
                scene.set_window_size(window_size.width, window_size.height);
                scene.set_land(land.all());
                target.render_one(&mut scene);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::DeviceEvent {
                event:
                    DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(keycode),
                        state,
                        ..
                    }),
                ..
            } => {
                let activate = match state {
                    winit::event::ElementState::Pressed => true,
                    winit::event::ElementState::Released => false,
                };
                let throttle = match keycode {
                    winit::event::VirtualKeyCode::Left => Some(Throttle::Left),
                    winit::event::VirtualKeyCode::Up => Some(Throttle::Bottom),
                    winit::event::VirtualKeyCode::Right => Some(Throttle::Right),
                    _ => None,
                };
                if let Some(throttle) = throttle {
                    ship.throttle(throttle, activate);
                }
            }
            Event::NewEvents(_) => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
