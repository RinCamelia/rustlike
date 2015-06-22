
//-----------------------------

use texture_cache::TextureCache;
use button_controller::ButtonController;
use piston::event::*;
use piston::input::{ Button, Key };
use opengl_graphics::*;
use graphics::types::Rectangle;
use graphics::Rectangle::*;
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
        let player_image = Image::new().rect([0.0, 0.0, 16.0, 16.0]);

        App {
            window_size: window_size,
            gl: GlGraphics::new(gl_version),
            input_manager: ButtonController::new(),
            texture_cache: tex_cache,
            player: player_image,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        //the classic, courtesy XNA
        const CORNFLOWER_BLUE: [f32; 4] = [0.391, 0.584, 0.929, 1.0];
        let player = &self.player;
        let player_texture = self.texture_cache.get_asset(&String::from_str("Player.png"));


        self.gl.draw(args.viewport(), |c, gl| {
            clear(CORNFLOWER_BLUE, gl);
            let transform = c.transform.trans(100.0, 100.0);
            player.draw(player_texture, default_draw_state(), transform, gl);
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
