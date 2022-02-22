use cgmath::{Matrix4, Quaternion, Vector3, SquareMatrix, Zero};
use egui::emath::Numeric;
use crate::render::debug::{Debugable, UIRenderType};
use egui::Ui;

#[derive(Clone, Copy)]
pub struct Transform  {
    pos : Vector3<f32>,
    scale: Vector3<f32>,
    rotation : Quaternion<f32>
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            pos: Vector3::zero(),
            scale: Vector3::new(1.0, 1.0, 1.0),
            rotation: Quaternion::zero()
        }
    }

    pub fn get_mat(&self) -> Matrix4<f32> {
        let mut mat = Matrix4::identity();
        mat.w.x = self.pos.x;
        mat.w.y = self.pos.y;
        mat.w.z = self.pos.z;
        mat
    }
}

impl Into<Matrix4<f32>> for Transform {
    fn into(self) -> Matrix4<f32> {
        todo!()
    }
}

impl Debugable for Transform {
    fn debug(&mut self, ui: &mut Ui, render_type: &UIRenderType) {
        self.pos.debug(ui, render_type)
    }
}