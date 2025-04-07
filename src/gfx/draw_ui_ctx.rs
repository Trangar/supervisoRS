use super::{Canvas, Paint, font_cache::FontCache, image_cache::ImageCache};
use crate::utils::{Point2, Vec2};
use sdl3::render::FRect;

pub struct DrawUiCtx<'a, 'b> {
    pub canvas: &'a mut Canvas<'b>,
    pub font_cache: &'a mut FontCache,
    pub image_cache: &'a mut ImageCache,
    pub ui_mouse: Point2,
    pub world_mouse: Point2,
    pub window_size: Point2,
}
impl DrawUiCtx<'_, '_> {
    pub(crate) fn fill_text(
        &mut self,
        x: f32,
        y: f32,
        label: &str,
        paint: Paint,
    ) -> Result<(), ()> {
        self.canvas.canvas.set_draw_color(paint.color);

        let texture = self.font_cache.get_texture(label, paint);

        let target = FRect::new(x, y, texture.width() as f32, texture.height() as f32);
        self.canvas
            .canvas
            .copy(&texture, None, Some(target))
            .map_err(|_| ())?;

        Ok(())
    }

    pub(crate) fn draw_fill(&mut self, rect: crate::utils::Rectangle, paint: Paint) {
        self.canvas.canvas.set_draw_color(paint.color);
        self.canvas
            .canvas
            .fill_rect(sdl3::rect::Rect::new(
                rect.x as i32,
                rect.y as i32,
                rect.width as u32,
                rect.height as u32,
            ))
            .unwrap();
    }

    pub(crate) fn draw_image(&mut self, image_path: &str, rect: crate::utils::Rectangle) {
        let texture = self.image_cache.get_texture(image_path);
        let target = FRect::new(rect.x, rect.y, rect.width, rect.height);
        self.canvas
            .canvas
            .copy(texture, None, Some(target))
            .unwrap();
    }

    pub(crate) fn draw_progress_bar(
        &mut self,
        point: Point2,
        size: Vec2,
        percent: f32,
        layer_color_1: Paint,
        layer_color_2: Paint,
        label: &str,
        label_color: Paint,
    ) {
        let rect = crate::utils::Rectangle {
            x: point.x,
            y: point.y,
            width: size.x,
            height: size.y,
        };
        self.draw_fill(rect, layer_color_1);
        let progress_rect = crate::utils::Rectangle {
            x: point.x,
            y: point.y,
            width: size.x * percent,
            height: size.y,
        };
        self.draw_fill(progress_rect, layer_color_2);
        self.fill_text_centered(rect, label, label_color);
    }

    pub fn fill_text_centered(&mut self, rect: crate::utils::Rectangle, label: &str, paint: Paint) {
        let texture = self.font_cache.get_texture(label, paint);
        let target = FRect::new(
            rect.x + (rect.width - texture.width() as f32) / 2.0,
            rect.y + (rect.height - texture.height() as f32) / 2.0,
            texture.width() as f32,
            texture.height() as f32,
        );
        self.canvas
            .canvas
            .copy(&texture, None, Some(target))
            .unwrap();
    }

    pub(crate) fn draw_fill_border(
        &mut self,
        rect: crate::utils::Rectangle,
        background_color: Paint,
        border: Paint,
    ) {
        self.draw_fill(rect, background_color);
        self.canvas.canvas.set_draw_color(border.color);
        self.canvas.canvas.draw_rect(rect.into()).unwrap();
    }
}
