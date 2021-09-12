use rand::prelude::Distribution;
use web_sys::{
    WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlUniformLocation,
};

use crate::geom::{Line, Mat3, Point, Vector};

use super::render_target::RenderScene;

pub struct Scene {
    program: WebGlProgram,
    uniform: WebGlUniformLocation,
    vertex_buffer: WebGlBuffer,
    vertex_location: i32,
    camera: Mat3,
    position: Mat3,
    throttles: Vec<i32>,
    land: Vec<Line>,
}
impl Scene {
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

    fn triangle_uniform(&self, transform: Mat3, context: &WebGlRenderingContext) {
        let transform = self.camera * transform;
        let transform = Mat3::scale_y(-1.0) * transform;
        let data = transform.as_f32_packed();
        context.uniform_matrix3fv_with_f32_array(Some(&self.uniform), false, &data);
    }

    fn ship_uniform<'a>(
        &'a self,
        context: &'a WebGlRenderingContext,
    ) -> impl Iterator<Item = ()> + 'a {
        let transform = self.position * Mat3::scale(3.0, 10.0);

        (0..1)
            .into_iter()
            .map(move |_| self.triangle_uniform(transform, context))
    }

    fn throttles_uniform<'a>(
        &'a self,
        context: &'a WebGlRenderingContext,
    ) -> impl Iterator<Item = ()> + 'a {
        let mut rng = rand::thread_rng();
        let between = rand::distributions::Uniform::from(100..300);

        self.throttles.iter().map(move |pos| {
            let throttle_size = (between.sample(&mut rng) as f32) / 100.0;
            let transform = self.position
                * Mat3::translate((*pos as f32) * 3.0, 0.0)
                * Mat3::scale(0.5, -throttle_size);

            self.triangle_uniform(transform, context)
        })
    }

    fn ground_uniform<'a>(
        &'a self,
        context: &'a WebGlRenderingContext,
    ) -> impl Iterator<Item = ()> + 'a {
        self.land.iter().map(move |line| {
            let pos = line.center();
            let direction = line.direction().rot90() * -1.0;

            let transform = Mat3::translate(pos.0, pos.1)
                * Mat3::rotate_y_to(direction)
                * Mat3::scale(line.len() * 0.52, -1.0);

            self.triangle_uniform(transform, context)
        })
    }

    pub fn compile_shader(
        context: &WebGlRenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    pub fn link_program(
        context: &WebGlRenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        context.attach_shader(&program, vert_shader);
        context.attach_shader(&program, frag_shader);
        context.link_program(&program);

        if context
            .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }
}
impl RenderScene for Scene {
    fn new_scene(context: &WebGlRenderingContext) -> Scene {
        let vert_shader = Self::compile_shader(
            context,
            WebGlRenderingContext::VERTEX_SHADER,
            include_str!("shader.vert"),
        )
        .unwrap();
        let frag_shader = Self::compile_shader(
            context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            include_str!("shader.frag"),
        )
        .unwrap();
        let program = Self::link_program(context, &vert_shader, &frag_shader).unwrap();
        let uniform = context.get_uniform_location(&program, "matrix").unwrap();

        let vertex_location = context.get_attrib_location(&program, "a_position");
        let vertex_buffer = context
            .create_buffer()
            .ok_or("Failed to create buffer")
            .unwrap();
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

        unsafe {
            let vertices: [f32; 6] = [-1.0, 0.0, 0.0, 1.0, 1.0, 0.0];
            let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

            context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);

        Scene {
            program,
            uniform,
            vertex_buffer,
            vertex_location,
            camera: Mat3::identity(),
            position: Mat3::identity(),
            throttles: vec![-1],
            land: vec![],
        }
    }

    fn render_one(&mut self, context: &WebGlRenderingContext) {
        context.use_program(Some(&self.program));
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        context.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.vertex_buffer),
        );
        context.vertex_attrib_pointer_with_i32(
            self.vertex_location as u32,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        context.enable_vertex_attrib_array(self.vertex_location as u32);

        for _ in self
            .ship_uniform(context)
            .chain(self.throttles_uniform(context))
            .chain(self.ground_uniform(context))
        {
            context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 3);
        }
    }
}
