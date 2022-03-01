use crate::render::buffer::{VBO, VAO};
use glow::Context;
use obj::Obj;
use std::path::{Path, PathBuf};

struct OBJModel {
    vao : VAO,
    verts : VBO,
    norms : VBO,
}

impl OBJModel {

    pub fn new(gl : &Context, file_name : &str) {
        let path = PathBuf::new();
        path.join("assets/models");
        path.join(file_name);
        let obj = Obj::load(path.as_path());
    }

}