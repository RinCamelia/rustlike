
//-----------------------------

use texture_cache::TextureCache;
use button_controller::ButtonController;
use piston::event::*;
use piston::input::{ Button, Key };
use opengl_graphics::*;
use graphics::Image;

//-----------------------------

pub struct App {
    window_size: (f64, f64),
    gl: GlGraphics,
    input_manager: ButtonController,
    texture_cache: TextureCache,
    player: Image
}

impl App {
    pub fn new(gl_version: OpenGL, window_size: (f64, f64)) -> App {

        let mut tex_cache = TextureCache::new(String::from_str("./assets"));
        let player_image = Image::new()

        App {
            window_size: window_size,
            gl: GlGraphics::new(gl_version),
            input_manager: ButtonController::new(),
            texture_cache:
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        //the classic, courtesy XNA
        const CORNFLOWER_BLUE: [f32; 4] = [0.391, 0.584, 0.929, 1.0];


        self.gl.draw(args.viewport(), |c, gl| {
            clear(CORNFLOWER_BLUE, gl);
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
