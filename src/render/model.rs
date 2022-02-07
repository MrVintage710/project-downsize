use std::io::BufReader;
use std::fs::File;
use obj::{Obj, load_obj, ObjError};
use crate::render::shader::ShaderProgram;
use crate::render::buffer::{VBO, VAO};
use egui_glow::glow::Context;
use std::error::Error;
use crate::render::RenderContext;
use cgmath::{Vector3, Matrix4};

struct OBJModel {

}

impl OBJModel {
    pub fn new(gl : &Context, model_filename : &str) -> Self {
        let input = BufReader::new(File::open(format!("assets/models/{}", model_filename)).expect(""));
        let o : Obj = load_obj(input).expect("Object could not load from file.");

        let mut vert_vbo = VBO::new(gl).expect("");
        let mut normal_vbo = VBO::new(gl).expect("");

        let mut vert_pos_vec = Vec::new();
        let mut vert_norm_vec = Vec::new();

        for vert in o.vertices.iter() {
            vert_pos_vec.push(Vector3::new(vert.position[0], vert.position[1], vert.position[2]));
            vert_norm_vec.push(Vector3::new(vert.normal[0], vert.normal[1], vert.normal[2]));
        };

        vert_vbo.load_vec3s(gl, vert_pos_vec);
        normal_vbo.load_vec3s(gl, vert_norm_vec);

        let mut vao = VAO::new(gl).expect("");
        vao.add_vbo(gl, 0, &vert_vbo);
        vao.add_vbo(gl, 1, &normal_vbo);

        let mut shader_program = ShaderProgram::new(gl).expect("Unable to make Shader Program.");
        shader_program.load_vertex_shader(gl, "models/static_model_vert.glsl");
        shader_program.load_fragment_shader(gl, "models/static_model_frag.glsl");

        OBJModel {}
    }
}