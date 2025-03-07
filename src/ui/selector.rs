use super::{app::App, image_ctx::ImageCtx, DrawCtx};
use crate::{
    state::{FluidId, GroupRow, Preset, RecipeId},
    utils::{Point2, Vec2},
    ItemId,
};
use femtovg::Paint;

pub struct Selector {
    pub tabs: Vec<SelectorTab>,
    // pub active_tab: usize,
    // pub hover_idx: Option<SelectorHover>,
    // pub scroll_offset: Vec2,
    pub size: Vec2,
}
impl Selector {
    const TAB_HEIGHT: f32 = 50.0;
    const TAB_WIDTH: f32 = 50.0;
    const ROW_HEIGHT: f32 = 50.0;
    // const ITEM_HEIGHT: f32 = 50.0;
    const ITEM_WIDTH: f32 = 50.0;

    pub(crate) fn try_click(self, _app: &mut App) {}

    pub(crate) fn draw(&self, ctx: &mut DrawCtx, image_ctx: &mut ImageCtx) {
        let position = ctx.top_left_of_window();

        let DrawCtx { theme, canvas, .. } = ctx;

        position
            .with_size(self.size)
            .draw_fill(canvas, &Paint::color(theme.layer_color(2)));
        position
            .with_size(Vec2::new(self.size.x, Self::TAB_HEIGHT))
            .draw_fill(canvas, &Paint::color(theme.layer_color(3)));
        for (idx, tab) in self.tabs.iter().enumerate() {
            let tab_position = position + Vec2::new(idx as f32 * Self::TAB_WIDTH, 0.0);
            let rect = tab_position
                .with_size(Vec2::new(Self::TAB_WIDTH, Self::TAB_HEIGHT))
                .shrink(1.);
            image_ctx.draw(canvas, &tab.icon, rect);
        }

        // let row_position = position + Vec2::new(0.0, Self::TAB_HEIGHT);
        // for (row_idx, row) in self.tabs[self.active_tab].rows.iter().enumerate() {
        //     let row_position = row_position + Vec2::new(0.0, row_idx as f32 * Self::ROW_HEIGHT);
        //     for (item_idx, item) in row.items.iter().enumerate() {
        //         let item_position =
        //             row_position + Vec2::new(Self::ITEM_WIDTH * item_idx as f32, 0.0);
        //         let rect = item_position.with_size((Self::ITEM_WIDTH, Self::ITEM_HEIGHT).into());
        //         rect.draw_fill(canvas, &Paint::color(theme.layer_color(1)));

        //         image_ctx.draw(canvas, &item.icon, rect);
        //     }
        // }
    }

    pub(crate) fn mouse_move(&mut self, _mouse: Point2) -> bool {
        false
    }

    fn new<'a, F, R>(preset: &'a Preset, mapper: F) -> Self
    where
        F: Fn(&'a GroupRow) -> R,
        R: Iterator<Item = SelectorItem> + 'a,
    {
        let mut tabs: Vec<SelectorTab> = preset
            .groups
            .iter()
            .map(|g| SelectorTab {
                name: g.name.clone(),
                icon: preset.icon_for_tab_group(g),
                rows: g
                    .rows
                    .iter()
                    .map(&mapper)
                    .map(|iter| SelectorRow {
                        items: iter.collect(),
                    })
                    .collect(),
            })
            .collect();

        for tab in &mut tabs {
            tab.rows.retain(|row| !row.items.is_empty());
        }
        // tabs.retain(|tab| !tab.rows.is_empty());

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
                (max_height + 1) as f32 * Self::ROW_HEIGHT,
            ),
            // active_tab: 0,
            // hover_idx: None,
            // scroll_offset: Vec2::ZERO,
        };

        result
    }

    pub fn new_recipe(
        preset: &Preset,
        onclick: impl Fn(RecipeId, &mut App) + Clone + 'static,
    ) -> Self {
        Self::new(preset, move |row| {
            let onclick = onclick.clone();
            row.recipes.iter().copied().map(move |recipe_id| {
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
            row.items.iter().copied().map(move |item_id| {
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
            row.fluids.iter().copied().map(move |fluid_id| {
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

// pub struct SelectorHover {
//     pub tab_idx: usize,
//     pub row_idx: usize,
//     pub item_idx: usize,
// }

pub struct SelectorTab {
    #[allow(dead_code)]
    pub name: String,
    pub icon: String,
    pub rows: Vec<SelectorRow>,
}

pub struct SelectorRow {
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
