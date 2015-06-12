
//-----------------------------

use sprite::*;
use button_controller::ButtonController;
use piston::event::*;
use piston::input::{ Button, Key };
use opengl_graphics::*;
use std::rc::Rc;
use std::path::Path;

//-----------------------------

pub struct App {
    window_size: (f64, f64),
    gl: GlGraphics,
    input_manager: ButtonController,
    player: Sprite<Texture>,
}

impl App {
    pub fn new(gl_version: OpenGL, window_size: (f64, f64)) -> App {
        let tex = Path::new("./assets/Player.png");
        let tex = Rc::new(Texture::from_path(&tex).unwrap());
        let mut sprite = Sprite::from_texture(tex.clone());
        sprite.set_position(window_size.0 / 2.0, window_size.1 / 2.0);
        App {
            window_size: window_size,
            gl: GlGraphics::new(gl_version),
            input_manager: ButtonController::new(),
            player: sprite
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        //the classic, courtesy XNA
        const CORNFLOWER_BLUE: [f32; 4] = [0.391, 0.584, 0.929, 1.0];

        let sprite = &self.player;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(CORNFLOWER_BLUE, gl);

            let transform = c.transform.trans(0.0, 0.0);
            sprite.draw(transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {

        //remaining as an example of how to use button controller for myself when i go to use it
        if self.input_manager.pressed_state(&Button::Keyboard(Key::A), false, true) {
        }
        else if self.input_manager.pressed_state(&Button::Keyboard(Key::S), false, true) {
        }

        self.input_manager.update();
    }

    pub fn handle_press(&mut self, button : &Button) {
        self.input_manager.register_press(button);
    }

    pub fn handle_release(&mut self, button : &Button) {
        self.input_manager.register_release(button);
    }
}
