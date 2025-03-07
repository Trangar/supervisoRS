use super::{app::App, DrawCtx};
use crate::utils::Point2;
use femtovg::Paint;

pub struct ContextMenu {
    pub position: Point2,
    pub items: Vec<ContextMenuItem>,
    pub hover_idx: Option<usize>,
}
impl ContextMenu {
    const WIDTH: f32 = 200.0;
    const ITEM_HEIGHT: f32 = 30.0;

    pub(crate) fn new(
        position: Point2,
        window_size: Point2,
        items: impl IntoIterator<Item = ContextMenuItem>,
    ) -> Self {
        let items = items.into_iter().collect::<Vec<_>>();
        let expected_width = Self::WIDTH;
        let expected_height = Self::ITEM_HEIGHT * items.len() as f32;

        let position = if position.x + expected_width > window_size.x {
            Point2::new(
                window_size.x - expected_width,
                position.y.min(window_size.y - expected_height),
            )
        } else {
            Point2::new(position.x, position.y.min(window_size.y - expected_height))
        };

        Self {
            position,
            items: items.into_iter().collect(),
            hover_idx: None,
        }
    }

    pub fn mouse_move(&mut self, mouse: Point2) -> bool {
        if mouse.x < self.position.x
            || mouse.x > self.position.x + Self::WIDTH
            || mouse.y < self.position.y
            || mouse.y > self.position.y + self.items.len() as f32 * Self::ITEM_HEIGHT
        {
            self.hover_idx = None;
            return false;
        }

        let idx = ((mouse.y - self.position.y) / Self::ITEM_HEIGHT) as usize;
        if idx < self.items.len() {
            self.hover_idx = Some(idx);
            true
        } else {
            self.hover_idx = None;
            false
        }
    }

    pub fn draw(&self, ctx: &mut DrawCtx) {
        let DrawCtx { theme, canvas, .. } = ctx;

        let background = Paint::color(theme.layer_color(1));
        let border = Paint::color(theme.layer_color(2));
        let text = Paint::color(theme.layer_color(3)).with_font_size(20.);

        for (i, item) in self.items.iter().enumerate() {
            let y = self.position.y + i as f32 * Self::ITEM_HEIGHT;
            let mut path = femtovg::Path::new();
            path.rect(self.position.x, y, Self::WIDTH, Self::ITEM_HEIGHT);

            let tmp_paint_1: Paint;
            let tmp_paint_2: Paint;

            let background_color = if self.hover_idx == Some(i) {
                tmp_paint_1 = Paint::color(theme.layer_color(2));
                &tmp_paint_1
            } else {
                &background
            };

            let text_color = if self.hover_idx == Some(i) {
                tmp_paint_2 = Paint::color(theme.layer_color(3)).with_font_size(text.font_size());
                &tmp_paint_2
            } else {
                &text
            };

            canvas.fill_path(&mut path, background_color);
            canvas.stroke_path(&mut path, &border);

            canvas
                .fill_text(self.position.x + 10.0, y + 24.0, &item.label, text_color)
                .unwrap();
        }
    }

    pub(crate) fn try_click(mut self, app: &mut App) {
        if let Some(idx) = self.hover_idx {
            self.items.remove(idx).click(app);
        }
    }
}

pub struct ContextMenuItem {
    label: String,
    action: Box<dyn FnOnce(&mut App)>,
}

impl ContextMenuItem {
    pub fn new(label: impl Into<String>, action: impl FnOnce(&mut App) + 'static) -> Self {
        Self {
            label: label.into(),
            action: Box::new(action),
        }
    }

    pub fn click(self, app: &mut App) {
        (self.action)(app);
    }
}
