use std::rc::Rc;
use glow::*;
use glutin::window::Window;
use glutin::event_loop::EventLoop;
use glutin::{ContextWrapper, PossiblyCurrent, WindowedContext};
use egui_glow::EguiGlow;
use egui::Ui;
use glutin::dpi::PhysicalSize;

pub mod frame;
pub mod buffer;
pub mod shader;
pub mod texture;
pub mod debug;
pub mod transform;
pub mod model;
pub mod downsize;
pub mod lighting;

pub trait Renderable {
    unsafe fn render(&self, gl : &Context);
}

pub trait Deletable {
    unsafe fn delete(&self, gl : &Context);
}

pub struct RenderContext {
    pub gl : Context,
    pub window : ContextWrapper<PossiblyCurrent, Window>,
}
//
// impl RenderContext {
//
//     pub fn gl(&self, gl_callback : impl FnOnce(&Context)) -> Result<(), RenderError> {
//         gl_callback(&self.gl);
//         self.has_gl_error()
//     }
//
//     pub fn render(&self, renderPacket : impl Renderable) -> Result<(), RenderError> {
//         unsafe { renderPacket.render(&self.gl); }
//         self.has_gl_error()
//     }
//
//     pub fn get_window_size(&self) -> PhysicalSize<u32> {
//         self.window.window().inner_size()
//     }
//
//     pub fn has_gl_error(&self) -> Result<(), RenderError> {
//         unsafe {
//             let error = RenderError::from_error_code(self.gl.get_error());
//
//             match error {
//                 RenderError::UNKNOWN => {}
//                 _ => {return Err(error)}
//             }
//         }
//
//         Ok(())
//     }
// }
//
//
// #[derive(Debug)]
// pub enum RenderError {
//     INVALID_ENUM,
//     INVALID_VALUE,
//     INVALID_OPERATION,
//     STACK_OVERFLOW,
//     STACK_UNDERFLOW,
//     OUT_OF_MEMORY,
//     INVALID_FRAMEBUFFER_OPERATION,
//     CONTEXT_LOST,
//     UNKNOWN
// }
//
// impl RenderError {
//     pub fn from_error_code(error : u32) -> Self {
//         if error == INVALID_ENUM {return RenderError::INVALID_ENUM}
//         else if error == INVALID_VALUE { return RenderError::INVALID_VALUE}
//         else if error == INVALID_OPERATION { return RenderError::INVALID_OPERATION }
//         else if error == STACK_OVERFLOW { return RenderError::STACK_OVERFLOW }
//         else if error == STACK_UNDERFLOW { return RenderError::STACK_UNDERFLOW }
//         else if error == OUT_OF_MEMORY { return RenderError::OUT_OF_MEMORY }
//         else if error == INVALID_FRAMEBUFFER_OPERATION { return RenderError::INVALID_FRAMEBUFFER_OPERATION }
//         else if error == CONTEXT_LOST { return RenderError::CONTEXT_LOST }
//
//         RenderError::UNKNOWN
//     }
// }

pub fn createGlutinContext<'a>(title : &str) -> (Rc<RenderContext>, &'a str, EventLoop<()>, EguiGlow) {
    unsafe {
        let event_loop = glutin::event_loop::EventLoop::with_user_event();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
        let window = glutin::ContextBuilder::new()
            .with_depth_buffer(0)
            .with_srgb(true)
            .with_stencil_buffer(0)
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();

        let gl =
            glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);

        gl.enable(FRAMEBUFFER_SRGB);
        gl.enable(DEPTH_TEST);

        // gl.enable(CULL_FACE);
        // gl.cull_face(BACK);
        // gl.front_face(CCW);

        let mut egui_glow = egui_glow::EguiGlow::new(&window, &gl);

        // gl.debug_message_callback(|source, t, id, severity, message| {
        //     let mut severity_text = "";
        //     if severity == DEBUG_SEVERITY_LOW {severity_text = "LOW"}
        //     if severity == DEBUG_SEVERITY_MEDIUM {severity_text = "MED"}
        //     if severity == DEBUG_SEVERITY_HIGH {severity_text = "HIGH"}
        //     if severity == DEBUG_SEVERITY_NOTIFICATION {severity_text = "LOG"; return;}
        //
        //     println!("[GL ERROR][{}]:{}", severity_text, message)
        // });

        (Rc::new(RenderContext{gl, window }) , "#version 410", event_loop, egui_glow)
    }
}