mod json;

use instant::Instant;
use noisy_float::types::R32;
use rustc_hash::FxHashMap;
use std::path::PathBuf;

#[derive(Debug)]
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
        println!("  Found {} recipes", deserialized.recipe.len());
        println!("  Found {} items", deserialized.item.len());
        println!("  Found {} fluids", deserialized.fluid.len());

        let mut preset = Preset {
            name: name.to_string(),
            recipes: FxHashMap::default(),
            buildings: FxHashMap::default(),
            items: FxHashMap::default(),
            fluids: FxHashMap::default(),
        };

        for (recipe_name, recipe) in deserialized.recipe {
            let id = RecipeId::hashed(recipe_name);

            if preset.recipes.contains_key(&id) {
                panic!(
                    "Duplicate recipe id: {:?}, {} - {}",
                    id, preset.recipes[&id].name, recipe_name
                );
            }
            preset.recipes.insert(
                id,
                Recipe {
                    id,
                    name: recipe_name.to_string(),
                    ..Default::default()
                },
            );
        }

        for (item_name, item) in deserialized.item {
            let id = ItemId::hashed(item_name);

            if preset.items.contains_key(&id) {
                panic!(
                    "Duplicate item id: {:?}, {} - {}",
                    id, preset.items[&id].name, item_name
                );
            }
            preset.items.insert(
                id,
                Item {
                    id,
                    name: item_name.to_string(),
                    ..Default::default()
                },
            );
        }

        for (fluid_name, fluid) in deserialized.fluid {
            let id = FluidId::hashed(fluid_name);

            if preset.fluids.contains_key(&id) {
                panic!(
                    "Duplicate fluid id: {:?}, {} - {}",
                    id, preset.fluids[&id].name, fluid_name
                );
            }
            preset.fluids.insert(
                id,
                Fluid {
                    id,
                    name: fluid_name.to_string(),
                },
            );
        }

        dbg!(preset)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RecipeId(pub u64);

impl RecipeId {
    pub fn hashed(name: &str) -> Self {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::hash::DefaultHasher::default();
        name.hash(&mut hasher);
        RecipeId(hasher.finish())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BuildingId(pub u64);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ItemId(pub u64);

impl ItemId {
    pub fn hashed(name: &str) -> Self {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::hash::DefaultHasher::default();
        name.hash(&mut hasher);
        ItemId(hasher.finish())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FluidId(pub u64);

impl FluidId {
    pub fn hashed(name: &str) -> Self {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::hash::DefaultHasher::default();
        name.hash(&mut hasher);
        FluidId(hasher.finish())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Recipe {
    pub id: RecipeId,
    pub name: String,
    pub allowed_buildings: Vec<BuildingId>,
    pub inputs: Vec<InOutput>,
    pub outputs: Vec<InOutput>,
}

impl Recipe {
    pub fn icon(&self, preset_name: &str) -> PathBuf {
        PathBuf::from(format!("preset/{}/recipe/{}.png", preset_name, self.name))
    }
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
    pub speed: R32,
}

impl Building {
    pub fn icon(&self, preset_name: &str) -> PathBuf {
        PathBuf::from(format!("preset/{}/entity/{}.png", preset_name, self.name))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ItemOrFluidId {
    Item(ItemId),
    Fluid(FluidId),
}

#[derive(Clone, Debug, Default)]
pub struct Item {
    pub id: ItemId,
    pub name: String,
}

impl Item {
    pub fn icon(&self, preset_name: &str) -> PathBuf {
        PathBuf::from(format!("preset/{}/item/{}.png", preset_name, self.name))
    }
}

#[derive(Clone, Debug, Default)]
pub struct Fluid {
    pub id: FluidId,
    pub name: String,
}

impl Fluid {
    pub fn icon(&self, preset_name: &str) -> PathBuf {
        PathBuf::from(format!("preset/{}/fluid/{}.png", preset_name, self.name))
    }
}
