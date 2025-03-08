use super::{app::App, image_ctx::ImageCtx, DrawCtx, PopupClickResult};
use crate::{
    state::{FluidId, GroupRow, ItemId, Preset, RecipeId},
    utils::{Point2, Vec2},
};
use femtovg::Paint;

pub struct Selector {
    pub tabs: Vec<SelectorTab>,
    pub active_tab: usize,
    pub hover: Hover,
    pub scroll_offset: Vec2,
    pub size: Vec2,
}

impl Selector {
    const TAB_HEIGHT: f32 = 50.0;
    const TAB_WIDTH: f32 = 50.0;
    const PADDING: f32 = 5.0;
    const ITEM_HEIGHT: f32 = 30.0;
    const ITEM_WIDTH: f32 = 30.0;
    const TOP_LEFT_OFFSET: Vec2 = Vec2::new(50., 50.);

    pub(crate) fn try_click(&mut self, _app: &mut App) -> PopupClickResult {
        match self.hover {
            Hover::None => PopupClickResult::Close,
            Hover::Tab { tab_idx } => {
                self.active_tab = tab_idx;
                return PopupClickResult::None;
            }
            Hover::Item { row_idx, item_idx } => {
                let item = &self.tabs[self.active_tab].rows[row_idx].items[item_idx];
                (item.on_click)(_app);
                return PopupClickResult::Close;
            }
        }
    }

    pub(crate) fn draw(&self, ctx: &mut DrawCtx, image_ctx: &mut ImageCtx) {
        let position = ctx.top_left_of_window(Self::TOP_LEFT_OFFSET);

        let DrawCtx { theme, canvas, .. } = ctx;

        (position - Vec2::splat(Self::PADDING))
            .with_size(self.size + Vec2::splat(Self::PADDING) * 2.)
            .draw_fill(canvas, &Paint::color(theme.layer_color(2)));

        position
            .with_size(Vec2::new(self.size.x, Self::TAB_HEIGHT))
            .draw_fill(canvas, &Paint::color(theme.layer_color(3)));
        for (idx, tab) in self.tabs.iter().enumerate() {
            let tab_position = position + Vec2::new(idx as f32 * Self::TAB_WIDTH, 0.0);
            let rect = tab_position.with_size(Vec2::new(Self::TAB_WIDTH, Self::TAB_HEIGHT));

            if idx == self.active_tab || self.hover.is_tab(idx) {
                rect.draw_fill(canvas, &Paint::color(theme.layer_color(4)));
            }

            image_ctx.draw(canvas, &tab.icon, rect.shrink(1.));
        }

        let row_position = position + Vec2::new(0.0, Self::TAB_HEIGHT);
        for (row_idx, row) in self.tabs[self.active_tab].rows.iter().enumerate() {
            let row_position = row_position + Vec2::new(0.0, row_idx as f32 * Self::ITEM_HEIGHT);
            for (item_idx, item) in row.items.iter().enumerate() {
                let item_position =
                    row_position + Vec2::new(Self::ITEM_WIDTH * item_idx as f32, 0.0);
                let rect = item_position.with_size((Self::ITEM_WIDTH, Self::ITEM_HEIGHT).into());

                image_ctx.draw(canvas, &item.icon, rect.shrink(1.));
            }
        }
    }

    pub(crate) fn mouse_move(&mut self, mut mouse: Point2) -> bool {
        mouse -= Self::TOP_LEFT_OFFSET;

        if mouse.y < 0. || mouse.x < 0. {
            return false;
        }
        if mouse.y < Self::TAB_HEIGHT {
            let idx = (mouse.x / Self::TAB_WIDTH).floor();
            if idx >= 0. && idx < self.tabs.len() as f32 {
                self.hover = Hover::Tab {
                    tab_idx: idx as usize,
                };
            }
            true
        } else {
            // self.hover_idx = None;
            false
        }
    }

