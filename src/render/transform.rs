use cgmath::{Matrix4, Quaternion, Vector3, SquareMatrix, Zero, Deg};
use egui::emath::Numeric;
use crate::render::debug::{Debugable, UIRenderType};
use egui::Ui;
use transform_matrix::Transform as TransformMatrix;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform  {
    pos : Vector3<f32>,
    scale: Vector3<f32>,
    rotation : Vector3<f32>
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            pos: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            rotation: Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn get_mat(&self) -> Matrix4<f32> {
        let transform_mat = Matrix4::from_translation(self.pos);
        let scale_mat = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        let rotation_x_mat = Matrix4::from_angle_x(Deg(self.rotation.x));
        let rotation_y_mat = Matrix4::from_angle_y(Deg(self.rotation.y));
        let rotation_z_mat = Matrix4::from_angle_z(Deg(self.rotation.z));
        let mat = rotation_x_mat * rotation_y_mat * rotation_z_mat * transform_mat * scale_mat;
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
        ui.vertical(|ui|{
            ui.horizontal(|ui| {
                ui.label("Position");
                self.pos.debug(ui, render_type)
            });
            ui.horizontal(|ui| {
                ui.label("Scale");
                self.scale.debug(ui, render_type);
            });
            ui.horizontal(|ui| {
                ui.label("Rotation");
                self.rotation.debug(ui, render_type)
            });
        });
    }
}