use clap::Parser;
use serde_json::Value;
use state::Preset;
use std::io::Write;
use utils::{Point2, Vec2};

mod factorio;
mod gfx;
mod state;
mod ui;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Export the different data-raw-dump.json types to `/tmp/`. Useful for figuring out what types exist
    #[arg(long)]
    export_tmp: bool,
    /// Convert `preset/NAME/script-output/data-raw-dump.json` to `preset/NAME/preset.json`
    #[arg(long)]
    convert_data_raw_dump: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.export_tmp {
        let content =
            std::fs::read_to_string("preset/py/script-output/data-raw-dump.json").unwrap();
        let _ = std::fs::remove_dir("tmp");
        let _ = std::fs::create_dir("tmp");
        let json: Value = serde_json::from_str(&content).unwrap();
        let Value::Object(o) = json else {
            unreachable!()
        };
        for (ty, list) in o {
            let Value::Object(o) = list else {
                unreachable!()
            };

            let mut file = std::fs::File::create(format!("tmp/{ty}.json")).unwrap();
            writeln!(&mut file, "[").unwrap();

            for (i, v) in o.values().enumerate() {
                if i > 0 {
                    writeln!(&mut file, ",").unwrap();
                }
                serde_json::to_writer(&mut file, v).unwrap();
            }
            writeln!(&mut file).unwrap();
            writeln!(&mut file, "]").unwrap();
        }
        return;
    }

    if cli.convert_data_raw_dump {}
    const PRESET_NAME: &str = "py";
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
    let mut window = gfx::Window::new();
    for item in preset.items.values() {
        window
            .image_cache
            .textures_to_load
            .insert(preset.icon_for_item(item));
    }
    for fluid in preset.fluids.values() {
        window
            .image_cache
            .textures_to_load
            .insert(preset.icon_for_fluid(fluid));
    }
    for recipe in preset.recipes.values() {
        window
            .image_cache
            .textures_to_load
            .insert(preset.icon_for_recipe(recipe));
    }
    gfx::run(&mut window, &mut ui::app::App::new(preset));
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub usize);

// #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// struct ItemId(pub usize);

// #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// struct FluidId(pub usize);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ItemOrFluidId {
    Item(String),
    Fluid(String),
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
