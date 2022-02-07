use glow::Context;
use crate::render::{Drawable, Debugable};
use egui::Ui;

pub struct GenericDrawable {
    callback : Box<dyn Fn(&Context)>
}

impl GenericDrawable {
    pub fn new(callback: impl Fn(&Context) + 'static) -> Self {
        GenericDrawable { callback: Box::new(callback) }
    }
}

impl Drawable for GenericDrawable {
    unsafe fn render(&self, gl: &Context) {
        (self.callback)(gl)
    }
}

pub struct GenericDebug {
    callback : Box<dyn Fn(&mut Ui)>
}

impl GenericDebug {
    pub fn new(callback: impl Fn(&mut Ui) + 'static) -> Self {
        GenericDebug{ callback: Box::new(callback) }
    }
}

impl Debugable for GenericDebug {
    fn debug(&mut self, ui: &mut Ui) {
        (self.callback)(ui)
    }
}