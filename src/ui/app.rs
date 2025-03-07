use super::{
    context_menu::{ContextMenu, ContextMenuItem},
    drag::{Drag, DragState},
    hover::Hover,
    image_ctx::ImageCtx,
    selector::Selector,
    utils::{draw_bezier, get_node_socket_position},
    Canvas, DrawCtx, EventCtx, PopupClickResult,
};
use crate::{
    state::{Preset, Theme},
    utils::{self, Point2, Rectangle, Vec2},
    Connection, Node, NodeId, SocketPos, BEZIER_CURVE_FACTOR,
};
use femtovg::Paint;
use rustc_hash::FxHashMap;

pub struct App {
    #[allow(dead_code)]
    preset: Preset,
    nodes: FxHashMap<NodeId, Node>,
    connections: Vec<Connection>,
    theme: Theme,
    hover: Hover,
    dragging: Drag,

    image_ctx: ImageCtx,
    context_menu: Option<ContextMenu>,
    selector: Option<Selector>,
}

impl App {
    pub fn new(preset: Preset) -> Self {
        Self {
            preset,
            nodes: utils::demo_nodes(),
            connections: Vec::new(),
            theme: Theme::default(),
            hover: Hover::None,
            dragging: Drag::default(),
            image_ctx: ImageCtx::default(),
            context_menu: None,
            selector: None,
        }
    }

    fn click_background(&self, ctx: &EventCtx) {
        println!("TODO click background at {:?}", ctx.mouse);
    }

    fn click_node(&self, ctx: &EventCtx, node_id: NodeId) {
        println!("TODO click node {:?} at {:?}", node_id, ctx.mouse);
    }

    fn click_node_socket(&mut self, ctx: &EventCtx, pos: SocketPos) {
        if let Some(idx) = self.connections.iter().position(|c| c.has_socket(pos)) {
            self.connections.remove(idx);
        } else {
            println!(
                "TODO click node {:?} socket {:?} at {:?}",
                pos.node_id,
                self.nodes[&pos.node_id].get_socket(pos.input, pos.socket_index),
                ctx.mouse
            );
        };
    }

    fn drop_node_socket(&mut self, ctx: &EventCtx, pos: SocketPos) {
        println!(
            "TODO drop node {:?} socket {} at {:?}",
            pos.node_id, pos.socket_index, ctx.mouse
        );
        println!("Hover is {:?}", self.hover);
        if let Hover::NodeSocket {
            node,
            socket,
            input: hover_is_input,
        } = self.hover
        {
            if node != pos.node_id {
                let src = pos;
                let dst = (node, socket, hover_is_input).into();

                if let Some(idx) = self
                    .connections
                    .iter()
                    .position(|c| c.has_socket(pos) && c.has_socket(src))
                {
                    self.connections.remove(idx);
                } else {
                    self.connections.push(Connection { src, dst })
                }
            }
        }
    }

    fn end_node_translate(&self, _ctx: &mut EventCtx, node_id: NodeId) {
        println!("TODO end node translate {:?}", node_id);
    }

    fn end_background_translate(&self, _ctx: &mut EventCtx<'_>) {
        println!("TODO end background translate");
    }

    fn set_right_click_menu(
        &mut self,
        ctx: &mut EventCtx,
        items: impl IntoIterator<Item = ContextMenuItem>,
    ) {
        self.context_menu = Some(ContextMenu::new(ctx.mouse, ctx.window_size, items));
        ctx.redraw();
    }

    fn open_recipe_selector(&mut self, pos: Point2) {
        self.selector = Some(Selector::new_recipe(&self.preset, move |recipe_id, app| {
            println!(
                "TODO add recipe {:?} at {pos:?}",
                app.preset.recipes[&recipe_id]
            );
        }));
    }

    fn open_item_selector(&mut self, pos: Point2) {
        self.selector = Some(Selector::new_item(&self.preset, move |item_id, app| {
            println!("TODO add item {:?} at {pos:?}", app.preset.items[&item_id]);
        }));
    }
    fn open_fluid_selector(&mut self, pos: Point2) {
        self.selector = Some(Selector::new_fluid(&self.preset, move |fluid_id, app| {
            println!(
                "TODO add fluid {:?} at {pos:?}",
                app.preset.fluids[&fluid_id]
            );
        }));
    }
}

