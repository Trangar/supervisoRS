use femtovg::{Color, Paint, Path};
use rustc_hash::FxHashMap;

// mod factorio;
mod ui;

fn main() {
    ui::start(1000, 800, "SupervisoRS", true, App::default());
}

struct App {
    nodes: FxHashMap<NodeId, Node>,
    connections: Vec<Connection>,
}

impl Default for App {
    fn default() -> Self {
        let mut nodes = FxHashMap::default();
        nodes.insert(
            NodeId(0),
            Node {
                id: NodeId(0),
                x: 100.0,
                y: 100.0,
                inputs: vec![
                    InOutput {
                        item_or_fluid: ItemOrFluidId::Item(ItemId(0)),
                        rate: 1.0,
                    },
                    InOutput {
                        item_or_fluid: ItemOrFluidId::Fluid(FluidId(0)),
                        rate: 1.0,
                    },
                ],
                outputs: vec![
                    InOutput {
                        item_or_fluid: ItemOrFluidId::Item(ItemId(1)),
                        rate: 1.0,
                    },
                    InOutput {
                        item_or_fluid: ItemOrFluidId::Fluid(FluidId(1)),
                        rate: 1.0,
                    },
                ],

                direction: Cardinal::East,
            },
        );
        nodes.insert(
            NodeId(1),
            Node {
                id: NodeId(1),
                x: 200.0,
                y: 200.0,

                inputs: vec![
                    InOutput {
                        item_or_fluid: ItemOrFluidId::Item(ItemId(1)),
                        rate: 1.0,
                    },
                    InOutput {
                        item_or_fluid: ItemOrFluidId::Fluid(FluidId(1)),
                        rate: 1.0,
                    },
                ],
                outputs: vec![
                    InOutput {
                        item_or_fluid: ItemOrFluidId::Item(ItemId(2)),
                        rate: 1.0,
                    },
                    InOutput {
                        item_or_fluid: ItemOrFluidId::Fluid(FluidId(2)),
                        rate: 1.0,
                    },
                ],

                direction: Cardinal::South,
            },
        );
        nodes.insert(
            NodeId(2),
            Node {
                id: NodeId(2),
                x: 300.0,
                y: 300.0,

                inputs: vec![
                    InOutput {
                        item_or_fluid: ItemOrFluidId::Item(ItemId(2)),
                        rate: 1.0,
                    },
                    InOutput {
                        item_or_fluid: ItemOrFluidId::Fluid(FluidId(2)),
                        rate: 1.0,
                    },
                ],
                outputs: vec![
                    InOutput {
                        item_or_fluid: ItemOrFluidId::Item(ItemId(3)),
                        rate: 1.0,
                    },
                    InOutput {
                        item_or_fluid: ItemOrFluidId::Fluid(FluidId(3)),
                        rate: 1.0,
                    },
                ],

                direction: Cardinal::West,
            },
        );
        nodes.insert(
            NodeId(3),
            Node {
                id: NodeId(3),
                x: 400.0,
                y: 400.0,

                inputs: vec![
                    InOutput {
                        item_or_fluid: ItemOrFluidId::Item(ItemId(3)),
                        rate: 1.0,
                    },
                    InOutput {
                        item_or_fluid: ItemOrFluidId::Fluid(FluidId(3)),
                        rate: 1.0,
                    },
                ],
                outputs: vec![
                    InOutput {
                        item_or_fluid: ItemOrFluidId::Item(ItemId(0)),
                        rate: 1.0,
                    },
                    InOutput {
                        item_or_fluid: ItemOrFluidId::Fluid(FluidId(0)),
                        rate: 1.0,
                    },
                ],

                direction: Cardinal::North,
            },
        );

        Self {
            nodes,
            connections: Vec::new(),
        }
    }
}

impl ui::App for App {
    fn draw(&mut self, canvas: &mut ui::Canvas) {
        let tile_background = Paint::color(Color::rgbf(0.0, 0.0, 0.0));
        let connection_background = Paint::color(Color::rgbf(0.0, 255.0, 0.0));
        for node in self.nodes.values() {
            let mut path = Path::new();
            path.rounded_rect(node.x - 50.0, node.y - 50.0, 100.0, 100.0, 20.0);
            canvas.fill_path(&path, &tile_background);

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
                    path.circle(x, y, 10.0);
                    canvas.fill_path(&path, &connection_background);
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
                    path.circle(x, y, 10.0);
                    canvas.fill_path(&path, &connection_background);
                }
            }
        }
    }

    fn key_down(&mut self, ctx: &mut ui::EventCtx, key: winit::event::VirtualKeyCode) {
        if key == winit::event::VirtualKeyCode::Escape {
            ctx.exit();
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
