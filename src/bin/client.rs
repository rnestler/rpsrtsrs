extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;
extern crate rpsrtsrs;
#[cfg(feature = "include_sdl2")] extern crate sdl2_window;
#[cfg(feature = "include_glfw")] extern crate glfw_window;
#[cfg(feature = "include_glutin")] extern crate glutin_window;


use std::f64::consts::PI;
use piston::window::WindowSettings;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::input::*;
use piston::event_loop::*;
#[cfg(feature = "include_sdl2")] use sdl2_window::Sdl2Window as Window;
#[cfg(feature = "include_glfw")] use glfw_window::GlfwWindow as Window;
#[cfg(feature = "include_glutin")] use glutin_window::GlutinWindow as Window;
use rpsrtsrs::shapes::Unit;
use rpsrtsrs::client::*;


fn main() {
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window : Window = WindowSettings::new(
        "rpsrtsrs",
        [640, 480]
    ).exit_on_esc(true).samples(8).build().unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        units: vec![],
    };
    for _ in 0..10 {
        // Create new unit in random location
        let x = rand::random::<f64>() * 600.0 + 40.0;
        let y = rand::random::<f64>() * 440.0 + 40.0;
        let r = (rand::random::<f64>() - 0.5) * PI;
        let unit = Unit::new([x,y], r);

        // Register unit
        app.units.push(unit);
    }

    let mut cursor = [0.0,0.0];

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(Button::Mouse(button)) = e.press_args() {
            match button {
                MouseButton::Left  => app.select(cursor),
                MouseButton::Right => app.move_selected(cursor),
                _ => println!("Pressed mouse button '{:?}'", button),
            }
        }
        e.mouse_cursor(|x, y| {
            cursor = [x, y];
        });

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
