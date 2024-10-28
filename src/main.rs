use std::{
    fs::File,
    io::{BufWriter, Write},
};

use femtovg::{Paint, Path};
use heck::ToUpperCamelCase;
use rustc_hash::{FxHashMap, FxHashSet};
use state::Theme;
use ui::Canvas;

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
    let _preset = Preset::load(PRESET_NAME);
    // ui::start(1000, 800, "SupervisoRS", true, App::default());
}

fn generate_type(name: String, ty: Ty) {
    std::fs::create_dir_all("src/generated").unwrap();
    let mut f = File::create(format!("src/generated/{name}.rs")).unwrap();

    generate_struct(&mut f, vec![name], ty.fields).unwrap();
}

fn generate_struct(
    file: &mut File,
    names: Vec<String>,
    fields: FxHashMap<String, FieldTy>,
) -> std::io::Result<()> {
    let name = names.iter().fold(String::new(), |mut agg, n| {
        agg.push_str(&n.to_upper_camel_case());
        agg
    });
    writeln!(file, "pub struct {name} {{ ")?;
    for (name, ty) in fields {
        let (field_ty, field_ty_comment) = getFieldType(&names, &name, ty);
        if let Some(comment) = field_ty_comment {
            writeln!(file, "  /// {comment}")?;
        }
        writeln!(file, "  {name}: {field_ty},")?;
    }
    Ok(())
}

fn getFieldType(names: &[String], field_name: &String, ty: FieldTy) -> (String, Option<String>) {
    match ty {
        FieldTy::Object(_) => {
            let ty = names
                .iter()
                .chain(Some(field_name))
                .fold(String::new(), |mut agg, n| {
                    agg.push_str(&n.to_upper_camel_case());
                    agg
                });
            (ty, None)
        }
        FieldTy::Float { min, max } => ("f64".to_owned(), Some(format!("min: {min}, max: {max}"))),
        FieldTy::Integer { min, max } => {
            ("i64".to_owned(), Some(format!("min: {min}, max: {max}")))
        }
        FieldTy::String(str) => ("String".to_owned(), Some(format!("{str:?}"))),
        FieldTy::Boolean => (String::from("bool"), None),
        _ => todo!("{names:?} {field_name:?} {ty:?}"),
    }
}

impl From<serde_json::Value> for FieldTy {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Object(o) => {
                let mut map = FxHashMap::default();
                for (key, val) in o {
                    map.insert(key, FieldTy::from(val));
                }
                FieldTy::Object(map)
            }
            serde_json::Value::Null => unreachable!(),
            serde_json::Value::Bool(_) => FieldTy::Boolean,
            serde_json::Value::Number(number) => {
                if number.is_i64() {
                    let n = number.as_i64().unwrap();
                    FieldTy::Integer { min: n, max: n }
                } else {
                    let n = number.as_f64().unwrap();
                    FieldTy::Float { min: n, max: n }
                }
            }
            serde_json::Value::String(s) => {
                let mut set = FxHashSet::default();
                set.insert(s);
                FieldTy::String(set)
            }
            serde_json::Value::Array(values) => {
                FieldTy::Array(values.into_iter().map(Into::into).collect())
            }
        }
    }
}

#[derive(Debug, Default, serde::Serialize)]
struct Ty {
    names: FxHashSet<String>,
    fields: FxHashMap<String, FieldTy>,
}

#[derive(Debug, Default, Clone, serde::Serialize)]
enum FieldTy {
    #[default]
    Unknown,
    Skip,

