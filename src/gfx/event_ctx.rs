use super::Canvas;
use crate::utils::{Point2, Vec2};

pub struct EventCtx<'a, 'b> {
    pub canvas: &'a mut Canvas<'b>,
    pub ui_mouse: Point2,
    pub world_mouse: Point2,
    pub window_size: Point2,

    pub redraw: bool,

    pub running: &'a mut bool,
}

impl EventCtx<'_, '_> {
    pub fn exit(&mut self) {
        *self.running = false;
    }

    pub fn zoom_at_mouse(&mut self, zoom: f32) {
        let pt = self.world_mouse;
        self.canvas.translate(pt);
        self.canvas.scale(Point2::spread(1.0 + (zoom / 10.0)));
        self.canvas.translate(-pt);
        self.redraw = true;
    }

    pub fn translate_by(&mut self, relative: Vec2) {
        self.canvas.translate(relative.into());
        self.redraw = true;
    }

    pub fn redraw(&mut self) {
        self.redraw = true;
    }
}
