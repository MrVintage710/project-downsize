use crate::render::buffer::{VBO, VAO};
use glow::Context;
use obj::Obj;
use std::path::{Path, PathBuf};
use crate::Renderable;

pub struct OBJModel {
    vao : VAO,
    verts : VBO,
    norms : VBO,
}

impl OBJModel {
    pub fn new(gl : &Context, file_name : &str) -> Obj {
        let path = Path::new("")
            .join("assets")
            .join("models")
            .join(file_name);
        let obj = Obj::load(path.as_path());
        obj.expect(&*format!("Object at path {} did NOT rendered correctly", file_name))
    }
}

impl Renderable for ObjModel {
    unsafe fn render(&self, gl: &Context) {
        let mut vert_vbo = VBO::new(&gl).unwrap();
        vert_vbo.load_vec3s(&gl, verts);

        let mut uv_vbo = VBO::new(&gl)?;
        uv_vbo.load_vec2s(&gl, uvs);

        let mut norm_vbo = VBO::new(&gl)?;
        norm_vbo.load_vec3s(&gl, norm);

        let mut vao = VAO::new(&gl).unwrap();
        vao.addIndexBuffer(&gl, vec![0, 2, 1, 1, 2, 3]);
        vao.add_vbo(&gl ,0, &vert_vbo);
        vao.add_vbo(&gl, 1, &uv_vbo);
    }
}
