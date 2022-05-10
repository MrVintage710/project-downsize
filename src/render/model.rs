use std::borrow::Borrow;
use crate::render::buffer::{VBO, VAO};
use glow::Context;

use std::fs::File;
use std::io::BufReader;

use std::path::{Path, PathBuf};
use std::rc::Rc;
use cgmath::{Vector2, Vector3};
use obj::{load_obj, Obj, ObjError, TexturedVertex, Vertex};
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
    pub fn new(render_context : Rc<RenderContext>, file_name : &str, shader: Shader)
        -> Result<OBJModel, ObjError> {
        let gl = &render_context.gl;
        let path = Path::new("")
            .join("assets")
            .join("models")
            .join(file_name);

        let input = BufReader::new(File::open(path)?);
        let generated_model: Obj<Vertex, u32> = load_obj(input)?;
        let obj_model: OBJModel = vao_load_obj_vertex(Rc::clone(&render_context),
                                               generated_model, shader);
        Ok(obj_model)
    }

    // pub fn update_uniforms(&mut self, gl: &Context) {
    //     self.shader.update_uniforms(&gl);
    // }
}

fn vao_load_obj_vertex(render_context: Rc<RenderContext>,
                model: Obj<Vertex, u32>, shader: Shader) -> OBJModel {
    // loads the Obj textured vertex data in to respective vertices, and returns vao with
    // all data loaded in it

    // vertexes
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
    // uv
    let mut uv_vbo = VBO::new(gl).unwrap();
    let uvs: Vec<Vector2<f32>> = model.vertices.iter()
        .map(|tv|
            Vector2::new(
                0.,
                0.,
            ))
        .collect();
    uv_vbo.load_vec2s(gl, uvs);
    // norms
    let mut norm_vbo = VBO::new(gl).unwrap();
    let norms: Vec<Vector3<f32>> = model.vertices.iter()
        .map(|tv|
            Vector3::new(
                tv.position[0],
                tv.position[1],
                tv.position[2]))
        .collect();
    norm_vbo.load_vec3s(gl, norms);

    // Create VAO, add VBOs to VAO
    let mut vao = VAO::new(gl).unwrap();

    // todo!() index buffer
    let i32_indices: Vec<i32> = model.indices.iter()
        .map(|number| *number as i32)
        .collect();

    vao.addIndexBuffer(gl, i32_indices);

    vao.add_vbo(gl, 0, &vert_vbo);
    vao.add_vbo(gl, 1, &uv_vbo);
    vao.add_vbo(gl, 2, &norm_vbo);

    let transform: Transform = Transform::default();

    // Create texture
    let texture = Texture::new(gl, "copper_block.png");

    // TODO: need to write generic shader code to allow user to define specific components
    // on a shader, as opposed to just a default shader
    // let mut shader = ShaderBuilder::new()
    //     .with_frag_shader("static_vert.glsl")
    //     .with_vert_shader("static_frag.glsl")
    //     .build(render_context).expect("Unable to create shader");

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

fn vao_load_obj(render_context: Rc<RenderContext>,
                model: Obj<TexturedVertex, u32>, shader: Shader) -> OBJModel {
    // loads the Obj textured vertex data in to respective vertices, and returns vao with
    // all data loaded in it

    // vertexes
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
    // uv
    let mut uv_vbo = VBO::new(gl).unwrap();
    let uvs: Vec<Vector2<f32>> = model.vertices.iter()
        .map(|tv|
            Vector2::new(
                tv.texture[0],
                tv.texture[1],
            ))
        .collect();
    uv_vbo.load_vec2s(gl, uvs);
    // norms
    let mut norm_vbo = VBO::new(gl).unwrap();
    let norms: Vec<Vector3<f32>> = model.vertices.iter()
        .map(|tv|
            Vector3::new(
                tv.position[0],
                tv.position[1],
                tv.position[2]))
        .collect();
    norm_vbo.load_vec3s(gl, norms);

    // Create VAO, add VBOs to VAO
    let mut vao = VAO::new(gl).unwrap();

    // todo!() index buffer
    let i32_indices: Vec<i32> = model.indices.iter()
        .map(|number| *number as i32)
        .collect();

    vao.addIndexBuffer(gl, i32_indices);

    vao.add_vbo(gl, 0, &vert_vbo);
    vao.add_vbo(gl, 1, &uv_vbo);
    vao.add_vbo(gl, 2, &norm_vbo);

    let transform: Transform = Transform::default();

    // Create texture
    let texture = Texture::new(gl, "copper_block.png");

    // TODO: need to write generic shader code to allow user to define specific components
    // on a shader, as opposed to just a default shader
    // let mut shader = ShaderBuilder::new()
    //     .with_frag_shader("static_vert.glsl")
    //     .with_vert_shader("static_frag.glsl")
    //     .build(render_context).expect("Unable to create shader");

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

// try to load model: pos, norms, verts
// then if you can't do that load pos and norm