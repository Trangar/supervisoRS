use femtovg::{Paint, Path};
use rustc_hash::FxHashMap;
use state::{Preset, Theme};
use ui::Canvas;

mod factorio;
mod state;
mod ui;
mod utils;

fn main() {
    // let factorio_path = factorio::find_factorio_install_dir().unwrap();
    // let config_dir = factorio::find_factorio_config_dir().unwrap();

    // factorio::export::export(factorio::export::ExportArgs {
    //     mod_directory: &config_dir,
    //     factorio_dir: &factorio_path,
    //     output_dir: &std::env::current_dir().unwrap().join("preset").join("k2se"),
    // });
    let preset = Preset::load("k2se");
    // ui::start(1000, 800, "SupervisoRS", true, App::default());
}

struct App {
    nodes: FxHashMap<NodeId, Node>,
    connections: Vec<Connection>,
    dragging: bool,
    theme: Theme,
}

impl Default for App {
    fn default() -> Self {
        Self {
            nodes: utils::demo_nodes(),
            connections: Vec::new(),
            dragging: false,
            theme: Theme::default(),
        }
    }
}

impl ui::App for App {
    fn draw(&mut self, canvas: &mut ui::Canvas) {
        canvas.clear_rect(
            0,
            0,
            canvas.width(),
            canvas.height(),
            self.theme.background.color,
        );

        for node in self.nodes.values() {
            draw_node(canvas, node, &self.theme);
        }
    }

    fn mouse_down(&mut self, _ctx: &mut ui::EventCtx, button: winit::event::MouseButton) {
        if button == winit::event::MouseButton::Left {
            self.dragging = true;
        }
    }

    fn mouse_up(&mut self, _ctx: &mut ui::EventCtx, button: winit::event::MouseButton) {
        if button == winit::event::MouseButton::Left {
            self.dragging = false;
        }
    }

    fn mouse_move(&mut self, ctx: &mut ui::EventCtx, x: f32, y: f32) {
        if self.dragging {
            ctx.translate(x, y);
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

fn draw_node(canvas: &mut Canvas, node: &Node, theme: &Theme) {
    let mut path = Path::new();
    path.rounded_rect(node.x - 50.0, node.y - 50.0, 100.0, 100.0, 20.0);
    let bg_paint = Paint::color(theme.layer_color(1));
    let border_paint = Paint::color(theme.layer_color(2));

    canvas.fill_path(&path, &bg_paint);
    canvas.stroke_path(&path, &border_paint);

    let bg_paint = Paint::color(theme.layer_color(2));
    let border_paint = Paint::color(theme.layer_color(3));

    if !node.inputs.is_empty() {
        let (mut input_offset, input_step) = node.direction.input_offset();
        input_offset = (node.x + input_offset.0, node.y + input_offset.1);
        input_offset = (
            input_offset.0 - (input_step.0 * (node.inputs.len() - 1) as f32 / 2.),
            input_offset.1 - (input_step.1 * (node.inputs.len() - 1) as f32 / 2.),
        );

        for (i, _input) in node.inputs.iter().enumerate() {
            let x = input_offset.0 + input_step.0 * i as f32;
            let y = input_offset.1 + input_step.1 * i as f32;
            let mut path = Path::new();
            path.rounded_rect(x - 10., y - 10., 20., 20., 5.0);
            canvas.fill_path(&path, &bg_paint);
            canvas.stroke_path(&path, &border_paint);
        }
    }

    if !node.outputs.is_empty() {
        let (mut output_offset, output_step) = node.direction.output_offset();
        output_offset = (node.x + output_offset.0, node.y + output_offset.1);
        output_offset = (
            output_offset.0 - (output_step.0 * (node.outputs.len() - 1) as f32 / 2.),
            output_offset.1 - (output_step.1 * (node.outputs.len() - 1) as f32 / 2.),
        );

        for (i, _output) in node.outputs.iter().enumerate() {
            let x = output_offset.0 + output_step.0 * i as f32;
            let y = output_offset.1 + output_step.1 * i as f32;
            let mut path = Path::new();
            path.rounded_rect(x - 10., y - 10., 20., 20., 5.0);

            canvas.fill_path(&path, &bg_paint);
            canvas.stroke_path(&path, &border_paint);
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
