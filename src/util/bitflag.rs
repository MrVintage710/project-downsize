#[cfg(test)]
mod tests {
    use crate::util::bitflag::*;

    #[test]
    fn bit_flag_add() {
        let mut bitflag = BitFlag16::new();

        bitflag.mark(2);
        assert!(bitflag.is_marked(2))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BitFlag16 {
    mask : u16
}

impl BitFlag16 {
    pub fn new() -> Self {
        BitFlag16 {mask : 0}
    }

    pub fn mark(&mut self, index : u16) {
        let bit = self.indexToBit(index);
        self.mask = self.mask | bit;
    }

    pub fn is_marked(&self, index : u16) -> bool {
        let bit = self.indexToBit(index);
        let mask = self.mask & bit;
        let mask = mask >> index;
        mask == 1
    }

    pub fn max() -> u16 { 16 }

    fn indexToBit(&self, index : u16) -> u16 {
        0b1 << index
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BitFlag32 {
    mask : u32
}

impl BitFlag32 {
    pub fn new() -> Self {
        BitFlag32 {mask : 0}
    }

    pub fn mark(&mut self, index : u8) {
        let bit = self.indexToBit(index);
        self.mask = self.mask | bit as u32;
    }

    pub fn is_marked(&self, index : u8) -> bool {
        let bit = self.indexToBit(index);
        let mask = self.mask & bit as u32;
        let mask = mask >> index;
        mask == 1
    }

    pub fn max() -> u8 { 32 }

    fn indexToBit(&self, index : u8) -> u8 {
        0b1 << index
    }
}