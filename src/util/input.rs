use egui::{Pos2, Rect};
use glutin::dpi::PhysicalPosition;
use glutin::event::{DeviceId, ElementState, ModifiersState, MouseButton, WindowEvent};

pub struct InputState {
    pub mouse : MouseState
}

impl InputState {
    pub fn update(&mut self) {
        self.mouse.update();
    }

    pub fn update_state(&mut self, event : &WindowEvent, dead_zone : Option<Rect>) {
        match event {
            WindowEvent::Resized(_) => {}
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested => {}
            WindowEvent::Destroyed => {}
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::ReceivedCharacter(_) => {}
            WindowEvent::Focused(_) => {}
            WindowEvent::KeyboardInput { .. } => {}
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::CursorMoved { device_id, position, modifiers } =>
                self.mouse.handle_mouse_movement(position, modifiers),
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::MouseInput { device_id, state, button, modifiers } => {
                if let Some(rect) = dead_zone {
                    if !self.mouse.is_in(rect) {
                        self.mouse.handle_mouse_state(state, button, modifiers)
                    }
                } else {
                    self.mouse.handle_mouse_state(state, button, modifiers)
                }
            },
            WindowEvent::TouchpadPressure { .. } => {}
            WindowEvent::AxisMotion { .. } => {}
            WindowEvent::Touch(_) => {}
            WindowEvent::ScaleFactorChanged { .. } => {}
            WindowEvent::ThemeChanged(_) => {}
        }
    }
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            mouse: Default::default()
        }
    }
}

pub struct MouseState {
    is_right_down : bool,
    is_middle_down : bool,
    is_left_down : bool,
    is_right_pressed : bool,
    is_middle_pressed : bool,
    is_left_pressed : bool,
    x : f32,
    y : f32,
    dx : f32,
    dy : f32
}

impl MouseState {
    pub fn update(&mut self) {
        self.is_left_pressed = false;
        self.is_middle_pressed = false;
        self.is_right_pressed = false;

        self.dx = 0.0;
        self.dy = 0.0;
    }

    pub fn handle_mouse_state(&mut self, state : &ElementState, button : &MouseButton, modifiers : &ModifiersState) {
        match button {
            MouseButton::Left => {
                match state {
                    ElementState::Pressed => {
                        self.is_left_down = true;
                        self.is_left_pressed = true;
                    }
                    ElementState::Released => {
                        self.is_left_down = false;
                        self.is_left_pressed = false;
                    }
                }
            }
            MouseButton::Right => {
                match state {
                    ElementState::Pressed => {
                        self.is_right_down = true;
                        self.is_right_pressed = true;
                    }
                    ElementState::Released => {
                        self.is_right_down = false;
                        self.is_right_pressed = false;
                    }
                }
            }
            MouseButton::Middle => {
                match state {
                    ElementState::Pressed => {
                        self.is_middle_down = true;
                        self.is_middle_pressed = true;
                    }
                    ElementState::Released => {
                        self.is_middle_down = false;
                        self.is_middle_pressed = false;
                    }
                }
            }
            MouseButton::Other(button) => {
                println!("Pressed button {}", button);
            }
        }
    }

    pub fn handle_mouse_movement(&mut self, position : &PhysicalPosition<f64>, modifiers : &ModifiersState) {
        let x = position.x as f32;
        let y = position.y as f32;
        self.dx = x - self.x;
        self.dy = y - self.y;
        self.x = x;
        self.y = y;
    }

    pub fn is_button_down(&self, button : MouseButton) -> bool {
        match button {
            MouseButton::Left => self.is_left_down,
            MouseButton::Right => self.is_right_down,
            MouseButton::Middle => self.is_middle_down,
            MouseButton::Other(_) => false
        }
    }

    pub fn is_button_pressed(&self, button : MouseButton) -> bool {
        match button {
            MouseButton::Left => self.is_left_pressed,
            MouseButton::Right => self.is_right_pressed,
            MouseButton::Middle => self.is_middle_pressed,
            MouseButton::Other(_) => false
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn dx(&self) -> f32 {
        self.dx
    }

    pub fn dy(&self) -> f32 {
        self.dy
    }

    pub fn is_in(&self, zone : Rect) -> bool {
        zone.contains(Pos2::new(self.x, self.y))
    }

    pub fn is_dragged(&self) -> bool {
        self.dx != 0.0 || self.dy != 0.0
    }

    pub fn on_drag(&self, button : MouseButton, callback : impl FnOnce(f32, f32)) {
        if (self.dx != 0.0 || self.dy != 0.0) && self.is_button_down(button) {
            callback(self.dx, self.dy)
        }
    }
}

impl Default for MouseState {
    fn default() -> Self {
        MouseState {
            is_right_down: false,
            is_middle_down: false,
            is_left_down: false,
            is_right_pressed: false,
            is_middle_pressed: false,
            is_left_pressed: false,
            x: 0.0,
            y: 0.0,
            dx: 0.0,
            dy: 0.0
        }
    }
}
