mod id_generator;
mod json;

use id_generator::{FrozenIdGenerator, IdGenerator};
use instant::Instant;
use itertools::Itertools;
use json::{Flags, Unit};
use rustc_hash::FxHashMap;
use serde::ser::SerializeSeq;
use std::path::PathBuf;

#[derive(Debug, serde::Serialize)]
pub struct Preset {
    pub name: String,
    #[serde(serialize_with = "fxhashmap_values")]
    pub recipes: FxHashMap<RecipeId, Recipe>,
    #[serde(serialize_with = "fxhashmap_values")]
    pub items: FxHashMap<ItemId, Item>,
    #[serde(serialize_with = "fxhashmap_values")]
    pub fluids: FxHashMap<FluidId, Fluid>,
}

fn fxhashmap_values<S, K, V>(map: &FxHashMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    K: serde::Serialize + Ord + Copy,
    V: serde::Serialize,
{
    let mut seq = serializer.serialize_seq(Some(map.len()))?;
    for (_, value) in map.iter().sorted_by_key(|(key, _)| *key) {
        seq.serialize_element(value)?;
    }
    seq.end()
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
            items: FxHashMap::default(),
            fluids: FxHashMap::default(),
        };

        let mut item_ids = IdGenerator::<ItemId>::default();
        let mut fluid_ids = IdGenerator::<FluidId>::default();
        for name in deserialized.item.keys() {
            item_ids.add(name);
        }
        for name in deserialized.fluid.keys() {
            fluid_ids.add(name);
        }
        for ingredient in deserialized
            .recipe
            .values()
            .flat_map(|r| &r.ingredients)
            .flat_map(|i| i.0.iter())
        {
            if ingredient.ty == "item" {
                item_ids.add(ingredient.name);
            } else if ingredient.ty == "fluid" {
                fluid_ids.add(ingredient.name);
            }
        }
        for result in deserialized
            .recipe
            .values()
            .flat_map(|r| &r.results)
            .flat_map(|r| r.0.iter())
        {
            if result.ty == "item" {
                item_ids.add(result.item);
            } else if result.ty == "fluid" {
                fluid_ids.add(result.item);
            }
        }
        for result in deserialized.recipe.values().filter_map(|r| r.result) {
            item_ids.add(result);
        }

        let item_ids = item_ids.freeze();
        let fluid_ids = fluid_ids.freeze();

        let mut recipe_ids = IdGenerator::<RecipeId>::default();
        for recipe_name in deserialized.recipe.keys() {
            recipe_ids.add(recipe_name);
        }
        let recipe_ids = recipe_ids.freeze();

        for (recipe_name, recipe) in deserialized.recipe {
            let id = recipe_ids.get(recipe_name);

            preset
                .recipes
                .insert(id, Recipe::from_raw(id, recipe, &item_ids, &fluid_ids));
        }

        for (item_name, item) in deserialized.item {
            let id = item_ids.get(item_name);

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
                    stack_size: item.stack_size,
                    group: item.group.map(|g| g.to_owned()),
                    subgroup: item.subgroup.map(|s| s.to_owned()),
                    category: item.category.map(|c| c.to_owned()),
                    hidden: item.hidden.unwrap_or(false),
                    order: item.order.map(|o| o.to_owned()),
                    rocket_launch_product: item
                        .rocket_launch_product
                        .map(|(n, c)| (item_ids.get(n), c)),
                    fuel_category: item.fuel_category.map(|s| s.to_owned()),
                    fuel_value: item.fuel_value,
                    flags: item.flags.map(|f| f.0),
                },
            );
        }

        for (fluid_name, fluid) in deserialized.fluid {
            let id = fluid_ids.get(fluid_name);

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

        std::fs::write(
            format!("preset/{}/preset.json", name),
            serde_json::to_string_pretty(&preset).unwrap(),
        )
        .unwrap();

        preset
    }
}

macro_rules! id {
    ($name:ident) => {
        #[derive(
            Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, serde::Serialize, Debug,
        )]
        pub struct $name(u32);

        impl From<u32> for $name {
            fn from(value: u32) -> Self {
                Self(value)
            }
        }
    };
}

