
//-----------------------------

use texture_cache::TextureCache;
use button_controller::ButtonController;
use piston::event::*;
use piston::input::{ Button, Key };
use opengl_graphics::*;
use graphics::Image;
use tile_map::*;

//-----------------------------

pub struct App {
    window_size: (f64, f64),
    gl: GlGraphics,
    input_manager: ButtonController,
    texture_cache: TextureCache,
    player: Image,
    src_tile: usize,
}

impl App {
    pub fn new(gl_version: OpenGL, window_size: (f64, f64)) -> App {

        let tex_cache = TextureCache::new(String::from("./assets"));
        let player_image = Image::new().rect([0.0, 0.0, 16.0, 16.0]).src_rect([0, 0, 16, 16]);

        App {
            window_size: window_size,
            gl: GlGraphics::new(gl_version),
            input_manager: ButtonController::new(),
            texture_cache: tex_cache,
            player: player_image,
            src_tile: 0,
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {

        if self.input_manager.pressed_state(&Button::Keyboard(Key::A), false, true) {
            if self.src_tile > 0 { self.src_tile -= 1; }
            self.player = self.player.src_rect([self.src_tile as i32 * 16, 0, 16, 16]);
        }
        else if self.input_manager.pressed_state(&Button::Keyboard(Key::S), false, true) {
            if self.src_tile < 3 { self.src_tile += 1; }
            self.player = self.player.src_rect([self.src_tile as i32 * 16, 0, 16, 16]);
        }


        self.input_manager.update();
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        //the classic, courtesy XNA
        const CORNFLOWER_BLUE: [f32; 4] = [0.391, 0.584, 0.929, 1.0];
        let player = &self.player;
        let player_texture = self.texture_cache.get_asset(&String::from("Tileset 16x.png"));

        let draw_location_x = self.window_size.0 / 2.0;
        let draw_location_y = self.window_size.1 / 2.0;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(CORNFLOWER_BLUE, gl);
            let transform = c.transform.trans(draw_location_x, draw_location_y);
            player.draw(player_texture, default_draw_state(), transform, gl);
        });
    }

    pub fn handle_press(&mut self, button : &Button) {
        self.input_manager.register_press(button);
    }

    pub fn handle_release(&mut self, button : &Button) {
        self.input_manager.register_release(button);
    }
}