    fn new<'a, F, R>(preset: &'a Preset, mapper: F) -> Self
    where
        F: Fn(&'a GroupRow) -> R,
        R: Iterator<Item = SelectorItem> + 'a,
    {
        let tabs: Vec<SelectorTab> = preset
            .groups
            .iter()
            .map(|g| SelectorTab {
                name: g.name.clone(),
                icon: preset.icon_for_tab_group(g),
                rows: g
                    .rows
                    .iter()
                    .map(|row| (row, mapper(row)))
                    .filter_map(|(row, iter)| {
                        let row = SelectorRow {
                            name: row.name.clone(),
                            items: iter.collect(),
                        };
                        if row.items.is_empty() {
                            None
                        } else {
                            Some(row)
                        }
                    })
                    .collect(),
            })
            .filter(|g| g.rows.len() > 0)
            .collect();

        let mut max_width = tabs.len();
        let mut max_height = 0;
        for tab in &tabs {
            max_height = max_height.max(tab.rows.len());
            for row in &tab.rows {
                max_width = max_width.max(row.items.len());
            }
        }

        let result = Self {
            tabs,
            size: Vec2::new(
                max_width as f32 * Self::ITEM_WIDTH,
                max_height as f32 * Self::ITEM_HEIGHT + Self::TAB_HEIGHT,
            ),
            active_tab: 0,
            hover: Hover::None,
            scroll_offset: Vec2::ZERO,
        };

        result
    }

    pub fn new_recipe(
        preset: &Preset,
        onclick: impl Fn(RecipeId, &mut App) + Clone + 'static,
    ) -> Self {
        Self::new(preset, move |row| {
            let onclick = onclick.clone();
            row.recipes.iter().cloned().map(move |recipe_id| {
                let recipe = &preset.recipes[&recipe_id];
                let onclick = onclick.clone();
                SelectorItem {
                    name: recipe.name.clone(),
                    icon: preset.icon_for_recipe(recipe),
                    on_click: Box::new(move |app| onclick(recipe_id, app)),
                }
            })
        })
    }
    pub fn new_item(preset: &Preset, onclick: impl Fn(ItemId, &mut App) + Clone + 'static) -> Self {
        Self::new(preset, move |row| {
            let onclick = onclick.clone();
            row.items.iter().cloned().map(move |item_id| {
                let item = &preset.items[&item_id];
                let onclick = onclick.clone();
                SelectorItem {
                    name: item.name.clone(),
                    icon: preset.icon_for_item(item),
                    on_click: Box::new(move |app| onclick(item_id, app)),
                }
            })
        })
    }

    pub fn new_fluid(
        preset: &Preset,
        onclick: impl Fn(FluidId, &mut App) + Clone + 'static,
    ) -> Self {
        Self::new(preset, move |row| {
            let onclick = onclick.clone();
            row.fluids.iter().cloned().map(move |fluid_id| {
                let fluid = &preset.fluids[&fluid_id];
                let onclick = onclick.clone();
                SelectorItem {
                    name: fluid.name.clone(),
                    icon: preset.icon_for_fluid(fluid),
                    on_click: Box::new(move |app| onclick(fluid_id, app)),
                }
            })
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hover {
    None,
    Tab { tab_idx: usize },
    Item { row_idx: usize, item_idx: usize },
}
impl Hover {
    fn is_tab(&self, idx: usize) -> bool {
        match self {
            Hover::Tab { tab_idx } => *tab_idx == idx,
            _ => false,
        }
    }
}

pub struct SelectorTab {
    #[allow(dead_code)]
    pub name: String,
    pub icon: String,
    pub rows: Vec<SelectorRow>,
}

pub struct SelectorRow {
    pub name: String,
    pub items: Vec<SelectorItem>,
}

pub struct SelectorItem {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub icon: String,
    #[allow(dead_code)]
    pub on_click: Box<dyn Fn(&mut App)>,
}

impl std::fmt::Debug for SelectorItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SelectorItem")
            .field("name", &self.name)
            .field("icon", &self.icon)
            .finish()
    }
}
