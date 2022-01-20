//Here I will add all the code that will help me in loading object, textures, and shaders into buffers
//like VBO's, VAO's, FBO's and so on.

use glow::*;

pub struct VBO {

}

impl VBO {

    pub fn new() {
        let event_loop = glutin::event_loop::EventLoop::new();

        let gl = glutin::ContextBuilder::new()
            .build_headless(&event_loop, glutin::dpi::PhysicalSize::new(32, 32))
            .expect("There was an error creating the context");

        unsafe {
            let gl = gl.make_current().expect("Test");

            let gl = glow::Context::from_loader_function(|s| gl.get_proc_address(s) as *const _);

        }
    }

}