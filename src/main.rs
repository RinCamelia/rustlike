
//-----------------------------

extern crate sprite;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate button_controller;

//-----------------------------

mod app;

//-----------------------------

use app::*;
use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;

//-----------------------------

fn main() {
    let opengl = OpenGL::_3_2;

    let window = Window::new(
        WindowSettings::new(
            "spinning-square",
            [200, 200]
        )
        .exit_on_esc(true)
    );

    let mut app = App::new(opengl, (200.0, 200.0));

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
