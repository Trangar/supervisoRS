use std::time::Instant;

use rustc_hash::FxHashMap;
use sdl3::{
    render::{Texture, TextureCreator},
    ttf::{Font, sys::TTF_HINTING_NORMAL},
    video::WindowContext,
};

use super::Paint;

pub struct FontCache {
    ctx: &'static sdl3::ttf::Sdl3TtfContext,
    font: FxHashMap<u16, Font<'static, 'static>>,
    pub texture_creator: &'static TextureCreator<WindowContext>,
    pub textures: FxHashMap<u64, (Texture<'static>, Instant)>,
}
impl FontCache {
    pub(crate) fn init(texture_creator: &'static TextureCreator<WindowContext>) -> FontCache {
        let ttf_context = Box::leak(Box::new(sdl3::ttf::init().unwrap()));
        Self {
            ctx: ttf_context,
            font: FxHashMap::default(),
            texture_creator,
            textures: FxHashMap::default(),
        }
    }
    pub fn get_texture(&mut self, text: &str, paint: Paint) -> &Texture {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::hash::DefaultHasher::new();
        text.hash(&mut hasher);
        paint.hash(&mut hasher);
        let hash = hasher.finish();

        let entry = self.textures.entry(hash).or_insert_with(|| {
            let font_size = paint.font_size.unwrap_or(14);
            let font = self.font.entry(font_size).or_insert_with(|| {
                let mut font = self
                    .ctx
                    .load_font("assets/Roboto-Regular.ttf", font_size as f32)
                    .unwrap();
                font.set_hinting(TTF_HINTING_NORMAL);
                font
            });
            let surface = font.render(text).blended(paint.color).unwrap();
            let texture = self
                .texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
            (texture, Instant::now())
        });

        entry.1 = Instant::now();
        &entry.0
    }

    pub fn cleanup(&mut self) {
        let now = Instant::now();
        self.textures
            .retain(|_, (_, time)| now.duration_since(*time).as_secs() < 5);
    }
}