impl super::App for App {
    fn draw(&mut self, canvas: &mut Canvas, mouse: Point2, window_size: Point2) {
        canvas.clear_rect(
            0,
            0,
            canvas.width(),
            canvas.height(),
            self.theme.background.color,
        );

        let line_color = Paint::color(self.theme.layer_color(3)).with_line_width(5.);

        for connection in &self.connections {
            let src_node = &self.nodes[&connection.src.node_id];
            let dst_node = &self.nodes[&connection.dst.node_id];
            let src = get_node_socket_position(
                src_node,
                connection.src.socket_index,
                connection.src.input,
            );
            let dst = get_node_socket_position(
                dst_node,
                connection.dst.socket_index,
                connection.dst.input,
            );
            let from = src.center();
            let to = dst.center();
            draw_bezier(
                canvas,
                &line_color,
                from,
                src_node.direction.line_direction(
                    connection.src.input,
                    from.distance(to) * BEZIER_CURVE_FACTOR,
                ),
                to,
                Some(dst_node.direction.line_direction(
                    connection.dst.input,
                    from.distance(to) * BEZIER_CURVE_FACTOR,
                )),
            );
        }
        let mut ctx = DrawCtx {
            canvas,
            theme: &self.theme,
            mouse,
            window_size,
        };

        for node in self.nodes.values() {
            draw_node(
                &mut ctx,
                node,
                self.hover.should_highlight_node(node) || self.dragging.should_highlight_node(node),
                self.hover
                    .get_highlight_socket(node)
                    .or_else(|| self.dragging.get_highlight_socket(node)),
            );
        }

        let mouse = ctx.mouse;
        self.dragging
            .draw_line(&self.nodes, &mut ctx, &line_color, mouse);
        if let Some(context_menu) = &self.context_menu {
            context_menu.draw(&mut ctx);
        }
        if let Some(selector) = &self.selector {
            selector.draw(&mut ctx, &mut self.image_ctx);
        }
    }

    fn mouse_down(&mut self, _ctx: &mut EventCtx, button: winit::event::MouseButton) {
        if button == winit::event::MouseButton::Left {
            if self.selector.is_some() || self.context_menu.is_some() {
                return;
            }

            if let Hover::NodeSocket {
                node,
                socket,
                input,
            } = self.hover
            {
                self.dragging.start_drag_line(
                    _ctx.mouse,
                    (node, socket, input).into(),
                    get_socket_initial_direction(&self.nodes, self.hover),
                );
            } else if let Hover::Node(node_id) = self.hover {
                self.dragging.start_drag_node(_ctx.mouse, node_id);
            } else {
                self.dragging.start_drag_background(_ctx.mouse);
            }
        }
    }

    fn mouse_up(&mut self, ctx: &mut EventCtx, button: winit::event::MouseButton) {
        if let Some(menu) = std::mem::take(&mut self.context_menu) {
            menu.try_click(self);
            self.dragging.clear();
            ctx.redraw();
            return;
        }
        if let Some(mut selector) = std::mem::take(&mut self.selector) {
            if selector.try_click(self) != PopupClickResult::Close {
                // If the selector is still open, put it back
                self.selector = Some(selector);
            }
            self.dragging.clear();
            ctx.redraw();
            return;
        }

        if button == winit::event::MouseButton::Left {
            match (self.dragging.mouse_up_was_click(), self.dragging.state()) {
                (true, DragState::LineFromNodeSocket { pos, .. }) => {
                    self.click_node_socket(ctx, *pos);
                }
                (false, DragState::LineFromNodeSocket { pos, .. }) => {
                    self.drop_node_socket(ctx, *pos);
                }
                (true, DragState::Node { node_id }) => {
                    self.click_node(ctx, *node_id);
                }
                (false, DragState::Node { node_id }) => {
                    self.end_node_translate(ctx, *node_id);
                }
                (true, DragState::Background) => {
                    self.click_background(ctx);
                }
                (false, DragState::Background) => {
                    self.end_background_translate(ctx);
                }

                _ => {}
            }

            self.dragging.clear();
            ctx.redraw();
        }
        if button == winit::event::MouseButton::Right {
            match self.hover {
                Hover::NodeSocket {
                    node,
                    socket,
                    input,
                } => {
                    println!(
                        "TODO right click node {:?} socket {} input {}",
                        node, socket, input
                    );
                }
                Hover::Node(node) => {
                    println!("TODO right click node {:?}", node);
                }
                Hover::None => {
                    let pos = ctx.mouse;
                    self.set_right_click_menu(
                        ctx,
                        [
                            ContextMenuItem::new("Add recipe", move |app| {
                                app.open_recipe_selector(pos)
                            }),
                            ContextMenuItem::new("Add item", move |app| {
                                app.open_item_selector(pos)
                            }),
                            ContextMenuItem::new("Add fluid", move |app| {
                                app.open_fluid_selector(pos)
                            }),
                        ],
                    );
                }
            }
        }
    }

