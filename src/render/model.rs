use std::borrow::Borrow;
use crate::render::buffer::{VBO, VAO};
use glow::Context;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use cgmath::{Vector2, Vector3};
use obj::{FromRawVertex, load_obj, Obj, ObjError, ObjResult, TexturedVertex, Vertex};
use crate::render::shader::Shader;
use crate::render::texture::Texture;
use crate::{Renderable, ShaderBuilder, Transform};
use crate::render::{Deletable, RenderContext};

pub struct OBJModel {
    pub texture : Option<Texture>,
    pub shader: Shader,
    pub vao : VAO,
    pub verts : VBO,
    pub uvs: VBO,
    norms : VBO,
    pub transform: Transform,
}

impl Renderable for OBJModel {
    unsafe fn render(&self, gl: &Context) {
        self.texture.as_ref().unwrap().bind(gl);
        self.shader.bind();
        self.vao.render(gl);
    }
}

impl OBJModel {
    pub fn new(render_context : &Rc<RenderContext>, file_name : &str, shader: Shader)
        -> Result<OBJModel, ObjError> {
        let gl = &render_context.gl;
        let path = Path::new("")
            .join("assets")
            .join("models")
            .join(file_name);

        let input = BufReader::new(File::open(path.clone())?);

        // attempt to load obj with textured vertex data
        // if there is a load error, attempt to load obj without textured vertex data
        let model: ObjResult<Obj<TexturedVertex, u32>> = load_obj(input);
        if let Err(ObjError::Load(e)) = model{
            let input = BufReader::new(File::open(path.clone())?);
            let model: ObjResult<Obj<Vertex, u32>> = load_obj(input);
            let obj_model: OBJModel =
                vao_load_obj_vertex(Rc::clone(&render_context),model.unwrap(), shader);
            Ok(obj_model)
        } else {
            let obj_model: OBJModel =
                vao_load_obj_textured_vertex(Rc::clone(&render_context),model.unwrap(), shader);
            Ok(obj_model)
        }
    }
}

fn vao_load_obj_vertex(render_context: Rc<RenderContext>, model: Obj<Vertex, u32>, shader: Shader) -> OBJModel {
    // load vertexes into vert_vbo
    let gl = &render_context.gl;
    let mut vert_vbo = VBO::new(gl).unwrap();
    let verts: Vec<Vector3<f32>> = model.vertices.iter()
        .map(|tv|
            Vector3::new(
                tv.position[0],
                tv.position[1],
                tv.position[2]))
        .collect();
    vert_vbo.load_vec3s(gl, verts);

    // load uvs into uv_vbo
    let mut uv_vbo = VBO::new(gl).unwrap();
    let uvs: Vec<Vector2<f32>> = model.vertices.iter()
        .map(|tv|
            Vector2::new(
                0.,
                0.,
            ))
        .collect();
    uv_vbo.load_vec2s(gl, uvs);

    // load norms into norms_vbo
    let mut norm_vbo = VBO::new(gl).unwrap();
    let norms: Vec<Vector3<f32>> = model.vertices.iter()
        .map(|tv|
            Vector3::new(
                tv.position[0],
                tv.position[1],
                tv.position[2]))
        .collect();
    norm_vbo.load_vec3s(gl, norms);

    // load indices from model and convert them to i32
    let indices: Vec<i32> = model.indices.iter()
        .map(|number| *number as i32)
        .collect();

    // Create VAO, add VBOs and indices to VAO
    let mut vao = VAO::new(gl).unwrap();
    vao.addIndexBuffer(gl, indices);
    vao.add_vbo(gl, 0, &vert_vbo);
    vao.add_vbo(gl, 1, &uv_vbo);
    vao.add_vbo(gl, 2, &norm_vbo);

    let texture = Texture::new(gl, "copper_block.png");
    let transform: Transform = Transform::default();

    OBJModel {
        texture: Some(texture),
        shader,
        vao,
        verts: vert_vbo,
        uvs: uv_vbo,
        norms: norm_vbo,
        transform,
    }
}
fn vao_load_obj_textured_vertex(render_context: Rc<RenderContext>, model: Obj<TexturedVertex, u32>, shader: Shader) -> OBJModel {
    // load vertexes into vert_vbo
    let gl = &render_context.gl;
    let mut vert_vbo = VBO::new(gl).unwrap();
    let verts: Vec<Vector3<f32>> = model.vertices.iter()
        .map(|tv|
            Vector3::new(
                tv.position[0],
                tv.position[1],
                tv.position[2]))
        .collect();
    vert_vbo.load_vec3s(gl, verts);

    // load uvs into uv_vbo
    let mut uv_vbo = VBO::new(gl).unwrap();
    let uvs: Vec<Vector2<f32>> = model.vertices.iter()
        .map(|tv|
            Vector2::new(
                tv.texture[0],
                tv.texture[1],
            ))
        .collect();
    uv_vbo.load_vec2s(gl, uvs);

    // load norms into norms_vbo
    let mut norm_vbo = VBO::new(gl).unwrap();
    let norms: Vec<Vector3<f32>> = model.vertices.iter()
        .map(|tv|
            Vector3::new(
                tv.position[0],
                tv.position[1],
                tv.position[2]))
        .collect();
    norm_vbo.load_vec3s(gl, norms);

    // load indices from model and convert them to i32
    let indices: Vec<i32> = model.indices.iter()
        .map(|number| *number as i32)
        .collect();

    // Create VAO, add VBOs and indices to VAO
    let mut vao = VAO::new(gl).unwrap();
    vao.addIndexBuffer(gl, indices);
    vao.add_vbo(gl, 0, &vert_vbo);
    vao.add_vbo(gl, 1, &uv_vbo);
    vao.add_vbo(gl, 2, &norm_vbo);

    let texture = Texture::new(gl, "copper_block.png");
    let transform: Transform = Transform::default();

    OBJModel {
        texture: Some(texture),
        shader,
        vao,
        verts: vert_vbo,
        uvs: uv_vbo,
        norms: norm_vbo,
        transform,
    }
}

impl Deletable for OBJModel {
    unsafe fn delete(&self, gl: &Context) {
        self.vao.delete(&gl);
        self.verts.delete(&gl);
        self.uvs.delete(&gl);
        self.shader.delete(&gl);
    }
}