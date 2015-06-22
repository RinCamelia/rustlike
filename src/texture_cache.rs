use std::path::{Path, PathBuf};
use std::collections::HashMap;
use opengl_graphics::Texture;

pub struct TextureCache {
    cache : HashMap<String, Texture>,
    base_path : PathBuf
}

impl TextureCache {
    pub fn new(path : String) -> TextureCache {
        TextureCache {
            cache : HashMap::new(),
            base_path : Path::new(&path).to_path_buf()
        }
    }

    pub fn get_asset(&mut self, key : &String) -> &Texture {
        if !self.cache.contains_key(key) {
            self.load_asset(key);
        }
        self.cache.get(key).unwrap()
    }

    fn load_asset(&mut self, key : &String) {
        let full_path = self.base_path.join(key);
        let texture = Texture::from_path(full_path);

        match texture {
            Ok(res) => {
                self.cache.insert(key.clone(), res);
            },
            Err(err) => panic!("attempted to load texture {:?} from path {:?}, got error {:?}", key, self.base_path.to_str(), err)
        }
    }
}
