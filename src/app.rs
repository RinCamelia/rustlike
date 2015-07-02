
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
    tile_map: TileMap,
}

impl App {
    pub fn new(gl_version: OpenGL, window_size: (f64, f64)) -> App {

        let mut tex_cache = TextureCache::new(String::from("./assets"));
        let player_image = Image::new().rect([0.0, 0.0, 16.0, 16.0]).src_rect([0, 0, 16, 16]);
        let tile_map = TileMap::with_default(String::from("Tileset 16x.png"), [16, 16], [16, 16]);
        tex_cache.load_asset(&String::from("Tileset 16x.png"));
        App {
            window_size: window_size,
            gl: GlGraphics::new(gl_version),
            input_manager: ButtonController::new(),
            texture_cache: tex_cache,
            player: player_image,
            src_tile: 0,
            tile_map: tile_map
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {

        if self.input_manager.pressed_state(&Button::Keyboard(Key::A), false, true) {
            //if self.src_tile > 0 { self.src_tile -= 1; }
            //self.player = self.player.src_rect([self.src_tile as i32 * 16, 0, 16, 16]);
            self.tile_map.set_tile_state([3,5], TileState::Wall);
        }
        else if self.input_manager.pressed_state(&Button::Keyboard(Key::S), false, true) {
            if self.src_tile < 3 { self.src_tile += 1; }
            self.player = self.player.src_rect([self.src_tile as i32 * 16, 0, 16, 16]);
            self.tile_map.set_tile_state([5,3], TileState::Void);
        }


        self.input_manager.update();
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        {
            //the classic, courtesy XNA
            const CORNFLOWER_BLUE: [f32; 4] = [0.391, 0.584, 0.929, 1.0];
            let player = &self.player;
            let player_texture = self.texture_cache.get_asset(&String::from("Tileset 16x.png"));

            let draw_location_x = self.window_size.0 / 2.0;
            let draw_location_y = self.window_size.1 / 2.0;

            self.gl.draw(args.viewport(), |c, gl| {
                clear(CORNFLOWER_BLUE, gl);
                //let transform = c.transform.trans(draw_location_x, draw_location_y);
                //player.draw(player_texture, default_draw_state(), transform, gl);
                });
        }
        self.tile_map.render(&mut self.gl, args, &mut self.texture_cache);

    }

    pub fn handle_press(&mut self, button : &Button) {
        self.input_manager.register_press(button);
    }

    pub fn handle_release(&mut self, button : &Button) {
        self.input_manager.register_release(button);
    }
}
