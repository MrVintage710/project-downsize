
pub struct UpdateVariable<T> {
    has_changed : bool,
    value : T
}

impl <T> UpdateVariable<T> {

    pub fn from(value : T) -> Self {
        UpdateVariable {
            has_changed: true,
            value
        }
    }

    pub fn has_changed(&self) -> bool {
        self.has_changed
    }

    pub fn mutate_value(&mut self, mutate_callback : impl FnOnce(&mut T)) {
        mutate_callback(&mut self.value);
        self.has_changed = true;
    }

    pub fn set_value(&mut self, value : T) {
        self.value = value;
        self.has_changed;
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }
}