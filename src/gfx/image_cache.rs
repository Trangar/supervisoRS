use rustc_hash::{FxHashMap, FxHashSet};
use sdl3::{
    render::{Texture, TextureCreator},
    sys::pixels::SDL_PixelFormat,
    video::WindowContext,
};

pub struct ImageCache {
    pub texture_creator: &'static TextureCreator<WindowContext>,
    pub textures: FxHashMap<String, Texture<'static>>,
    pub textures_to_load: FxHashSet<String>,
}

impl ImageCache {
    pub(crate) fn init(texture_creator: &'static TextureCreator<WindowContext>) -> ImageCache {
        Self {
            texture_creator,
            textures: FxHashMap::default(),
            textures_to_load: FxHashSet::default(),
        }
    }

    pub fn load_percent(&self) -> f32 {
        if self.textures_to_load.is_empty() {
            return 1.0;
        }
        let total = (self.textures.len() + self.textures_to_load.len()) as f32;
        let loaded = self.textures.len() as f32;
        loaded / total
    }

    pub fn preload(&mut self) {
        if self.textures_to_load.is_empty() {
            return;
        }
        let path = self.textures_to_load.iter().next().unwrap().clone();
        self.textures_to_load.remove(&path);
        self.get_texture(&path);
    }

    pub fn get_texture(&mut self, path: &str) -> &Texture {
        self.textures_to_load.remove(path);
        self.textures.entry(path.to_string()).or_insert_with(|| {
            let mut png = png::Decoder::new(std::fs::File::open(path).unwrap())
                .read_info()
                .unwrap();
            let mut buf = vec![0; png.output_buffer_size()];
            png.next_frame(&mut buf).unwrap();

            let surface = sdl3::surface::Surface::from_data(
                &mut buf,
                png.info().width,
                png.info().height,
                png.info().raw_row_length() as u32,
                SDL_PixelFormat::RGBA32.try_into().unwrap(),
            )
            .unwrap();
            self.texture_creator
                .create_texture_from_surface(&surface)
                .unwrap()
        })
    }
}
