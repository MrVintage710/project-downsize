use glow::*;
use crate::render::Renderable;
use crate::render::shader::ShaderProgram;
use crate::render::texture::Texture;

pub struct RenderContext {
    shader_program : Option<ShaderProgram>,
    textures : [Option<Texture>; 16]
}

impl RenderContext {

    pub fn new() -> Self {
        RenderContext {shader_program : None,
            textures : [None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None]}
    }

    pub fn with_shader_program(mut self, gl : &Context, vertex_shader: &str, fragment_shader : &str) -> Result<Self, String> {
        let mut program = ShaderProgram::new(gl)?;
        program.load_vertex_shader(gl, vertex_shader);
        program.load_fragment_shader(gl, fragment_shader);
        program.link(gl);
        Ok(self)
    }

    pub fn with_shader_program_init<T>(mut self, gl : &Context, vertex_shader: &str, fragment_shader : &str, init_closure : Option<T>) -> Result<Self, String> where T : FnOnce(&mut ShaderProgram) {
        let mut program = ShaderProgram::new(gl)?;
        program.load_vertex_shader(gl, vertex_shader);
        program.load_fragment_shader(gl, fragment_shader);
        program.link(gl);
        if init_closure.is_some() {init_closure.unwrap()(&mut program)}
        Ok(self)
    }

    pub fn with_texture(mut self, gl : &Context, location : usize, texture_filename : &str) -> Result<Self, String> {
        if self.textures[location].is_some() {panic!("Texture location '{}' is already used by this Render Context", location)}

        let mut texture = Texture::new(gl, texture_filename);
        self.textures[location] = Some(texture);
        Ok(self)
    }

    pub fn add_texture(mut self, texture : Texture, location : usize) -> Result<Self, String> {
        if self.textures[location].is_some() {panic!("Texture location '{}' is already used by this Render Context", location)}

        self.textures[location] = Some(texture);
        Ok(self)
    }

    pub fn render(&self, gl : &Context, render_call : impl FnOnce(&Option<&mut ShaderProgram>)) {
        if self.shader_program.is_some() {
            self.shader_program.as_ref().unwrap().bind(gl);
        }
        for i in 0..16 {
            if self.textures[i].is_some() {
                self.textures[i].as_ref().unwrap().bind_index(gl, i as u8)
            }
        }
        render_call(self.shader_program);
        if self.shader_program.is_some() {
            self.shader_program.as_ref().unwrap().bind(gl);
        }
        unsafe { gl.bind_texture(TEXTURE_2D, None); }
    }
}