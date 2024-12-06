use rand::rngs::ThreadRng;
use rand::Rng;
pub type Byte = u8;
pub type Word = u16;

pub struct CPU
{
    pub memory: [Byte; CPU::MEM_SIZE],
    pub v: [Byte; 16],
    pub stack: [Word; 16],
    pub display: [[bool; CPU::DISP_X]; CPU::DISP_Y], 
    pub i: Word,
    pub pc: Word,
    pub sp: Byte,
    pub dt: Byte,
    pub st: Byte,
    pub ticks: u32,
    pub redraw: bool,
    pub keyboard: [bool; 16],
    pub rng: ThreadRng

}

impl CPU {
    pub const NREG: usize = 16;
    pub const MEM_SIZE: usize = 4096;
    pub const DISP_X: usize = 64;
    pub const DISP_Y: usize = 32;

    pub const SYS_ADDR: Word = 0x0000;
    pub const CLS: Word = 0x00E0;
    pub const RET: Word = 0x00EE;
    pub const JP_ADDR: Word = 0x1000;
    pub const CALL_ADDR: Word = 0x2000;
    pub const SE_VX: Word = 0x3000;
    pub const SNE_VX: Word = 0x4000;
    pub const SE_VX_VY: Word = 0x5000;
    pub const LD_VX: Word = 0x6000;
    pub const ADD_VX: Word = 0x7000;
    pub const COMP_INS: Word = 0x8000;
    pub const LD_VX_VY: Word = 0x8000;
    pub const OR_VX_VY: Word = 0x8001;
    pub const AND_VX_VY: Word = 0x8002;
    pub const XOR_VX_VY: Word = 0x8003;
    pub const ADD_VX_VY: Word = 0x8004;
    pub const SUB_VX_VY: Word = 0x8005;
    pub const SHR_VX: Word = 0x8006;
    pub const SUBN_VX_VY: Word = 0x8007;
    pub const SHL_VX: Word = 0x800E;
    pub const SNE_VX_VY: Word = 0x9000;
    pub const LD_I: Word = 0xA000;
    pub const JP_V0: Word = 0xB000;
    pub const RND_VX: Word = 0xC000;
    pub const DRW_VX_VY: Word = 0xD000;
    pub const KEY_OPS: Word = 0xE000;
    pub const SKP_VX: Word = 0xE09E;
    pub const SKNP_VX: Word = 0xE0A1;
    pub const DTST_OPS: Word = 0xF000;
    pub const LD_VX_DT: Word = 0xF007;
    pub const LD_VX_K: Word = 0xF00A;
    pub const LD_DT_VX: Word = 0xF015;
    pub const LD_ST_VX: Word = 0xF018;
    pub const ADD_I_VX: Word = 0xF01E;
    pub const LD_F_VX: Word = 0xF029;
    pub const LD_B_VX: Word = 0xF033;
    pub const LD_STO_I_VX: Word = 0xF055;
    pub const LD_STO_VX_I: Word = 0xF065;
    pub const CHARACTERS: [Byte; 128] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, 0x00, 0x00, 0x00, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, 0x00, 0x00, 0x00, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, 0x00, 0x00, 0x00, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x00, 0x00, 0x00, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, 0x00, 0x00, 0x00, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, 0x00, 0x00, 0x00, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, 0x00, 0x00, 0x00, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, 0x00, 0x00, 0x00, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, 0x00, 0x00, 0x00, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, 0x00, 0x00, 0x00, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, 0x00, 0x00, 0x00, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, 0x00, 0x00, 0x00, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, 0x00, 0x00, 0x00, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, 0x00, 0x00, 0x00, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, 0x00, 0x00, 0x00, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, 0x00, 0x00, 0x00, // F
    ];

    pub fn default() -> Self {
        Self {
            memory: [0; CPU::MEM_SIZE],
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
            keyboard: [false; 16],
            rng: rand::thread_rng()
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0x200;
        self.sp = 0x0;
        self.i = 0x0;
    }

    pub fn fetch(&mut self) -> Word {
        let ins1: Byte = self.memory[self.pc as usize];
        self.pc += 1;
        let ins2: Byte = self.memory[self.pc as usize];
        let data: Word = ((ins1 as Word) << 8) | (ins2 as Word);
        return data;
    }

    pub fn rand_byte(&mut self) -> Byte {
        return self.rng.gen_range(0..=255);
    }

    pub fn load_sprites(&mut self) {
        for (index, &val) in CPU::CHARACTERS.iter().enumerate() {
            self.memory[index] = val;
        }
    }

    pub fn reset_display(&mut self) {
        for row in self.display.iter_mut() {
            for cell in row.iter_mut() {
                *cell = false; // Reset each cell to false
            }
        }
    }

    pub fn execute(&mut self) {
        let ins: Word = self.fetch();
        let vx: Word = (ins >> 8) & 0x000F;
        let vy: Word = (ins >> 4) & 0x000F;
        let xcoord: Byte;
        let ycoord: Byte;
        let carry: Byte;
        let n: Word;
        let pixel_byte: Byte;
        let pixel: bool;
        self.ticks += 1;

        if self.ticks % 9 == 0 {
            if self.dt > 0
            {
                self.dt -= 1;
            }
            if self.st > 0
            {
                self.st -= 1;
            }
        }
        match ins {
            CPU::SYS_ADDR => { return }
            CPU::CLS => {
                self.redraw = true;
                self.reset_display();
                return;
            }
            CPU::RET => {
                if self.sp == 0 {
                    panic!("Stack Underflow at PC {:X}", self.pc);
                }
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
                return;
            }
            _ => {}
        }

        match ins & 0xF000 {
            CPU::JP_ADDR => {
                self.pc = ins & 0xFFF;
            }
            _ => {}
        }

    }
}


#[cfg(test)]
mod tests {
    use super::*; // Import CPU, Memory, and other necessary items

    #[test]
    fn test_fetch() {
        let mut cpu = CPU::default(); // Assuming CPU has a default constructor

        // Set memory data at the program counter
        cpu.memory[cpu.pc as usize] = 0x1A;
        cpu.memory[(cpu.pc + 1) as usize] = 0xFA;

        // Call the fetch method
        let fetched = cpu.fetch();

        // Assert the expected value
        assert_eq!(fetched, 0x1AFA, "Fetched value does not match expected value");
    }

    #[test]
    fn test_jmp() {
        let mut cpu = CPU::default();

        cpu.memory[cpu.pc as usize] = 0x1F;
        cpu.memory[(cpu.pc + 1) as usize] = 0x4C;

        cpu.execute();

        assert_eq!(cpu.pc, 0x0F4C, "PC value does not match expected value");
    }


}