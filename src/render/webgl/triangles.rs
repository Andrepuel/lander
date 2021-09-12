use web_sys::{
    WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlUniformLocation,
};

use crate::{geom::Mat3, render::triangles};

use super::target::RenderScene;

pub struct TriangleScene {
    program: WebGlProgram,
    uniform: WebGlUniformLocation,
    vertex_buffer: WebGlBuffer,
    vertex_location: i32,
}
impl TriangleScene {
    fn new_scene(context: &WebGlRenderingContext) -> TriangleScene {
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

        TriangleScene {
            program,
            uniform,
            vertex_buffer,
            vertex_location,
        }
    }

    fn triangle_uniform(&self, transform: Mat3, context: &WebGlRenderingContext) {
        let transform = Mat3::scale_y(-1.0) * transform;
        let data = transform.as_f32_packed();
        context.uniform_matrix3fv_with_f32_array(Some(&self.uniform), false, &data);
    }

    fn render_one<'a>(
        &self,
        triangles: Box<dyn Iterator<Item = Mat3> + 'a>,
        context: &WebGlRenderingContext,
    ) {
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

        for triangle in triangles {
            self.triangle_uniform(triangle, context);
            context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 3);
        }
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
impl<T: triangles::TriangleScene<TriangleScene>> RenderScene for T {
    type Context = T::Context;

    fn new_scene(context: &WebGlRenderingContext) -> Self {
        <Self as triangles::TriangleScene<TriangleScene>>::new_scene(TriangleScene::new_scene(
            context,
        ))
    }

    fn render_one(&mut self, scene_context: &Self::Context, context: &WebGlRenderingContext) {
        let triangles = self.triangles(scene_context);
        self.attr_pipeline().render_one(triangles, context);
    }
}
