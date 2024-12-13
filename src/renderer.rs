use sfml::graphics::{Color, Image, RenderWindow, Texture, Sprite};
use sfml::window::{Event, Key, Style};
use crate::CPU;


pub struct Renderer<'a> {
    pub window: &'a mut RenderWindow,
    pub cpu: &'a mut CPU,
    pub image: Image,
    pub window_size_x: u32,
    pub window_size_y: u32,
}

impl<'a> Renderer<'a> {
    pub const DISP_SIZE_X: u32 = 64;
    pub const DISP_SIZE_Y: u32 = 64;
    pub const HEXKEYS: [Key; 16] = [
        Key::Numpad0,
        Key::Numpad1,
        Key::Numpad2,
        Key::Numpad3,
        Key::Numpad4,
        Key::Numpad5,
        Key::Numpad6,
        Key::Numpad7,
        Key::Numpad8,
        Key::Numpad9,
        Key::A,
        Key::B,
        Key::C,
        Key::D,
        Key::E,
        Key::F
    ];

    pub fn new(window: &'a mut RenderWindow, cpu: &'a mut CPU, window_size_x: u32, window_size_y: u32) -> Self {
        let image = Image::new(Renderer::DISP_SIZE_X, Renderer::DISP_SIZE_Y);
        Self {window, cpu, image, window_size_x, window_size_y}
    }

    pub fn draw_buffer(&mut self) {
        for y in 0..Renderer::DISP_SIZE_Y {
            for x in 0..Renderer::DISP_SIZE_Y {
                if self.cpu.display[y as usize][x as usize] == true {
                    let color = if self.cpu.display[y as usize][x as usize] {
                        Color::YELLOW
                    } else {
                        Color::BLACK
                    };
                    // TODO
                }
            }
        }
    }
}
