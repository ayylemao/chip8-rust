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
use sfml::system::Clock;
use crate::renderer::Renderer;

// Assuming CPU is defined in another file
mod cpu; // Import the CPU module
use crate::cpu::CPU;

fn main() {
    let display_size_x: u32 = 640;
    let display_size_y: u32 = 320;

    let mut clock = Clock::start();

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
    
    let mut window: RenderWindow = RenderWindow::new((display_size_x, display_size_y),"Test", Style::CLOSE, &Default::default());
    let mut image: Image = Image::new(display_size_x as u32, display_size_y as u32);
    let mut texture = Texture::new().unwrap();
    texture.create(64, 32);

    let mut renderer: Renderer = Renderer {
        window: &mut window,
        image: &mut image,
        cpu: &mut cpu,
        window_size_x: display_size_x,
        window_size_y: display_size_y
    };
    while (renderer.window.is_open()) {
        let mut event: Event;
        while let Some(event) = renderer.window.poll_event() {
            match event {
                Event::Closed => renderer.window.close(),
                _ => { }
            }
            
        }
        renderer.window.clear(Color::BLACK);
        if clock.elapsed_time().as_seconds() >= 0.00185 {
            clock.restart();
            renderer.cpu.execute();
        }

        if renderer.cpu.redraw {
            renderer.draw_buffer();
            renderer.cpu.redraw = false;
        }
        let image: Image = renderer.image.clone();
        texture.load_from_image(&image, IntRect::new(0, 0, 100, 100));
        let mut sprite: Sprite = Sprite::with_texture(&texture);
        sprite.set_scale((10.0, 10.0));
        renderer.window.clear(Color::BLACK);
        renderer.window.draw(&sprite);
        renderer.window.display();
        
    }
}
