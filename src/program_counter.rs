#[derive(Default)]
pub struct ProgramCounter {
    pub pc: u16,
    pub changed: bool
}

impl ProgramCounter {

    pub fn set(&mut self, v: u16) {
        self.pc = v;
        self.changed = true;
    }

    pub fn reset_flag(&mut self) {
        self.changed = false;
    }
}
