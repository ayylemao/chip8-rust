#![allow(unused)]
use std::fs::File;
use std::io::Read;
use std::io::BufReader;


mod renderer; // Import the renderer module

use cpu::Byte;
use sfml::graphics::IntRect;
use sfml::graphics::RenderTarget;
use sfml::graphics::Transformable;
use sfml::graphics::{Color, Image, RenderWindow, Texture, Sprite};
use sfml::window::{Event, Key, Style};
use crate::renderer::Renderer;

// Assuming CPU is defined in another file
mod cpu; // Import the CPU module
use crate::cpu::CPU;

fn main() {

    let mut cpu = CPU::default();
    cpu.reset();
    cpu.load_sprites();

    let filename = "1-chip8-logo.ch8";
    let buffer = BufReader::new(File::open(filename).unwrap()); 
    let mut inc: usize = 0;
    for byte in buffer.bytes() {
        cpu.memory[cpu.pc as usize + inc] = byte.unwrap();
        inc += 1;
    }
    
    let mut window: RenderWindow = RenderWindow::new((640, 320),"Test", Style::CLOSE, &Default::default());
    let mut image: Image = Image::new(640 as u32, 320 as u32);
    let mut texture = Texture::new().unwrap();
    texture.create(64, 32);

    let mut renderer: Renderer = Renderer {
        window: &mut window,
        image: &mut image,
        cpu: &mut cpu,
        window_size_x: 640,
        window_size_y: 320
    };
    while (renderer.window.is_open()) {
        let mut event: Event;
        while let Some(event) = renderer.window.poll_event() {
            match event {
                Event::Closed => renderer.window.close(),
                _ => { }
            }
            
        }
        renderer.cpu.execute();

        if renderer.cpu.redraw {
            renderer.draw_buffer();
            renderer.cpu.redraw = false;
        }
        let image: Image = renderer.image.clone();
        texture.load_from_image(&image, IntRect::new(0, 0, 1000, 1000));
        let mut sprite: Sprite = Sprite::with_texture(&texture);
        sprite.set_scale((10.0, 10.0));
        renderer.window.clear(Color::BLACK);
        renderer.window.draw(&sprite);
        renderer.window.display();
        
    }
}
