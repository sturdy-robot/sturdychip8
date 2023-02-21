pub struct Registers {
    pub v_r: [u8; 16],
    pub pc: u16,
    pub i: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            v_r: [0; 16],
            pc: 0x200,
            i: 0,
        }
    }
}