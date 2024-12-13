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
            pc: 0x200,
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
        self.pc += 1;
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
                return;
            }
            CPU::CALL_ADDR => {
                if self.pc == 15 {
                    panic!("Stack overflow at PC {:X}", self.pc);
                }
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = (ins & 0x0FFF);
                return;
            }
            CPU::SE_VX => {
                if self.v[vx as usize] == (ins & 0x00FF) as Byte {
                    self.pc += 2;
                }
                return;
            }
            CPU::SNE_VX => {
                if self.v[vx as usize] != (ins & 0x00FF) as Byte {
                    self.pc += 2;
                }
                return;
            }
            CPU::SE_VX_VY => {
                if self.v[((ins >> 8) & 0x000F) as usize] == self.v[vy as usize] {
                    self.pc += 2;
                }
                return;
            }
            CPU::LD_VX => {
                self.v[vx as usize] = (ins & 0x00ff) as Byte;
                return;
            }
            CPU::ADD_VX => {
                let value = (ins & 0x00FF) as u8; // Extract the immediate value (lower 8 bits)
                self.v[vx as usize] = self.v[vx as usize].wrapping_add(value);
                return;
            }
            CPU::COMP_INS => {
                match ins & 0xF00F {
                    CPU::LD_VX_VY => {
                        self.v[vx as usize] = self.v[vy as usize];
                        return;
                    }
                    CPU::OR_VX_VY => {
                        self.v[vx as usize] = self.v[vx as usize] | self.v[vy as usize];
                        self.v[0xf] = 0;
                        return;
                    }
                    CPU::AND_VX_VY => {
                        self.v[vx as usize] = self.v[vx as usize] & self.v[vy as usize];
                        self.v[0xf] = 0;
                        return;
                    }
                    CPU::XOR_VX_VY => {
                        self.v[vx as usize] = self.v[vx as usize] ^ self.v[vy as usize];
                        self.v[0xf] = 0;
                        return;
                    }
                    CPU::ADD_VX_VY => {
                        let (sum, carry) = self.v[vx as usize].overflowing_add(self.v[vy as usize]);
                        self.v[vx as usize] = sum;
                        self.v[0xf] = if carry {1} else {0};
                        return;
                    }
                    CPU::SUB_VX_VY => {
                        let (diff, carry) = self.v[vx as usize].overflowing_sub(self.v[vy as usize]);
                        self.v[vx as usize] = diff;
                        self.v[0xf] = if carry {0} else {1};
                        return;
                    }
                    CPU::SHR_VX => {
                        let lsb = self.v[vx as usize] & 0x01;
                        self.v[0xF] = lsb;
                        self.v[vx as usize] >>= 1;
                        return;
                    }
                    CPU::SUBN_VX_VY => {
                        self.v[0xF] = if self.v[vy as usize] >= self.v[vx as usize] { 1 } else { 0 };
                        self.v[vx as usize] = self.v[vy as usize].wrapping_sub(self.v[vx as usize]);
                    }
                    CPU::SHL_VX => {
                        let msb = (self.v[vx as usize] & 0x80) >> 7; // Extract the MSB (bit 7)
                        self.v[0xF] = msb;                          // Set VF to the MSB (carry flag)
                        self.v[vx as usize] <<= 1;                  // Perform the left shift
                        return;
                    }
                    _ => {
                    }
                }
            }
            CPU::SNE_VX_VY => {
                if self.v[vx as usize] != self.v[vy as usize] {
                    self.pc += 2; // Skip the next instruction
                }
                return;
            }
            CPU::LD_I => {
                self.i = ins & 0x0FFF; // Load lower 12 bits of instruction into I
                return;
            }
            CPU::JP_V0 => {
                self.pc = (ins & 0x0FFF) + self.v[0x0] as Word;
                return;
            }
            CPU::RND_VX => {
                let random_byte: Byte = self.rand_byte();
                self.v[vx as usize] = random_byte & (ins & 0x00FF) as Byte;
                return;
            }
            CPU::DRW_VX_VY => {
                self.redraw = true; // Mark the display for redraw
            
                let xcoord = self.v[vx as usize] as usize % CPU::DISP_X;
                let ycoord = self.v[vy as usize] as usize % CPU::DISP_Y;
                self.v[0xF] = 0; // Reset collision flag
            
                let n = (ins & 0x000F) as usize; // Number of rows in the sprite
            
                for row in 0..n {
                    let pixel_byte = self.memory[self.i as usize + row]; // Fetch sprite row
            
                    for x in (0..8).rev() {
                        let pixel = (pixel_byte & (1 << x)) != 0; // Check if bit is set
            
                        let screen_x = (xcoord + (7 - x)) % CPU::DISP_X;
                        let screen_y = (ycoord + row) % CPU::DISP_Y;
            
                        // Check for collision
                        if pixel && self.display[screen_y][screen_x] {
                            self.v[0xF] = 1; // Set collision flag
                        }
            
                        // XOR the pixel onto the display
                        self.display[screen_y][screen_x] ^= pixel;
                    }
                }
            
                return;
            }
            CPU::KEY_OPS => {
                match ins & 0xF0FF {
                    CPU::SKP_VX => {
                        // Skip next instruction if key with the value of Vx is pressed.
                        if self.v[vx as usize] <= 0xF {
                            if self.keyboard[self.v[vx as usize] as usize] {
                                self.pc += 2; // Skip next instruction
                            }
                        }
                    }
                    CPU::SKNP_VX => {
                        // Skip next instruction if key with the value of Vx is not pressed.
                        if self.v[vx as usize] <= 0xF {
                            if !self.keyboard[self.v[vx as usize] as usize] {
                                self.pc += 2; // Skip next instruction
                            }
                        }
                    }
                    _ => {}
                }
            }
            CPU::DTST_OPS => {
                match ins & 0xF0FF {
                    CPU::LD_VX_DT => {
                        self.v[vx as usize] = self.dt;
                    }
                    CPU::LD_VX_K => {
                        for i in 0..16 {
                            if self.keyboard[i] {
                                self.v[vx as usize] = i as Byte;
                                break;
                            }
                        }
                        self.pc -= 2;
                    }
                    CPU::LD_DT_VX => {
                        self.dt = self.v[vx as usize];
                    }
                    CPU::LD_ST_VX => {
                        self.st = self.v[vx as usize];
                    }
                    CPU::ADD_I_VX => {
                        self.i += self.v[vx as usize] as Word;
                    }
                    CPU::LD_F_VX => {
                        if self.v[vx as usize] < 16 {
                            self.i = (self.v[vx as usize] as Word) * 5;
                        }
                    }
                    CPU::LD_B_VX => {
                        let value = self.v[vx as usize];
                        self.memory[self.i as usize] = value / 100;
                        self.memory[(self.i + 1) as usize] = (value / 10) % 10;
                        self.memory[(self.i + 2) as usize] = value % 10;
                    }
                    CPU::LD_STO_I_VX => {
                        // Store registers V0 through Vx in memory starting at location I
                        for i in 0..=vx as usize {
                            self.memory[self.i as usize + i] = self.v[i];
                        }
                    }
                    CPU::LD_STO_VX_I => {
                        // Read registers V0 through Vx from memory starting at location I
                        for i in 0..=vx as usize {
                            self.v[i] = self.memory[self.i as usize + i];
                        }
                    }
                    _ => {}
                }
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

    #[test]
    fn test_sevx() {
        let mut cpu = CPU::default();
        let initial_pc = cpu.pc;

        cpu.memory[cpu.pc as usize] = 0x3b;
        cpu.memory[(cpu.pc + 1) as usize] = 0x4c;
        cpu.v[0xb] = 0x4c;
        cpu.execute();

        assert_eq!(cpu.pc, initial_pc+4, "PC value does not match expected value");
    }

    #[test]
    fn test_senvx() {
        let mut cpu = CPU::default();
        let initial_pc = cpu.pc;

        cpu.memory[cpu.pc as usize] = 0x4b;
        cpu.memory[(cpu.pc + 1) as usize] = 0x8c;
        cpu.v[0xb] = 0x4b;
        cpu.execute();

        assert_eq!(cpu.pc, initial_pc+4, "PC value does not match expected value");
    }

    #[test]
    fn test_sevxvy() {
        let mut cpu = CPU::default();
        let initial_pc = cpu.pc;

        cpu.memory[cpu.pc as usize] = 0x5b;
        cpu.memory[(cpu.pc + 1) as usize] = 0xc0;
        cpu.v[0xb] = 0x4b;
        cpu.v[0xc] = 0x4b;
        cpu.execute();

        assert_eq!(cpu.pc, initial_pc+4, "PC value does not match expected value");
    }

    #[test]
    fn test_ld_vx() {
        let mut cpu = CPU::default();

        cpu.memory[cpu.pc as usize] = 0x6E;
        cpu.memory[(cpu.pc + 1) as usize] = 0xC0;

        cpu.execute();

        assert_eq!(cpu.v[0xE], 0xC0, "Register V[0xE] did not match expected value");
    }

    #[test]
    fn test_add_vx() {
        let mut cpu = CPU::default();
    
        let old_value = 0xAB;
        let add_value = 0x05;
        cpu.memory[cpu.pc as usize] = 0x7A;
        cpu.memory[(cpu.pc + 1) as usize] = add_value;
        cpu.v[0xA] = old_value;
    
        cpu.execute();
    
        assert_eq!(cpu.v[0xA], old_value.wrapping_add(add_value), "ADD_VX failed");
    
        cpu.reset();
        let old_value = 0xFA;
        let add_value = 0xEC;
        cpu.memory[cpu.pc as usize] = 0x7A;
        cpu.memory[(cpu.pc + 1) as usize] = add_value;
        cpu.v[0xA] = old_value;
    
        cpu.execute();
    
        assert_eq!(cpu.v[0xA], 0xE6, "ADD_VX failed with wraparound");
    }

    #[test]
    fn test_ld_vx_vy() {
        let mut cpu = CPU::default();

        cpu.memory[cpu.pc as usize] = 0x8E;
        cpu.memory[(cpu.pc + 1) as usize] = 0xC0;
        cpu.v[0xC] = 0xAA;

        cpu.execute();

        assert_eq!(cpu.v[0xE], cpu.v[0xC], "LD_VX_VY failed");
    }

    #[test]
    fn test_or_vx_vy() {
        let mut cpu = CPU::default();

        cpu.memory[cpu.pc as usize] = 0x8E;
        cpu.memory[(cpu.pc + 1) as usize] = 0xC1;
        cpu.v[0xE] = 0xCA;
        cpu.v[0xC] = 0xAA;

        cpu.execute();

        assert_eq!(cpu.v[0xE], 0xEA, "OR_VX_VY failed");
    }
    
    #[test]
    fn test_and_vx_vy() {
        let mut cpu = CPU::default();
    
        cpu.memory[cpu.pc as usize] = 0x8E;
        cpu.memory[(cpu.pc + 1) as usize] = 0xC2;
        cpu.v[0xE] = 0xCA;
        cpu.v[0xC] = 0xAA;
    
        cpu.execute();
    
        assert_eq!(cpu.v[0xE], 0x8A, "AND_VX_VY failed");
    }
    #[test]
    fn test_xor_vx_vy() {
        let mut cpu = CPU::default();

        cpu.memory[cpu.pc as usize] = 0x8E;
        cpu.memory[(cpu.pc + 1) as usize] = 0xC3;
        cpu.v[0xE] = 0xCA;
        cpu.v[0xC] = 0xAA;

        cpu.execute();

        assert_eq!(cpu.v[0xE], 0x60, "XOR_VX_VY failed");
    }

    #[test]
    fn test_add_vx_vy() {
        let mut cpu = CPU::default();

        cpu.memory[cpu.pc as usize] = 0x8E;
        cpu.memory[(cpu.pc + 1) as usize] = 0xC4;
        cpu.v[0xE] = 0xFA;
        cpu.v[0xC] = 0xEF;

        cpu.execute();

        assert_eq!(cpu.v[0xE], 0xE9, "ADD_VX_VY result incorrect");
        assert_eq!(cpu.v[0xF], 0x1, "Carry flag not set correctly");
    }

    #[test]
    fn test_sub_vx_vy() {
        let mut cpu = CPU::default();

        cpu.memory[cpu.pc as usize] = 0x8E;
        cpu.memory[(cpu.pc + 1) as usize] = 0xC5;
        cpu.v[0xE] = 0xCA;
        cpu.v[0xC] = 0x12;

        cpu.execute();

        assert_eq!(cpu.v[0xE], 0xB8, "SUB_VX_VY failed");
        assert_eq!(cpu.v[0xF], 0x1, "Carry flag incorrect");
    }
    #[test]
    fn test_shr_vx() {
        let mut cpu = CPU::default();
    
        cpu.memory[cpu.pc as usize] = 0x8E;
        cpu.memory[(cpu.pc + 1) as usize] = 0xC6;
        cpu.v[0xE] = 0xA5;
    
        cpu.execute();
    
        assert_eq!(cpu.v[0xE], 0x52, "SHR_VX result incorrect");
        assert_eq!(cpu.v[0xF], 0x1, "Carry flag not set correctly");
    }
    #[test]
    fn test_shl_vx() {
        let mut cpu = CPU::default();

        cpu.memory[cpu.pc as usize] = 0x8E;
        cpu.memory[(cpu.pc + 1) as usize] = 0xCE;
        cpu.v[0xE] = 0xAB;

        cpu.execute();

        assert_eq!(cpu.v[0xE], 0x56, "SHL_VX result incorrect");
        assert_eq!(cpu.v[0xF], 0x1, "Carry flag not set correctly");
    }
    #[test]
    fn test_sne_vx_vy() {
        let mut cpu = CPU::default();
        let initial_pc = cpu.pc;
    
        cpu.memory[cpu.pc as usize] = 0x9A;
        cpu.memory[(cpu.pc + 1) as usize] = 0xB0;
        cpu.v[0xA] = 0xAB;
        cpu.v[0xB] = 0xAF;
    
        cpu.execute();
    
        assert_eq!(cpu.pc, initial_pc + 4, "SNE_VX_VY failed");
    }

    #[test]
    fn test_drw() {
        let mut cpu = CPU::default();

        // Reset the CPU and load sprites into memory
        cpu.reset();
        cpu.load_sprites();

        // Setup memory for instructions
        // Load sprite address into I (0xA00F)
        cpu.memory[cpu.pc as usize] = 0xA0;
        cpu.memory[(cpu.pc + 1) as usize] = 0x0F;

        // Load x-coordinate into VA (0x6A07)
        cpu.memory[(cpu.pc + 2) as usize] = 0x6A;
        cpu.memory[(cpu.pc + 3) as usize] = 0x07;

        // Load y-coordinate into VB (0x6B20)
        cpu.memory[(cpu.pc + 4) as usize] = 0x6B;
        cpu.memory[(cpu.pc + 5) as usize] = 0x20;

        // Draw the sprite (0xDAB8) - draw sprite at VA, VB, 8 rows
        cpu.memory[(cpu.pc + 6) as usize] = 0xDA;
        cpu.memory[(cpu.pc + 7) as usize] = 0xB8;

        // Clear screen (0x00E0)
        cpu.memory[(cpu.pc + 8) as usize] = 0x00;
        cpu.memory[(cpu.pc + 9) as usize] = 0xE0;

        // 1. Load sprite address into I
        cpu.execute();
        assert_eq!(cpu.i, 0x000F, "I register did not load the sprite address correctly");

        // 2. Load x-coordinate into VA
        cpu.execute();
        assert_eq!(cpu.v[0xA], 0x07, "VA did not load the x-coordinate correctly");

        // 3. Load y-coordinate into VB
        cpu.execute();
        assert_eq!(cpu.v[0xB], 0x20, "VB did not load the y-coordinate correctly");

        // 4. Draw sprite
        cpu.execute();

        // Verify part of the display
        for y in 0..8 { // Since N = 8 rows
            let sprite_row = cpu.memory[(cpu.i + y) as usize];
            for x in 0..8 {
                let expected_pixel = (sprite_row >> (7 - x)) & 1 != 0; // Extract pixel value
                let display_x = (cpu.v[0xA] as usize + x) % CPU::DISP_X;
                let display_y = (cpu.v[0xB] as usize + y as usize) % CPU::DISP_Y;

                assert_eq!(
                    cpu.display[display_y][display_x],
                    expected_pixel,
                    "Display pixel at ({}, {}) did not match expected sprite pixel",
                    display_x,
                    display_y
                );
            }
        }

        // 5. Clear the screen
        cpu.execute();
        for row in cpu.display.iter() {
            for &cell in row.iter() {
                assert_eq!(cell, false, "Screen was not cleared properly");
            }
        }
    }
    #[test]
    fn test_add_vx_vy_nof() {
        let mut cpu = CPU::default();

        // Setup instruction: ADD Vx, Vy (no carry)
        cpu.memory[cpu.pc as usize] = 0x8E;
        cpu.memory[(cpu.pc + 1) as usize] = 0xC4;

        cpu.v[0xE] = 0xCA; // Register V[E]
        cpu.v[0xC] = 0x12; // Register V[C]

        cpu.execute();

        assert_eq!(cpu.v[0xE], 0xCA + 0x12, "ADD_VX_VY_NOF failed");
        assert_eq!(cpu.v[0xF], 0x0, "Carry flag should be 0");
    }
    #[test]
    fn test_subn_vx_vy() {
        let mut cpu = CPU::default();
    
        cpu.memory[cpu.pc as usize] = 0x8A;
        cpu.memory[(cpu.pc + 1) as usize] = 0xB7;
    
        cpu.v[0xA] = 0x0A; // Register V[A]
        cpu.v[0xB] = 0xAC; // Register V[B]
    
        cpu.execute();
    
        assert_eq!(cpu.v[0xA], 0xAC - 0x0A, "SUBN_VX_VY failed");
        assert_eq!(cpu.v[0xF], 0x1, "Carry flag should be 1");
    }
    #[test]
    fn test_ld_i() {
        let mut cpu = CPU::default();
    
        cpu.memory[cpu.pc as usize] = 0xAA;
        cpu.memory[(cpu.pc + 1) as usize] = 0xBC;
    
        cpu.execute();
    
        assert_eq!(cpu.i, 0x0ABC, "LD_I failed to load correct value into I register");
    }
    #[test]
    fn test_jp_v0() {
        let mut cpu = CPU::default();
    
        cpu.memory[cpu.pc as usize] = 0xBA;
        cpu.memory[(cpu.pc + 1) as usize] = 0xBC;
        cpu.v[0x0] = 0x09;
    
        cpu.execute();
    
        assert_eq!(cpu.pc, 0x0ABC + 0x09, "JP_V0 failed to jump with offset");
    }
    #[test]
    fn test_subroutine() {
        let mut cpu = CPU::default();
        // CALL 0x0A00
        cpu.memory[cpu.pc as usize] = 0x2A;
        cpu.memory[(cpu.pc + 1) as usize] = 0x00;
    
        // Instruction at address 0x0A00
        cpu.memory[0x0A00] = 0x7A;
        cpu.memory[0x0A01] = 0x69;
    
        // RET instruction at 0x0A03
        cpu.memory[0x0A02] = 0x00;
        cpu.memory[0x0A03] = 0xEE;
    
        // Call Subroutine
        cpu.execute();
        assert_eq!(cpu.pc, 0x0A00, "CALL did not jump to subroutine");
        assert_eq!(cpu.stack[cpu.sp as usize - 1], 0x0200 + 2, "Stack did not store return address");
    
        // Execute next instruction in subroutine
        cpu.execute();
        assert_eq!(cpu.v[0xA], 0x69, "Instruction in subroutine failed");
    
        // Return from subroutine
        cpu.execute();
        assert_eq!(cpu.pc, 0x0202, "RET did not return to correct address");
    }
    


}