id!(RecipeId);
id!(ItemId);
id!(FluidId);

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Recipe {
    pub id: RecipeId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subgroup: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

impl Recipe {
    pub fn from_raw(
        id: RecipeId,
        raw: json::Recipe,
        item_ids: &FrozenIdGenerator<ItemId>,
        fluid_ids: &FrozenIdGenerator<FluidId>,
    ) -> Self {
        let mut result = Self {
            id,
            name: raw.name.to_owned(),
            category: raw.category.map(|s| s.to_owned()),
            group: raw.group.map(|s| s.to_owned()),
            subgroup: raw.subgroup.map(|s| s.to_owned()),
            order: raw.order.map(|s| s.to_owned()),
            ..Default::default()
        };

        if let Some(inputs) = &raw.ingredients {
            for input in inputs.iter() {
                result.inputs.push(Input::from_raw_recipe_ingredient(
                    input, item_ids, fluid_ids,
                ));
            }
        }
        if let Some(result_name) = &raw.result {
            result.outputs.push(Output::from_raw_name_amount(
                result_name,
                raw.result_count.unwrap(),
                item_ids,
            ));
        }
        if let Some(outputs) = &raw.results {
            for output in outputs.iter() {
                result
                    .outputs
                    .push(Output::from_raw_recipe_result(output, item_ids, fluid_ids));
            }
        }
        if let Some(normal) = &raw.normal {
            if !normal.ingredients.is_empty() {
                for ingredient in normal.ingredients.iter() {
                    result.inputs.push(Input::from_raw_recipe_ingredient(
                        ingredient, item_ids, fluid_ids,
                    ));
                }
            }
            if let Some(results) = &normal.results {
                for output in results.iter() {
                    result
                        .outputs
                        .push(Output::from_raw_recipe_result(output, item_ids, fluid_ids));
                }
            }
        }
        result
    }

    pub fn icon(&self, preset_name: &str) -> PathBuf {
        PathBuf::from(format!("preset/{}/recipe/{}.png", preset_name, self.name))
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Input {
    pub item_or_fluid: ItemOrFluidId,
    pub rate: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalyst_amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_temperature: Option<f32>,
}

impl Input {
    pub fn from_raw_recipe_ingredient(
        raw: &json::RecipeIngredient,
        item_ids: &FrozenIdGenerator<ItemId>,
        fluid_ids: &FrozenIdGenerator<FluidId>,
    ) -> Self {
        Self {
            item_or_fluid: if raw.ty == "item" {
                ItemOrFluidId::Item(item_ids.get(raw.name))
            } else if raw.ty == "fluid" {
                ItemOrFluidId::Fluid(fluid_ids.get(raw.name))
            } else {
                panic!("Unknown ingredient type: {}", raw.ty);
            },
            rate: raw.amount,
            catalyst_amount: raw.catalyst_amount,
            minimum_temperature: raw.minimum_temperature,
            maximum_temperature: raw.maximum_temperature,
        }
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Output {
    pub item_or_fluid: ItemOrFluidId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_min: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_max: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluidbox_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalyst_amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

impl Output {
    pub fn from_raw_recipe_result(
        raw: &json::RecipeResult,
        item_ids: &FrozenIdGenerator<ItemId>,
        fluid_ids: &FrozenIdGenerator<FluidId>,
    ) -> Self {
        Self {
            item_or_fluid: if raw.ty == "item" {
                ItemOrFluidId::Item(item_ids.get(raw.item))
            } else if raw.ty == "fluid" {
                ItemOrFluidId::Fluid(fluid_ids.get(raw.item))
            } else {
                panic!("Unknown result type: {}", raw.ty);
            },
            amount: raw.amount,
            catalyst_amount: raw.catalyst_amount,
            probability: raw.probability,
            amount_min: raw.amount_min,
            amount_max: raw.amount_max,
            fluidbox_index: raw.fluidbox_index,
            temperature: raw.temperature,
        }
    }

    fn from_raw_name_amount(name: &str, amount: f32, item_ids: &FrozenIdGenerator<ItemId>) -> Self {
        Self {
            item_or_fluid: ItemOrFluidId::Item(item_ids.get(name)),
            amount: Some(amount),
            ..Default::default()
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub enum ItemOrFluidId {
    Item(ItemId),
    Fluid(FluidId),
}

impl Default for ItemOrFluidId {
    fn default() -> Self {
        ItemOrFluidId::Item(ItemId::from(u32::MAX))
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Item {
    pub id: ItemId,
    pub name: String,
    pub stack_size: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subgroup: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub hidden: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_launch_product: Option<(ItemId, usize)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_value: Option<Unit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<Flags>>,
}

fn is_false(b: &bool) -> bool {
    !*b
}

// impl Item {
//     pub fn icon(&self, preset_name: &str) -> PathBuf {
//         PathBuf::from(format!("preset/{}/item/{}.png", preset_name, self.name))
//     }
// }

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Fluid {
    pub id: FluidId,
    pub name: String,
}

// impl Fluid {
//     pub fn icon(&self, preset_name: &str) -> PathBuf {
//         PathBuf::from(format!("preset/{}/fluid/{}.png", preset_name, self.name))
//     }
// }
