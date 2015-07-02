//-----------------------------

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate button_controller;
extern crate vecmath;

//-----------------------------

mod app;
mod texture_cache;
mod tile_map;

//-----------------------------

use app::*;
use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;

static WINDOW_X_SIZE : usize = 256;
static WINDOW_Y_SIZE : usize = 256;

//-----------------------------

fn main() {
    let window = Window::new(
        WindowSettings::new(
            "rustlike",
            [WINDOW_X_SIZE as u32, WINDOW_Y_SIZE as u32]
        )
        .exit_on_esc(true)
    );

    let mut app = App::new(OpenGL::_3_2, (WINDOW_X_SIZE as f64, WINDOW_Y_SIZE as f64));

    for e in window.events() {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(i) = e.press_args() {
            app.handle_press(&i);
        }
        if let Some(i) = e.release_args() {
            app.handle_release(&i);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
