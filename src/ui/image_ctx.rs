use femtovg::{ImageFlags, ImageId, Paint, Path};
use rustc_hash::FxHashMap;

use crate::utils::Rectangle;

use super::Canvas;

#[derive(Default)]
pub struct ImageCtx {
    image_path_to_id: FxHashMap<String, ImageInfo>,
}

#[derive(Copy, Clone)]
struct ImageInfo {
    id: ImageId,
    width: f32,
    height: f32,
}

impl ImageCtx {
    pub fn draw(&mut self, canvas: &mut Canvas, image_path: &str, rect: Rectangle) {
        let image = match self.image_path_to_id.get(image_path) {
            Some(image) => image,
            None => {
                let mut info = ImageInfo {
                    id: canvas
                        .load_image_file(image_path, ImageFlags::NEAREST)
                        .unwrap(),
                    width: 0.0,
                    height: 0.0,
                };
                let i_info = canvas.image_info(info.id).unwrap();
                info.width = i_info.width() as f32;
                info.height = i_info.height() as f32;
                self.image_path_to_id.insert(image_path.to_string(), info);
                &self.image_path_to_id[image_path]
            }
        };
        let mut path = Path::new();
        path.rect(rect.x, rect.y, rect.width, rect.height);
        let paint = Paint::image(image.id, 0., 0., rect.width, rect.height, 0.0, 1.0);

        canvas.fill_path(&path, &paint);
    }
}