    fn mouse_move(&mut self, ctx: &mut EventCtx, delta: Vec2) {
        if let Some(menu) = &mut self.context_menu {
            if menu.mouse_move(ctx.mouse) {
                ctx.redraw();
                return;
            }
        }
        if let Some(selector) = &mut self.selector {
            if selector.mouse_move(ctx.mouse) {
                ctx.redraw();
                return;
            }
        }
        self.dragging.mouse_move(delta, ctx, &mut self.nodes);

        let hover = self.find_hover(ctx.mouse);
        if hover != self.hover {
            self.hover = hover;
            ctx.redraw();
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx, key: winit::event::VirtualKeyCode) {
        if key == winit::event::VirtualKeyCode::Escape {
            ctx.exit();
        }
    }

    fn mouse_scroll(&mut self, ctx: &mut EventCtx, delta: winit::event::MouseScrollDelta) {
        let zoom = match delta {
            winit::event::MouseScrollDelta::LineDelta(_x, y) => y,
            winit::event::MouseScrollDelta::PixelDelta(winit::dpi::PhysicalPosition {
                y, ..
            }) => y as f32,
        };
        ctx.zoom_at_mouse(zoom);
    }
}

impl App {
    pub fn find_hover(&self, point: Point2) -> Hover {
        for node in self.nodes.values() {
            for (index, _input) in node.inputs.iter().enumerate() {
                if get_node_socket_position(node, index, true).contains(point) {
                    return Hover::NodeSocket {
                        node: node.id,
                        socket: index,
                        input: true,
                    };
                }
            }

            for (index, _output) in node.outputs.iter().enumerate() {
                if get_node_socket_position(node, index, false).contains(point) {
                    return Hover::NodeSocket {
                        node: node.id,
                        socket: index,
                        input: false,
                    };
                }
            }

            if get_node_position(node).contains(point) {
                return Hover::Node(node.id);
            }
        }
        Hover::None
    }
}

fn draw_node(ctx: &mut DrawCtx, node: &Node, hover: bool, hover_socket: Option<(usize, bool)>) {
    let DrawCtx { canvas, theme, .. } = ctx;
    let rectangle = get_node_position(node);
    let bg_paint = Paint::color(theme.layer_color(if hover { 2 } else { 1 }));
    let border_paint = Paint::color(theme.layer_color(if hover { 3 } else { 2 }));
    rectangle.draw_rounded(canvas, &bg_paint, &border_paint, 5.);

    let bg_paint = Paint::color(theme.layer_color(2));
    let border_paint = Paint::color(theme.layer_color(3));

    let highlight_bg_paint = Paint::color(theme.layer_color(3));
    let highlight_border_paint = Paint::color(theme.layer_color(4));

    if !node.inputs.is_empty() {
        for (i, _input) in node.inputs.iter().enumerate() {
            let rectangle = get_node_socket_position(node, i, true);
            let is_hover = hover_socket == Some((i, true));

            let bg_paint = if is_hover {
                &highlight_bg_paint
            } else {
                &bg_paint
            };
            let border_paint = if is_hover {
                &highlight_border_paint
            } else {
                &border_paint
            };
            rectangle.draw_rounded(canvas, bg_paint, border_paint, 2.0);
        }
    }

    if !node.outputs.is_empty() {
        for (i, _output) in node.outputs.iter().enumerate() {
            let rectangle = get_node_socket_position(node, i, false);
            let is_hover = hover_socket == Some((i, false));

            let bg_paint = if is_hover {
                &highlight_bg_paint
            } else {
                &bg_paint
            };
            let border_paint = if is_hover {
                &highlight_border_paint
            } else {
                &border_paint
            };
            rectangle.draw_rounded(canvas, bg_paint, border_paint, 2.0);
        }
    }
}

fn get_socket_initial_direction(nodes: &FxHashMap<NodeId, Node>, hover: Hover) -> Vec2 {
    match hover {
        Hover::NodeSocket { node, input, .. } => {
            let node = nodes.get(&node).unwrap();
            node.direction.line_direction(input, 50.)
        }
        _ => Vec2::ZERO,
    }
}

fn get_node_position(node: &Node) -> Rectangle {
    Rectangle::centered_square(node.position, 100.)
}
