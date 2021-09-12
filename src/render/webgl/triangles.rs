use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

use crate::{geom::Mat3, render::triangles};

use super::target::RenderScene;

pub struct TriangleScene {
    program: WebGlProgram,
    uniform: WebGlUniformLocation,
}
impl TriangleScene {
    fn new_scene(context: &WebGl2RenderingContext) -> TriangleScene {
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

        TriangleScene { program, uniform }
    }

    fn triangle_uniform(&self, transform: Mat3, context: &WebGl2RenderingContext) {
        let transform = Mat3::scale_y(-1.0) * transform;
        let data = transform.as_f32_packed();
        context.uniform_matrix3fv_with_f32_array(Some(&self.uniform), false, &data);
    }

    fn render_one<'a>(
        &self,
        triangles: Box<dyn Iterator<Item = Mat3> + 'a>,
        context: &WebGl2RenderingContext,
    ) {
        context.use_program(Some(&self.program));
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        for triangle in triangles {
            self.triangle_uniform(triangle, context);
            context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);
        }
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
impl<T: triangles::TriangleScene<TriangleScene>> RenderScene for T {
    type Context = T::Context;

    fn new_scene(context: &WebGl2RenderingContext) -> Self {
        <Self as triangles::TriangleScene<TriangleScene>>::new_scene(TriangleScene::new_scene(
            context,
        ))
    }

    fn render_one(&mut self, scene_context: &Self::Context, context: &WebGl2RenderingContext) {
        let triangles = self.triangles(scene_context);
        self.attr_pipeline().render_one(triangles, context);
    }
}
