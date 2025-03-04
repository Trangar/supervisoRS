mod id_generator;
mod json;

use id_generator::{FrozenIdGenerator, IdGenerator};
use instant::Instant;
use noisy_float::types::R32;
use rustc_hash::FxHashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Preset {
    pub name: String,
    // pub recipes: FxHashMap<RecipeId, Recipe>,
    // pub items: FxHashMap<ItemId, Item>,
    // pub fluids: FxHashMap<FluidId, Fluid>,
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

        todo!();

        // let mut preset = Preset {
        //     name: name.to_string(),
        //     recipes: FxHashMap::default(),
        //     items: FxHashMap::default(),
        //     fluids: FxHashMap::default(),
        // };

        // let mut recipe_ids = IdGenerator::<RecipeId>::default();
        // for recipe_name in deserialized.recipe.keys() {
        //     recipe_ids.add(recipe_name);
        // }
        // let recipe_ids = recipe_ids.freeze();

        // let mut item_ids = IdGenerator::<ItemId>::default();
        // for item_name in deserialized.item.keys() {
        //     item_ids.add(item_name);
        // }
        // for capsule_name in deserialized.capsule.keys() {
        //     item_ids.add(capsule_name);
        // }
        // for module in deserialized.module.keys() {
        //     item_ids.add(module);
        // }
        // let item_ids = item_ids.freeze();

        // let mut fluid_ids = IdGenerator::<FluidId>::default();
        // for fluid_name in deserialized.fluid.keys() {
        //     fluid_ids.add(fluid_name);
        // }
        // let fluid_ids = fluid_ids.freeze();

        // for (recipe_name, recipe) in deserialized.recipe {
        //     let id = recipe_ids.get(recipe_name);

        //     preset
        //         .recipes
        //         .insert(id, Recipe::from_raw(id, recipe, &item_ids, &fluid_ids));
        // }

        // for (item_name, item) in deserialized.item {
        //     let id = item_ids.get(item_name);

        //     if preset.items.contains_key(&id) {
        //         panic!(
        //             "Duplicate item id: {:?}, {} - {}",
        //             id, preset.items[&id].name, item_name
        //         );
        //     }
        //     preset.items.insert(
        //         id,
        //         Item {
        //             id,
        //             name: item_name.to_string(),
        //             ..Default::default()
        //         },
        //     );
        // }

        // for (fluid_name, fluid) in deserialized.fluid {
        //     let id = fluid_ids.get(fluid_name);

        //     if preset.fluids.contains_key(&id) {
        //         panic!(
        //             "Duplicate fluid id: {:?}, {} - {}",
        //             id, preset.fluids[&id].name, fluid_name
        //         );
        //     }
        //     preset.fluids.insert(
        //         id,
        //         Fluid {
        //             id,
        //             name: fluid_name.to_string(),
        //         },
        //     );
        // }

        // dbg!(preset)
    }
}

// macro_rules! id {
//     ($name:ident) => {
//         #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
//         pub struct $name(u32);

//         impl std::fmt::Debug for $name {
//             fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//                 write!(f, "{}({})", stringify!($name), self.0)
//             }
//         }

//         impl From<u32> for $name {
//             fn from(id: u32) -> Self {
//                 $name(id)
//             }
//         }
//     };
// }

// id!(RecipeId);
// id!(ItemId);
// id!(FluidId);

// #[derive(Clone, Debug, Default)]
// pub struct Recipe {
//     pub id: RecipeId,
//     pub name: String,
//     pub category: Option<String>,
//     pub group: Option<String>,
//     pub subgroup: Option<String>,
//     pub order: Option<String>,
//     pub inputs: Vec<Input>,
//     pub outputs: Vec<Output>,
// }

// impl Recipe {
//     pub fn from_raw(
//         id: RecipeId,
//         raw: json::Recipe,
//         item_ids: &FrozenIdGenerator<ItemId>,
//         fluid_ids: &FrozenIdGenerator<FluidId>,
//     ) -> Self {
//         let mut result = Self {
//             id,
//             name: raw.name.to_owned(),
//             category: raw.category.map(|s| s.to_owned()),
//             group: raw.group.map(|s| s.to_owned()),
//             subgroup: raw.subgroup.map(|s| s.to_owned()),
//             order: raw.order.map(|s| s.to_owned()),
//             ..Default::default()
//         };

