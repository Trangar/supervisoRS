use super::{Canvas, Color, Paint, font_cache::FontCache, image_cache::ImageCache};
use crate::utils::{Point2, Vec2};
use sdl3::render::FPoint;

pub struct DrawWorldCtx<'a, 'b> {
    pub canvas: &'a mut Canvas<'b>,
    pub font_cache: &'a mut FontCache,
    pub image_cache: &'a mut ImageCache,
    pub ui_mouse: Point2,
    pub world_mouse: Point2,
    pub window_size: Point2,
}
impl DrawWorldCtx<'_, '_> {
    pub(crate) fn draw_rounded(
        &mut self,
        rectangle: crate::utils::Rectangle,
        bg_paint: Paint,
        border_paint: Paint,
        radius: f64,
    ) {
        self.canvas
            .draw_rounded(rectangle, bg_paint, border_paint, radius);
    }

    pub(crate) fn clear_color(&mut self, color: Color) {
        self.canvas.clear_color(color);
    }

    pub(crate) fn draw_bezier(
        &mut self,
        line_color: &Paint,
        from: Point2,
        bezier_curve_factor_1: Vec2,
        to: Point2,
        bezier_curve_factor_2: Option<Vec2>,
    ) {
        self.canvas.canvas.set_draw_color(line_color.color);
        let mut lines: [FPoint; 4] = [
            from.into(),
            (from + bezier_curve_factor_1).into(),
            to.into(),
            to.into(),
        ];
        let mut n = 3;
        if let Some(bezier_curve_factor_2) = bezier_curve_factor_2 {
            n = 4;
            lines[2] = (to + bezier_curve_factor_2).into();
        }
        self.canvas.canvas.draw_lines(&lines[..n]).unwrap();
    }
}
