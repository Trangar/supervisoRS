use state::Preset;
use utils::{Point2, Vec2};

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
    ui::start(1000, 800, "SupervisoRS", true, ui::app::App::new(preset));
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub usize);

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
    position: Point2,

    inputs: Vec<InOutput>,
    outputs: Vec<InOutput>,
    direction: Cardinal,
}
impl Node {
    pub fn get_socket(&self, input: bool, socket_index: usize) -> &InOutput {
        if input {
            &self.inputs[socket_index]
        } else {
            &self.outputs[socket_index]
        }
    }
}

#[derive(Debug)]
pub struct InOutput {
    #[allow(dead_code)]
    item_or_fluid: ItemOrFluidId,
    #[allow(dead_code)]
    rate: f32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Cardinal {
    North,
    East,
    South,
    West,
}

pub const BEZIER_CURVE_FACTOR: f32 = 0.5;

impl Cardinal {
    pub fn line_direction(&self, is_input: bool, len: f32) -> Vec2 {
        match (self, is_input) {
            (Cardinal::North, true) => (0., -len),
            (Cardinal::East, true) => (len, 0.),
            (Cardinal::South, true) => (0., len),
            (Cardinal::West, true) => (-len, 0.),
            (Cardinal::South, false) => (0., -len),
            (Cardinal::West, false) => (len, 0.),
            (Cardinal::North, false) => (0., len),
            (Cardinal::East, false) => (-len, 0.),
        }
        .into()
    }
    fn input_offset(&self) -> (Point2, Vec2) {
        match self {
            Cardinal::North => ((0.0, -50.0).into(), (20.0, 0.0).into()),
            Cardinal::East => ((50.0, 0.0).into(), (0.0, 20.0).into()),
            Cardinal::South => ((0.0, 50.0).into(), (20.0, 0.0).into()),
            Cardinal::West => ((-50.0, 0.0).into(), (0.0, 20.0).into()),
        }
    }

    fn output_offset(&self) -> (Point2, Vec2) {
        match self {
            Cardinal::South => ((0.0, -50.0).into(), (20.0, 0.0).into()),
            Cardinal::West => ((50.0, 0.0).into(), (0.0, 20.0).into()),
            Cardinal::North => ((0.0, 50.0).into(), (20.0, 0.0).into()),
            Cardinal::East => ((-50.0, 0.0).into(), (0.0, 20.0).into()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SocketPos {
    pub node_id: NodeId,
    pub socket_index: usize,
    pub input: bool,
}

impl From<(NodeId, usize, bool)> for SocketPos {
    fn from((node_id, socket_index, input): (NodeId, usize, bool)) -> Self {
        Self {
            node_id,
            socket_index,
            input,
        }
    }
}

pub struct Connection {
    pub src: SocketPos,
    pub dst: SocketPos,
}
impl Connection {
    pub fn has_socket(&self, pos: SocketPos) -> bool {
        self.src == pos || self.dst == pos
    }
}