//         if let Some(inputs) = &raw.ingredients {
//             for input in inputs.iter() {
//                 result.inputs.push(Input::from_raw_recipe_ingredient(
//                     input, item_ids, fluid_ids,
//                 ));
//             }
//         }
//         if let Some(result_name) = &raw.result {
//             println!("{raw:?}");
//             result.outputs.push(Output::from_raw_name_amount(
//                 result_name,
//                 raw.result_count,
//                 item_ids,
//             ));
//         }
//         if let Some(outputs) = &raw.results {
//             for output in outputs.iter() {
//                 result
//                     .outputs
//                     .push(Output::from_raw_recipe_result(output, item_ids, fluid_ids));
//             }
//         }
//         if let Some(normal) = &raw.normal {
//             if !normal.ingredients.is_empty() {
//                 for ingredient in normal.ingredients.iter() {
//                     result.inputs.push(Input::from_raw_recipe_ingredient(
//                         ingredient, item_ids, fluid_ids,
//                     ));
//                 }
//             }
//         }
//         result
//     }

//     pub fn icon(&self, preset_name: &str) -> PathBuf {
//         PathBuf::from(format!("preset/{}/recipe/{}.png", preset_name, self.name))
//     }
// }

// #[derive(Clone, Debug, Default)]
// pub struct Input {
//     pub item_or_fluid: ItemOrFluidId,
//     pub rate: f32,
//     pub catalyst_amount: Option<f32>,
//     pub minimum_temperature: Option<f32>,
//     pub maximum_temperature: Option<f32>,
// }

// impl Input {
//     pub fn from_raw_recipe_ingredient(
//         raw: &json::RecipeIngredient,
//         item_ids: &FrozenIdGenerator<ItemId>,
//         fluid_ids: &FrozenIdGenerator<FluidId>,
//     ) -> Self {
//         Self {
//             item_or_fluid: if raw.ingredient_type == "item" {
//                 ItemOrFluidId::Item(item_ids.get(raw.name))
//             } else if raw.ingredient_type == "fluid" {
//                 ItemOrFluidId::Fluid(fluid_ids.get(raw.name))
//             } else {
//                 panic!("Unknown ingredient type: {}", raw.ingredient_type);
//             },
//             rate: raw.amount,
//             catalyst_amount: raw.catalyst_amount,
//             minimum_temperature: raw.minimum_temperature,
//             maximum_temperature: raw.maximum_temperature,
//         }
//     }
// }

// #[derive(Clone, Debug, Default)]
// pub struct Output {
//     pub item_or_fluid: ItemOrFluidId,
//     pub amount: f32,
//     pub probability: Option<f32>,
//     pub amount_min: Option<f32>,
//     pub amount_max: Option<f32>,
//     pub fluidbox_index: Option<u8>,
//     pub catalyst_amount: Option<f32>,
//     pub temperature: Option<f32>,
// }

// impl Output {
//     pub fn from_raw_recipe_result(
//         raw: &json::RecipeResult,
//         item_ids: &FrozenIdGenerator<ItemId>,
//         fluid_ids: &FrozenIdGenerator<FluidId>,
//     ) -> Self {
//         Self {
//             item_or_fluid: if raw.result_type == "item" {
//                 ItemOrFluidId::Item(item_ids.get(raw.name))
//             } else if raw.result_type == "fluid" {
//                 ItemOrFluidId::Fluid(fluid_ids.get(raw.name))
//             } else {
//                 panic!("Unknown result type: {}", raw.result_type);
//             },
//             amount: raw.amount,
//             catalyst_amount: raw.catalyst_amount,
//             probability: raw.probability,
//             amount_min: raw.amount_min,
//             amount_max: raw.amount_max,
//             fluidbox_index: raw.fluidbox_index,
//             temperature: raw.temperature,
//         }
//     }

//     fn from_raw_name_amount(name: &str, amount: f32, item_ids: &FrozenIdGenerator<ItemId>) -> Self {
//         Self {
//             item_or_fluid: ItemOrFluidId::Item(item_ids.get(name)),
//             amount,
//             ..Default::default()
//         }
//     }
// }

// #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum ItemOrFluidId {
//     Item(ItemId),
//     Fluid(FluidId),
// }

// impl Default for ItemOrFluidId {
//     fn default() -> Self {
//         ItemOrFluidId::Item(ItemId::from(u32::MAX))
//     }
// }

// #[derive(Clone, Debug, Default)]
// pub struct Item {
//     pub id: ItemId,
//     pub name: String,
// }

// impl Item {
//     pub fn icon(&self, preset_name: &str) -> PathBuf {
//         PathBuf::from(format!("preset/{}/item/{}.png", preset_name, self.name))
//     }
// }

// #[derive(Clone, Debug, Default)]
// pub struct Fluid {
//     pub id: FluidId,
//     pub name: String,
// }

// impl Fluid {
//     pub fn icon(&self, preset_name: &str) -> PathBuf {
//         PathBuf::from(format!("preset/{}/fluid/{}.png", preset_name, self.name))
//     }
// }
