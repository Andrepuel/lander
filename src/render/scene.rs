use std::borrow::Cow;

use rand::prelude::Distribution;
use wgpu::util::DeviceExt;

use crate::geom::{Line, Mat3, Point, Vector};

use super::render_target::RenderScene;

pub struct Scene {
    render_pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    camera: Mat3,
    position: Mat3,
    throttles: Vec<i32>,
    land: Vec<Line>,
}
impl Scene {
    pub fn set_position(&mut self, bottom: Point, direction: Vector) {
        self.position = Mat3::translate(bottom.0, bottom.1) * Mat3::rotate_y_to(direction);
        self.camera = Mat3::scale(0.02, 0.02) * Mat3::translate(-bottom.0, -bottom.1);
    }

    pub fn set_throttles(&mut self, throttles: &[i32]) {
        self.throttles = throttles.to_owned();
    }

    pub fn set_land<L: Iterator<Item = Line>>(&mut self, land: L) {
        self.land = land.map(|x| x).collect();
    }

    fn triangle_bind_group(&self, device: &wgpu::Device, transform: Mat3) -> wgpu::BindGroup {
        let transform = self.camera * transform;

        let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: to_u8(&transform.as_f32()),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buf.as_entire_binding(),
            }],
            label: None,
        });

        bind_group
    }

    fn ship_bind_group(&self, device: &wgpu::Device) -> wgpu::BindGroup {
        let transform = self.position * Mat3::scale(3.0, 10.0);

        self.triangle_bind_group(device, transform)
    }

    fn throttles_bind_group<'a>(
        &'a self,
        device: &'a wgpu::Device,
    ) -> impl Iterator<Item = wgpu::BindGroup> + 'a {
        let mut rng = rand::thread_rng();
        let between = rand::distributions::Uniform::from(100..300);

        self.throttles.iter().map(move |pos| {
            let throttle_size = (between.sample(&mut rng) as f32) / 100.0;
            let transform = self.position
                * Mat3::translate((*pos as f32) * 3.0, 0.0)
                * Mat3::scale(0.5, -throttle_size);

            self.triangle_bind_group(device, transform)
        })
    }

    fn ground_bind_groups<'a>(
        &'a self,
        device: &'a wgpu::Device,
    ) -> impl Iterator<Item = wgpu::BindGroup> + 'a {
        self.land.iter().map(move |line| {
            let pos = line.center();
            let direction = line.direction().rot90() * -1.0;

            let transform = Mat3::translate(pos.0, pos.1)
                * Mat3::rotate_y_to(direction)
                * Mat3::scale(line.len() * 0.52, -1.0);

            self.triangle_bind_group(device, transform)
        })
    }
}
impl RenderScene for Scene {
    fn new_scene(
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
        target_format: wgpu::TextureFormat,
    ) -> Scene {
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
            flags: wgpu::ShaderFlags::all(),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(4 * 12),
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[target_format.into()],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                polygon_mode: wgpu::PolygonMode::Fill,
                ..wgpu::PrimitiveState::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
        });

        Scene {
            render_pipeline,
            bind_group_layout,
            camera: Mat3::identity(),
            position: Mat3::identity(),
            throttles: vec![-1],
            land: vec![],
        }
    }

    fn render_one(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, view: &wgpu::TextureView) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut triangles = vec![self.ship_bind_group(device)];
            triangles.extend(self.throttles_bind_group(device));
            triangles.extend(self.ground_bind_groups(device));

            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            rpass.set_pipeline(&self.render_pipeline);

            triangles.iter().for_each(|bind| {
                rpass.set_bind_group(0, &bind, &[]);
                rpass.draw(0..3, 0..1);
            });
        }

        queue.submit(Some(encoder.finish()));
    }
}

fn to_u8<T: Copy>(a: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(a as *const T as *const u8, std::mem::size_of::<T>()) }
}
