mod json;

use instant::Instant;
use noisy_float::types::R32;
use rustc_hash::FxHashMap;
use std::path::PathBuf;

pub struct Preset {
    pub name: String,
    pub recipes: FxHashMap<RecipeId, Recipe>,
    pub buildings: FxHashMap<BuildingId, Building>,
    pub items: FxHashMap<ItemId, Item>,
    pub fluids: FxHashMap<FluidId, Fluid>,
}

impl Preset {
    pub fn load(name: &str) -> Self {
        let start = Instant::now();
        let path = format!("preset/{}/script-output/data-raw-dump.json", name);
        let data = std::fs::read_to_string(path).expect("Failed to read data-raw-dump.json");

        let deserialized = serde_json::from_str::<json::Root>(&data)
            .expect("Failed to deserialize data-raw-dump.json");
        println!(
            "Parsed {} bytes ({:.2} MB) in {:?}",
            data.bytes().len(),
            data.bytes().len() as f64 / 1024.0 / 1024.0,
            start.elapsed()
        );

        // dbg!(deserialized);

        todo!();
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecipeId(pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuildingId(pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemId(pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FluidId(pub usize);

#[derive(Clone, Debug)]
pub struct Recipe {
    pub id: RecipeId,
    pub allowed_buildings: Vec<BuildingId>,
    pub inputs: Vec<InOutput>,
    pub outputs: Vec<InOutput>,
    pub icon: PathBuf,
}

#[derive(Clone, Debug)]
pub struct InOutput {
    pub item_or_fluid: ItemOrFluidId,
    pub rate: R32,
}

#[derive(Clone, Debug)]
pub struct Building {
    pub id: BuildingId,
    pub name: String,
    pub icon: PathBuf,
    pub speed: R32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ItemOrFluidId {
    Item(ItemId),
    Fluid(FluidId),
}

#[derive(Clone, Debug)]
pub struct Item {
    pub id: ItemId,
    pub name: String,
    pub icon: PathBuf,
}

#[derive(Clone, Debug)]
pub struct Fluid {
    pub id: FluidId,
    pub name: String,
    pub icon: PathBuf,
}
