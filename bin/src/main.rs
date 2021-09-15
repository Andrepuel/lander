use std::time::{Duration, Instant};

use lander::{ship::Throttle, world::World};
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

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

    let mut world = World::from(&window);

    event_loop.run(move |event, _loop_target, control_flow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(33));
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                world.resize(size.width, size.height);
            }
            Event::RedrawRequested(_) => {
                world.redraw();
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
                    world.control(throttle, activate);
                }
            }
            Event::NewEvents(_) => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