    String(FxHashSet<String>),
    Boolean,
    Color,
    Integer {
        min: i64,
        max: i64,
    },
    Float {
        min: f64,
        max: f64,
    },
    Object(FxHashMap<String, FieldTy>),
    Multiple(Vec<FieldTy>),
    Array(Vec<FieldTy>),
    ArrayOrEmptyObject(Vec<FieldTy>),
}
impl FieldTy {
    fn merge(&mut self, val: FieldTy) {
        match (self, val) {
            (s @ FieldTy::Unknown, val) => *s = val,
            (FieldTy::Object(o), FieldTy::Object(val)) => {
                for (key, val) in val {
                    o.entry(key).or_default().merge(val);
                }
            }
            (FieldTy::String(a), FieldTy::String(b)) => {
                a.extend(b);
            }
            (FieldTy::Array(a), FieldTy::Array(b)) => {
                a.extend(b);
            }
            (
                s @ FieldTy::Integer { .. },
                FieldTy::Float {
                    min: val_min,
                    max: val_max,
                },
            ) => {
                let (min, max) = match s {
                    FieldTy::Integer { min, max } => (*min as f64, *max as f64),
                    _ => unreachable!(),
                };
                *s = FieldTy::Float {
                    min: min.min(val_min),
                    max: max.min(val_max),
                }
            }
            (FieldTy::Multiple(m), o) => {
                m.push(o);
            }
            (
                FieldTy::Float { min, max },
                FieldTy::Integer {
                    min: i_min,
                    max: i_max,
                },
            ) => {
                *min = (*min).min(i_min as f64);
                *max = (*max).max(i_max as f64);
            }
            (
                FieldTy::Integer { min, max },
                FieldTy::Integer {
                    min: val_min,
                    max: val_max,
                },
            ) => {
                *min = (*min).min(val_min);
                *max = (*max).max(val_max);
            }
            (
                FieldTy::Float { min, max },
                FieldTy::Float {
                    min: val_min,
                    max: val_max,
                },
            ) => {
                *min = (*min).min(val_min);
                *max = (*max).max(val_max);
            }
            (FieldTy::Boolean, FieldTy::Boolean) => {}
            (a, o) => *a = FieldTy::Multiple(vec![a.clone(), o]),
        }
    }

    fn collapse(&mut self, path: Vec<String>) {
        if path.iter().any(|t| t.contains("expression")) {
            *self = FieldTy::Skip;
            return;
        }
        let mut set_color = false;
        let mut set_is_array_or_empty_object = false;
        let mut set_flatten_array = false;
        match self {
            Self::Array(a) => {
                if a.iter().all(|a| a.is_color()) {
                    set_color = true;
                }
            }
            Self::Object(o) => {
                for (key, val) in o.iter_mut() {
                    let mut p = path.clone();
                    p.push(key.clone());
                    val.collapse(p);
                }
            }
            Self::Multiple(a) => {
                if a.iter().all(|f| f.is_color()) {
                    set_color = true;
                } else if a.iter().all(|f| f.is_array() || f.is_empty_object()) {
                    set_is_array_or_empty_object = true;
                } else if a.iter().any(|f| f.is_array()) {
                    set_flatten_array = true;
                } else {
                    println!("TODO: Collapse (path: {path:?})");
                    for a in a {
                        println!(" - {a:?}");
                    }
                    std::process::exit(1);
                }
            }
            Self::ArrayOrEmptyObject(a) => {
                let mut agg = FieldTy::Unknown;
                for mut a in a.clone() {
                    let mut p = path.clone();
                    p.push("[..]".to_owned());
                    a.collapse(p);
                    agg.merge(a);
                }
                *self = agg;
            }
            _ => {}
        }
        if set_is_array_or_empty_object {
            let Self::Multiple(a) = std::mem::take(self) else {
                panic!()
            };
            let array = a
                .into_iter()
                .filter_map(|a| {
                    if let Self::Array(a) = a {
                        Some(a)
                    } else {
                        None
                    }
                })
                .flat_map(|a| a);
            *self = Self::ArrayOrEmptyObject(array.collect());
            self.collapse(path);
        } else if set_color {
            *self = FieldTy::Color;
        } else if set_flatten_array {
            let Self::Multiple(a) = std::mem::take(self) else {
                panic!()
            };
            let mut array = Vec::new();
            for a in a {
                match a {
                    FieldTy::Array(a) => array.extend(a),
                    x => array.push(x),
                }
            }
            *self = Self::Array(array);
            self.collapse(path);
        }
    }

    fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }
    fn is_empty_object(&self) -> bool {
        if let Self::Object(o) = self {
            o.len() == 0
        } else {
            false
        }
    }

    fn is_color(&self) -> bool {
        match self {
            Self::Object(o)
                if o.contains_key("r") && o.contains_key("g") && o.contains_key("b") =>
            {
                true
            }
            Self::Array(e)
                if e.len() == 3 && e.iter().all(|e| matches!(e, FieldTy::Integer { .. })) =>
            {
                true
            }
            _ => false,
        }
    }
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
