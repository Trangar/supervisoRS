use femtovg::{Paint, Path};
use rustc_hash::FxHashMap;
use state::{Preset, Theme};
use ui::{Canvas, DrawCtx};

mod factorio;
mod state;
mod ui;
mod utils;

fn main() {
    const PRESET_NAME: &str = "space_age";
    let preset_path = std::env::current_dir()
        .unwrap()
        .join("preset")
        .join(PRESET_NAME);
    if !preset_path.exists() {
        let factorio_path = factorio::find_factorio_install_dir().unwrap();
        let config_dir = factorio::find_factorio_config_dir().unwrap();

        factorio::export::export(factorio::export::ExportArgs {
            mod_directory: &config_dir,
            factorio_dir: &factorio_path,
            output_dir: &preset_path,
        });
    }

    let preset = Preset::load(PRESET_NAME);
    // ui::start(
    //     1000,
    //     800,
    //     "SupervisoRS",
    //     true,
    //     App {
    //         preset,
    //         ..Default::default()
    //     },
    // );
}

struct App {
    preset: Preset,
    nodes: FxHashMap<NodeId, Node>,
    connections: Vec<Connection>,
    theme: Theme,
    hover: Hover,
    dragging: Drag,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hover {
    None,
    Node(NodeId),
    NodeSocket {
        node: NodeId,
        socket: usize,
        input: bool,
    },
}

impl Hover {
    fn should_highlight_node(&self, node: &Node) -> bool {
        match self {
            Hover::Node(id) => id == &node.id,
            _ => false,
        }
    }

