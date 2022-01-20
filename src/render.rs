use glow::Context;
use glutin::window::Window;
use glutin::event_loop::EventLoop;
use glutin::{ContextWrapper, PossiblyCurrent};

pub mod frame;
pub mod buffer;

pub trait Renderable {
    fn render(gl : Context);
}

pub fn createGlutinContext() -> (Context, &str, ContextWrapper<PossiblyCurrent, Window>, EventLoop<()> ) {
    unsafe {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("Hello triangle!")
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();
        let gl =
            glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
        (gl, "#version 410", window, event_loop)
    }
}