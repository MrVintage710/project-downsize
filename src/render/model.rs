use crate::render::buffer::{VBO, VAO};
use glow::Context;
use obj::Obj;
use std::path::{Path, PathBuf};
use crate::render::texture::Texture;
use crate::render::shader::ShaderProgram;

struct OBJModel {
    texture : Option<Texture>,
    program : ShaderProgram,
    vao : VAO,
    verts : VBO,
    norms : VBO,
}

impl OBJModel {

    pub fn new(gl : &Context, file_name : &str) {
        let path = Path::new("")
            .join("assets")
            .join("models")
            .join(file_name);
        let obj = Obj::load(path.as_path());
    }

}