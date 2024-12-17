#![allow(unused)]
#![allow(deprecated)]
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, console};
mod cpu;
use cpu::CPU;

#[wasm_bindgen]
pub struct Chip8 {
    cpu: CPU,
    display: [[bool; CPU::DISP_X]; CPU::DISP_Y]
}


#[wasm_bindgen]
impl Chip8 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: CPU::default(),
            display: [[false; CPU::DISP_X]; CPU::DISP_Y]
        }
    }

    #[wasm_bindgen]
    pub fn init(&mut self) {
        self.cpu.init(); // <-- Add safeguard here
    }

   #[wasm_bindgen]
    pub fn step(&mut self) {
        self.cpu.execute();
        self.display = self.cpu.get_display();
    }

   #[wasm_bindgen]
   pub fn render(&self, context: CanvasRenderingContext2d, scale: u32) {
       context.set_fill_style(&JsValue::from_str("black"));
       context.fill_rect(0.0, 0.0, (64 * scale) as f64, (32 * scale) as f64);

       for y in 0..32 {
           for x in 0..64 {
               if self.display[y][x] {
                   context.set_fill_style(&JsValue::from_str("yellow"));
                   context.fill_rect(
                       (x as u32 * scale) as f64,
                       (y as u32 * scale) as f64,
                       scale as f64,
                       scale as f64,
                   );
               }
           }
       }
   }

   #[wasm_bindgen]
   pub fn update_keyboard(&mut self, key_states: &[u8]) {
       for (i, &state) in key_states.iter().enumerate().take(16) {
           self.cpu.keyboard[i] = state;
       }
   }
}