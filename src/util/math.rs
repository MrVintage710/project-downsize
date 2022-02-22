use cgmath::Matrix4;
use egui::emath::Numeric;

pub fn matrix_4_to_slice(mat : &Matrix4<f32>) -> &[f32;16] {
    let mut array : [f32; 16] = [0.0; 16];
    array[0] = mat.x.x;
    array[1] = mat.x.y;
    array[2] = mat.x.z;
    &array
}