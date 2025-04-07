use super::{font_cache::FontCache, image_cache::ImageCache};

pub struct Window {
    pub sdl_context: sdl3::Sdl,
    pub video_subsystem: sdl3::VideoSubsystem,
    pub font_cache: FontCache,
    pub image_cache: ImageCache,
    pub canvas: sdl3::render::Canvas<sdl3::video::Window>,
}

impl Window {
    pub fn new() -> Self {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Node Editor", 800, 600)
            .position_centered()
            .high_pixel_density()
            .vulkan()
            .resizable()
            .build()
            .unwrap();

        let canvas = window.into_canvas();
        let font_cache = FontCache::init(Box::leak(Box::new(canvas.texture_creator())));
        let image_cache = ImageCache::init(Box::leak(Box::new(canvas.texture_creator())));
        Self {
            sdl_context,
            video_subsystem,
            font_cache,
            image_cache,
            canvas,
        }
    }
}
