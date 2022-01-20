mod render;

use glow::*;
use crate::render::createGlutinContext;

fn main() {
    let (gl, shader_version, window, event_loop) = createGlutinContext();


}
