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
    // pub fn image_id(&mut self, canvas: &mut Canvas, image_path: &str) -> ImageId {
    //     self.image_info(canvas, image_path).id
    // }

    pub fn draw(&mut self, canvas: &mut Canvas, image_path: &str, rect: Rectangle) {
        let image = self.image_info(canvas, image_path);
        let mut path = Path::new();
        path.rect(rect.x, rect.y, rect.width, rect.height);
        let paint = Paint::image(image.id, rect.x, rect.y, rect.width, rect.height, 0.0, 1.0);

        canvas.fill_path(&path, &paint);
    }

    fn image_info<'a>(&'a mut self, canvas: &mut Canvas, image_path: &str) -> &'a ImageInfo {
        self.image_path_to_id
            .entry(image_path.to_owned())
            .or_insert_with(|| {
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
                info
            })
    }
}
