use crate::render::buffer::{VBO, VAO};
use glow::Context;

use std::fs::File;
use std::io::BufReader;
use obj::{load_obj, Obj, ObjError, ObjResult, TexturedVertex};

use std::path::{Path, PathBuf};
use cgmath::{Vector2, Vector3};
use crate::render::texture::Texture;
use crate::render::shader::ShaderProgram;

pub struct OBJModel {
    texture : Option<Texture>,
    program : ShaderProgram,
    vao : VAO,
    verts : VBO,
    uvs: VBO,
    norms : VBO,
}

impl OBJModel {
    pub fn new(gl : &Context, file_name : &str) -> Result<OBJModel, ObjError> {

        let path = Path::new("")
            .join("assets")
            .join("models")
            .join(file_name);

        let input = BufReader::new(File::open(path)?);
        let model: Obj<TexturedVertex, u32> = load_obj(input)?;
        let obj_model: OBJModel = vao_load_obj(&gl, &model);
        Ok(obj_model)
    }
}

fn vao_load_obj(gl: &Context, model: &Obj<TexturedVertex, u32>) -> OBJModel {
    // loads the Obj textured vertex data in to respective vertices, and returns vao with
    // all data loaded in it

    // vertexes
    let mut vert_vbo = VBO::new(&gl).unwrap();
    let verts: Vec<Vector3<u32>> = model.vertices.iter()
        .map(|tv|
            Vector3::new(
            tv.position[0] as u32,
            tv.position[1] as u32,
            tv.position[2] as u32))
        .collect();
    vert_vbo.load_vec3s(&gl, verts);
    // uv
    let mut uv_vbo = VBO::new(&gl).unwrap();
    let uvs: Vec<Vector2<u32>> = model.vertices.iter()
        .map(|tv|
        Vector2::new(
            tv.texture[0] as u32,
            tv.texture[1] as u32,
        ))
        .collect();
    uv_vbo.load_vec2s(&gl, uvs);
    // norms
    let mut norm_vbo = VBO::new(&gl).unwrap();
    let norms: Vec<Vector3<u32>> = model.vertices.iter()
        .map(|tv|
        Vector3::new(
            tv.position[0] as u32,
            tv.position[1] as u32,
            tv.position[2] as u32))
        .collect();
    norm_vbo.load_vec3s(&gl, norms);

    // Create VAO, add VBOs to VAO
    let mut vao = VAO::new(&gl).unwrap();

    // todo!() index buffer
    vao.addIndexBuffer(&gl, Vec::from::<i32>(model.indices.clone()));

    vao.add_vbo(&gl, 0, &vert_vbo);
    vao.add_vbo(&gl, 1, &uv_vbo);
    vao.add_vbo(&gl, 2, &norm_vbo);

    // Create texture
    let texture = Texture::new(&gl, "copper_block.png");

    // Load and Link Shaders
    let mut shader_program = ShaderProgram::new(&gl).expect("Expected shader Program");
    shader_program.load_vertex_shader(&gl, "static_vert.glsl");
    shader_program.load_fragment_shader(&gl, "static_frag.glsl");
    shader_program.link(&gl);

    OBJModel{
        texture: Some(texture),
        program: shader_program,
        vao,
        verts: vert_vbo,
        uvs: uv_vbo,
        norms: norm_vbo,
    }
}