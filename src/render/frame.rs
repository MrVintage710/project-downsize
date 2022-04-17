//Here Im going to make function that can take in 3D Model files and render them to framebuffers
//so that we can do post process effects on them. This will also downsize the render to a desired frame size.

use glow::Context;

pub struct FrameBufferBuilder {
    color_attachments: [Option<(u32, u32)>; 32],
}

impl FrameBufferBuilder {
    pub fn with_texture_attachment(mut self, width : u32, height : u32, color_attachment : usize) -> Self {
        self.color_attachments[color_attachment] = Some((width, height));
        self
    }
}

//input: 3D models
//output: framebuffers
