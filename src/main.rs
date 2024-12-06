#![allow(unused)]
mod cpu;
use cpu::CPU;

fn main() {
    let mut cpu: CPU = cpu::CPU::default();
    cpu.reset();
    cpu.load_sprites();
    println!("{:X?}", cpu.memory);
}
