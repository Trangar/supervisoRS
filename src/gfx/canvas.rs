use super::{Color, Paint};
use crate::utils::Point2;

pub struct Canvas<'a> {
    pub rect: sdl3::rect::Rect,
    pub window_size: Point2,
    pub canvas: &'a mut sdl3::render::Canvas<sdl3::video::Window>,
}
impl Canvas<'_> {
    pub(crate) fn clear_color(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    pub fn translate(&mut self, by: Point2) {
        self.rect.set_x(self.rect.x() + by.x as i32);
    }

    pub fn scale(&mut self, zoom: Point2) {}

    pub fn ui_to_world(&self, ui: Point2) -> Point2 {
        ui
    }

    pub fn draw_rounded(
        &mut self,
        rectangle: crate::utils::Rectangle,
        bg_paint: Paint,
        border_paint: Paint,
        radius: f64,
    ) {
        self.canvas.set_draw_color(bg_paint.color);
        self.canvas
            .fill_rect(sdl3::rect::Rect::new(
                rectangle.x as i32,
                rectangle.y as i32,
                rectangle.width as u32,
                rectangle.height as u32,
            ))
            .unwrap();
        self.canvas.set_draw_color(border_paint.color);
        self.canvas
            .draw_rect(sdl3::render::FRect::new(
                rectangle.x,
                rectangle.y,
                rectangle.width,
                rectangle.height,
            ))
            .unwrap();
    }
}
