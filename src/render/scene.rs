use rand::prelude::Distribution;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

use crate::geom::{Line, Mat3, Point, Vector};

use super::render_target::RenderScene;

pub struct Scene {
    program: WebGlProgram,
    uniform: WebGlUniformLocation,
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

    fn triangle_uniform(&self, transform: Mat3, context: &WebGl2RenderingContext) {
        let transform = self.camera * transform;
        let transform = Mat3::scale_y(-1.0) * transform;
        let data = transform.as_f32_packed();
        context.uniform_matrix3fv_with_f32_array(Some(&self.uniform), false, &data);
    }

    fn ship_uniform<'a>(
        &'a self,
        context: &'a WebGl2RenderingContext,
    ) -> impl Iterator<Item = ()> + 'a {
        let transform = self.position * Mat3::scale(3.0, 10.0);

        (0..1)
            .into_iter()
            .map(move |_| self.triangle_uniform(transform, context))
    }

    fn throttles_uniform<'a>(
        &'a self,
        context: &'a WebGl2RenderingContext,
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
        context: &'a WebGl2RenderingContext,
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
        context: &WebGl2RenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
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
        context: &WebGl2RenderingContext,
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
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
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
    fn new_scene(context: &WebGl2RenderingContext) -> Scene {
        let vert_shader = Self::compile_shader(
            context,
            WebGl2RenderingContext::VERTEX_SHADER,
            include_str!("shader.vert"),
        )
        .unwrap();
        let frag_shader = Self::compile_shader(
            context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            include_str!("shader.frag"),
        )
        .unwrap();
        let program = Self::link_program(context, &vert_shader, &frag_shader).unwrap();
        let uniform = context.get_uniform_location(&program, "matrix").unwrap();

        Scene {
            program,
            uniform,
            camera: Mat3::identity(),
            position: Mat3::identity(),
            throttles: vec![-1],
            land: vec![],
        }
    }

    fn render_one(&mut self, context: &WebGl2RenderingContext) {
        context.use_program(Some(&self.program));
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        for _ in self
            .ship_uniform(context)
            .chain(self.throttles_uniform(context))
            .chain(self.ground_uniform(context))
        {
            context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);
        }
    }
}