    fn get_highlight_socket(&self, node: &Node) -> Option<(usize, bool)> {
        match self {
            Hover::NodeSocket {
                node: id,
                socket,
                input,
            } if id == &node.id => Some((*socket, *input)),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
enum Drag {
    None,
    Everything,
    Node(NodeId),
    LineFromNodeSocket {
        node: NodeId,
        socket_index: usize,
        input: bool,
        initial_direction: (f32, f32),
    },
}

impl Drag {
    const BEZIER_DIST: f32 = 50.;

    fn draw_line(
        &self,
        nodes: &FxHashMap<NodeId, Node>,
        canvas: &mut Canvas,
        paint: &Paint,
        to: (f32, f32),
    ) {
        if let Drag::LineFromNodeSocket {
            node,
            socket_index,
            input,
            initial_direction,
        } = self
        {
            let mut path = Path::new();
            let node = nodes.get(node).unwrap();
            let position = get_node_socket_position(node, *socket_index, *input);
            let initial_ = (position.x + 10.0, position.y + 10.0);
            path.move_to(initial_.0, initial_.1);
            let end_direction = (
                if to.0 > initial_.0 {
                    -Self::BEZIER_DIST
                } else {
                    Self::BEZIER_DIST
                },
                if to.1 > initial_.1 {
                    -Self::BEZIER_DIST
                } else {
                    Self::BEZIER_DIST
                },
            );
            path.bezier_to(
                initial_.0 + initial_direction.0,
                initial_.1 + initial_direction.1,
                to.0 + end_direction.0,
                to.1 + end_direction.1,
                to.0,
                to.1,
            );

            canvas.stroke_path(&path, paint);
        }
    }

    fn should_highlight_node(&self, node: &Node) -> bool {
        match self {
            Drag::Node(id) => id == &node.id,
            Drag::LineFromNodeSocket { node: id, .. } => id == &node.id,
            _ => false,
        }
    }

    fn get_highlight_socket(&self, node: &Node) -> Option<(usize, bool)> {
        match self {
            Drag::LineFromNodeSocket {
                node: id,
                socket_index,
                input,
                ..
            } if id == &node.id => Some((*socket_index, *input)),
            _ => None,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            preset: Preset::default(),
            nodes: utils::demo_nodes(),
            connections: Vec::new(),
            theme: Theme::default(),
            hover: Hover::None,
            dragging: Drag::None,
        }
    }
}

impl ui::App for App {
    fn draw(&mut self, canvas: &mut ui::Canvas, ctx: DrawCtx) {
        canvas.clear_rect(
            0,
            0,
            canvas.width(),
            canvas.height(),
            self.theme.background.color,
        );

        for node in self.nodes.values() {
            draw_node(
                canvas,
                node,
                &self.theme,
                self.hover.should_highlight_node(node) || self.dragging.should_highlight_node(node),
                self.hover
                    .get_highlight_socket(node)
                    .or_else(|| self.dragging.get_highlight_socket(node)),
            );
        }

        self.dragging.draw_line(
            &self.nodes,
            canvas,
            &Paint::color(self.theme.layer_color(3)).with_line_width(5.),
            (ctx.mousex, ctx.mousey),
        );
    }

    fn mouse_down(&mut self, _ctx: &mut ui::EventCtx, button: winit::event::MouseButton) {
        if button == winit::event::MouseButton::Left {
            if let Hover::NodeSocket {
                node,
                socket,
                input,
            } = self.hover
            {
                self.dragging = Drag::LineFromNodeSocket {
                    node,
                    socket_index: socket,
                    input,
                    initial_direction: get_socket_initial_direction(&self.nodes, self.hover),
                };
            } else if let Hover::Node(node_id) = self.hover {
                self.dragging = Drag::Node(node_id);
            } else {
                self.dragging = Drag::Everything;
            }
        }
    }

    fn mouse_up(&mut self, ctx: &mut ui::EventCtx, button: winit::event::MouseButton) {
        if button == winit::event::MouseButton::Left {
            self.dragging = Drag::None;
            ctx.redraw();
        }
    }

    fn mouse_move(&mut self, ctx: &mut ui::EventCtx, x: f32, y: f32) {
        match self.dragging {
            Drag::Everything => {
                ctx.translate(x, y);
                ctx.redraw();
                return;
            }
            Drag::Node(node_id) => {
                let node = self.nodes.get_mut(&node_id).unwrap();
                node.x = x;
                node.y = y;
                ctx.redraw();
                return;
            }
            Drag::LineFromNodeSocket { .. } => ctx.redraw(),
            _ => {}
        }

        let hover = self.find_hover(x, y);
        if hover != self.hover {
            self.hover = hover;
            ctx.redraw();
        }
    }

    fn key_down(&mut self, ctx: &mut ui::EventCtx, key: winit::event::VirtualKeyCode) {
        if key == winit::event::VirtualKeyCode::Escape {
            ctx.exit();
        }
    }

    fn mouse_scroll(&mut self, ctx: &mut ui::EventCtx, delta: winit::event::MouseScrollDelta) {
        let zoom = match delta {
            winit::event::MouseScrollDelta::LineDelta(_x, y) => y,
            winit::event::MouseScrollDelta::PixelDelta(winit::dpi::PhysicalPosition {
                y, ..
            }) => y as f32,
        };
        ctx.zoom_at_mouse(zoom);
    }
}

fn get_socket_initial_direction(nodes: &FxHashMap<NodeId, Node>, hover: Hover) -> (f32, f32) {
    match hover {
        Hover::NodeSocket { node, input, .. } => {
            let len = Drag::BEZIER_DIST;
            let node = nodes.get(&node).unwrap();
            match (node.direction, input) {
                (Cardinal::North, true) => (0., -len),
                (Cardinal::East, true) => (len, 0.),
                (Cardinal::South, true) => (0., len),
                (Cardinal::West, true) => (-len, 0.),
                (Cardinal::South, false) => (0., -len),
                (Cardinal::West, false) => (len, 0.),
                (Cardinal::North, false) => (0., len),
                (Cardinal::East, false) => (-len, 0.),
            }
        }
        _ => (0., 0.),
    }
}

impl App {
    fn find_hover(&self, x: f32, y: f32) -> Hover {
        for node in self.nodes.values() {
            for (index, _input) in node.inputs.iter().enumerate() {
                if get_node_socket_position(node, index, true).contains(x, y) {
                    return Hover::NodeSocket {
                        node: node.id,
                        socket: index,
                        input: true,
                    };
                }
            }

            for (index, _output) in node.outputs.iter().enumerate() {
                if get_node_socket_position(node, index, false).contains(x, y) {
                    return Hover::NodeSocket {
                        node: node.id,
                        socket: index,
                        input: false,
                    };
                }
            }

            if get_node_position(node).contains(x, y) {
                return Hover::Node(node.id);
            }
        }
        Hover::None
    }
}

struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Rectangle {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    pub fn draw(&self, canvas: &mut Canvas, bg_paint: &Paint, border_paint: &Paint) {
        let mut path = Path::new();
        path.rounded_rect(self.x, self.y, self.width, self.height, 5.0);
        canvas.fill_path(&path, bg_paint);
        canvas.stroke_path(&path, border_paint);
    }
}

fn get_node_position(node: &Node) -> Rectangle {
    Rectangle {
        x: node.x - 50.0,
        y: node.y - 50.0,
        width: 100.0,
        height: 100.0,
    }
}

fn get_node_socket_position(node: &Node, socket: usize, is_input: bool) -> Rectangle {
    let (offset, step) = if is_input {
        node.direction.input_offset()
    } else {
        node.direction.output_offset()
    };

    let offset = (node.x + offset.0, node.y + offset.1);
    let offset = (
        offset.0 - (step.0 * (node.inputs.len() - 1) as f32 / 2.),
        offset.1 - (step.1 * (node.inputs.len() - 1) as f32 / 2.),
    );

    let x = offset.0 + step.0 * socket as f32;
    let y = offset.1 + step.1 * socket as f32;

    Rectangle {
        x: x - 10.0,
        y: y - 10.0,
        width: 20.0,
        height: 20.0,
    }
}

fn draw_node(
    canvas: &mut Canvas,
    node: &Node,
    theme: &Theme,
    hover: bool,
    hover_socket: Option<(usize, bool)>,
) {
    let rectangle = get_node_position(node);
    let bg_paint = Paint::color(theme.layer_color(if hover { 2 } else { 1 }));
    let border_paint = Paint::color(theme.layer_color(if hover { 3 } else { 2 }));
    rectangle.draw(canvas, &bg_paint, &border_paint);

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
            rectangle.draw(canvas, bg_paint, border_paint);
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
            rectangle.draw(canvas, bg_paint, border_paint);
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct NodeId(pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ItemId(pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct FluidId(pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ItemOrFluidId {
    Item(ItemId),
    Fluid(FluidId),
}

struct Node {
    id: NodeId,
    x: f32,
    y: f32,

    inputs: Vec<InOutput>,
    outputs: Vec<InOutput>,
    direction: Cardinal,
}

struct InOutput {
    item_or_fluid: ItemOrFluidId,
    rate: f32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Cardinal {
    North,
    East,
    South,
    West,
}

impl Cardinal {
    fn input_offset(&self) -> ((f32, f32), (f32, f32)) {
        match self {
            Cardinal::North => ((0.0, -50.0), (20.0, 0.0)),
            Cardinal::East => ((50.0, 0.0), (0.0, 20.0)),
            Cardinal::South => ((0.0, 50.0), (20.0, 0.0)),
            Cardinal::West => ((-50.0, 0.0), (0.0, 20.0)),
        }
    }

    fn output_offset(&self) -> ((f32, f32), (f32, f32)) {
        match self {
            Cardinal::South => ((0.0, -50.0), (20.0, 0.0)),
            Cardinal::West => ((50.0, 0.0), (0.0, 20.0)),
            Cardinal::North => ((0.0, 50.0), (20.0, 0.0)),
            Cardinal::East => ((-50.0, 0.0), (0.0, 20.0)),
        }
    }
}

struct Connection {
    from: NodeId,
    to: NodeId,
}
