//Here I will add all the code that will help me in loading object, textures, and shaders into buffers
//like VBO's, VAO's, FBO's and so on.

use glow::*;

pub struct VBO {
    buffer : NativeBuffer
}

impl VBO {
    pub fn new(gl : &Context) -> Result<Self, String> {
        unsafe {
            let buffer = gl.create_buffer()?;
            Ok(VBO { buffer })
        }
    }

    pub fn load_int3(&self, gl : &Context, ) {

    }
}