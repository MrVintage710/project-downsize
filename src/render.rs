use glow::{Context, HasContext, FRAMEBUFFER_SRGB};
use glutin::window::Window;
use glutin::event_loop::EventLoop;
use glutin::{ContextWrapper, PossiblyCurrent, WindowedContext};
use egui::Ui;
use egui_glow::EguiGlow;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Debug};
use egui::epaint::ClippedShape;
use glutin::event::WindowEvent;
use crate::util::*;
use std::collections::HashMap;
use std::any::TypeId;

pub mod frame;
pub mod buffer;
pub mod shader;
pub mod model;
pub mod generic;
pub mod test;

///The render contexts makes sure that anything that is .

pub struct RenderContext {
    pub gl : Context,
    egui : EguiGlow,
    render_list : Vec<Box<dyn RenderPass>>
}

impl RenderContext {
    pub fn new(window : &ContextWrapper<PossiblyCurrent, Window>, event_loop : &EventLoop<()>) -> Self {
        unsafe {
            let gl =
                glow::Context::from_loader_function(|s| window.get_proc_address(s));

            {
                use glow::HasContext as _;
                gl.enable(glow::FRAMEBUFFER_SRGB);

                let mut egui = egui_glow::EguiGlow::new(&window, &gl);

                RenderContext {
                    gl,
                    egui,
                    render_list : Vec::new()
                }
            }
        }
    }

    pub fn on_event(&mut self, event : &WindowEvent<'_>) {
        self.egui.on_event(event);
    }

    pub fn debug(&mut self, window : &ContextWrapper<PossiblyCurrent, Window>) {
        let (should_render, list) = self.egui.run(window.window(), |egui_ctx| {
            egui::SidePanel::left("side_panel").show(egui_ctx, |ui|{
                for render_group in self.render_list.iter_mut() {
                    if render_group.debugable.is_some() {
                        ui.collapsing(&render_group.name, |ui| {
                            render_group.debugable.as_mut().unwrap().debug(ui)
                        });
                    }
                }
            });
        });

        self.egui.paint(window, &self.gl, list);
    }

    pub fn render(&self) {
        unsafe {
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            for render_group in self.render_list.iter() {
                if render_group.drawable.is_some() {
                    render_group.drawable.as_ref().unwrap().render(&self.gl);
                }
            }
        }
    }

    pub fn destroy(&mut self) {
        unsafe {
            for render_group in self.render_list.iter() {
                if render_group.destroyable.is_some() {
                    render_group.destroyable.as_ref().unwrap().destroy(&self.gl)
                }
            }

            self.egui.destroy(&self.gl);
        }
    }

    pub fn add_render_group(&mut self, render_group : impl RenderPass) -> usize {
        self.render_list.push(Box::new(render_group));
        self.render_list.len() - 1
    }

    pub fn get_render_group<T>(&self, index : usize) -> &T {

        if TypeId::of::<T>() == self.render_list[index].as_any() {

        }
        &self.render_list[index]
    }

    pub fn get_render_group_mut(&mut self, index : usize) -> &mut RenderGroup {
        &mut self.render_list[index]
    }
}

pub enum RenderError {
    ShaderError,
}

pub trait RenderPass {
    unsafe fn draw(&self) -> Result<(), RenderError>;

    fn debug(&mut self) -> Result<(), RenderError> { Ok(())  }

    unsafe fn destroy(&self) -> Result<(), RenderError>;
}

pub struct RenderGroup {
    pub priority : u32,
    pub name : String,
    pub drawable : Option<Box<dyn Drawable>>,
    pub debugable : Option<Box<dyn Debugable>>,
    pub destroyable : Option<Box<dyn Destroyable>>
}

impl RenderGroup {
    pub(crate) fn new(name : String, priority : u32) -> Self {
        RenderGroup {
            name,
            priority,
            drawable : None,
            debugable : None,
            destroyable : None
        }
    }

    pub fn with_drawable<T: 'static>(mut self, drawable : T) -> Self where T : Drawable {
        self.drawable = Some(Box::new(drawable));
        self
    }

    pub fn with_debugable<T: 'static>(mut self, debugable : T) -> Self where T : Debugable {
        self.debugable = Some(Box::new(debugable));
        self
    }

    pub fn with_destroyable<T: 'static>(mut self, destroyable : T) -> Self where T : Destroyable {
        self.destroyable = Some(Box::new(destroyable));
        self
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_priority(&self) -> u32 {
        self.priority
    }
}

impl PartialEq for RenderGroup {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for RenderGroup {}

impl Ord for RenderGroup {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for RenderGroup {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for RenderGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{}]", self.name, self.priority)
    }
}

impl Display for RenderGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{}]", self.name, self.priority)
    }
}

pub trait Drawable{
    unsafe fn render(&self, gl : &Context);
}

pub trait Debugable {
    fn debug(&mut self, ui : &mut Ui);
}

pub trait Destroyable {
    unsafe fn destroy(&self, gl : &Context);
}



pub fn createGlutinContext<'a>(title : &str) -> (ContextWrapper<PossiblyCurrent, Window>, EventLoop<()> ) {
    unsafe {
        let event_loop = glutin::event_loop::EventLoop::with_user_event();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
        let window = glutin::ContextBuilder::new()
            .with_srgb(true)
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();

        (window, event_loop)
    }
}

pub fn disable_shader_program(gl : &Context) {
    unsafe {
        gl.use_program(None)
    }
}