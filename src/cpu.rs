pub type Byte = u8;
pub type Word = u16;

pub struct CPU
{
    pub v: [Byte; 16],
    pub stack: [Word; 16],
    pub display: [[bool; CPU::DISP_X]; CPU::DISP_Y], 
    pub i: Word,
    pub pc: Word,
    pub sp: Byte,
    pub dt: Byte,
    pub st: Byte,
    pub ticks: u32,
    pub redraw: bool
}

impl CPU {
    pub const NREG: usize = 16;
    pub const MEM_SIZE: usize = 4096;
    pub const DISP_X: usize = 64;
    pub const DISP_Y: usize = 32;

    fn default() -> Self {
        Self {
            v: [0; CPU::NREG],
            stack: [0; CPU::NREG],
            display: [[false; CPU::DISP_X]; CPU::DISP_Y],
            i: 0,
            pc: 0,
            sp: 0,
            dt: 0,
            st: 0,
            ticks: 0,
            redraw: false,
        }
    }

    fn reset(&mut self) {
        self.v.iter_mut()
        .enumerate()
        .for_each(|(_i, elem)| *elem = 0x0 as u8); // Set each element to its index

    }
}