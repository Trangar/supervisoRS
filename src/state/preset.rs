mod id_generator;
mod json;

use id_generator::{FrozenIdGenerator, IdGenerator};
use instant::Instant;
use itertools::Itertools;
use json::{Flags, Unit};
use rustc_hash::FxHashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Preset {
    pub name: String,
    #[serde(with = "fxhashmap_values")]
    pub recipes: FxHashMap<RecipeId, Recipe>,
    #[serde(with = "fxhashmap_values")]
    pub items: FxHashMap<ItemId, Item>,
    #[serde(with = "fxhashmap_values")]
    pub fluids: FxHashMap<FluidId, Fluid>,
    pub groups: Vec<Group>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Group {
    pub name: String,
    pub rows: Vec<GroupRow>,
}
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GroupRow {
    pub name: String,
    pub items: Vec<ItemId>,
    pub fluids: Vec<FluidId>,
    pub recipes: Vec<RecipeId>,
}

mod fxhashmap_values {
    use itertools::Itertools;
    use rustc_hash::FxHashMap;
    use serde::{ser::SerializeSeq, Deserialize};

    pub fn serialize<S, K, V>(map: &FxHashMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
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

    pub fn deserialize<'de, S, K, V>(deserializer: S) -> Result<FxHashMap<K, V>, S::Error>
    where
        S: serde::de::Deserializer<'de>,
        K: serde::de::Deserialize<'de> + std::hash::Hash + Eq,
        V: serde::de::Deserialize<'de> + Keyable<K>,
    {
        let values = Vec::<V>::deserialize(deserializer)?;
        let mut map = FxHashMap::default();
        for value in values {
            let key = value.key();
            map.insert(key, value);
        }
        Ok(map)
    }

    pub trait Keyable<K> {
        fn key(&self) -> K;
    }
}

impl Preset {
    pub fn load(name: &str) -> Self {
        let start = Instant::now();
        let json_file_name = format!("preset/{}/preset.json", name);

        if std::fs::exists(&json_file_name).unwrap_or(false) {
            let data =
                std::fs::read_to_string(&json_file_name).expect("Failed to read preset.json");
            match serde_json::from_str(&data) {
                Ok(preset) => {
                    println!("Loaded preset.json in {:?}", start.elapsed());
                    return preset;
                }
                Err(e) => {
                    println!("Failed to deserialize preset.json: {:?}", e);
                }
            }
        }

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

        #[derive(Default)]
        struct SubgroupAggregator<'a> {
            items: Vec<(&'a str, ItemId)>,
            fluids: Vec<(&'a str, FluidId)>,
            recipes: Vec<(&'a str, RecipeId)>,
        }

        let mut subgroups = deserialized
            .item_subgroup
            .values()
            .map(|sg| ((sg.group, sg.name), SubgroupAggregator::default()))
            .collect::<FxHashMap<_, _>>();

        let mut preset = Preset {
            name: name.to_string(),
            recipes: FxHashMap::default(),
            items: FxHashMap::default(),
            fluids: FxHashMap::default(),
            groups: Vec::new(),
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
                item_ids.add(result.name);
            } else if result.ty == "fluid" {
                fluid_ids.add(result.name);
            }
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
                .insert(id, Recipe::from_raw(id, &recipe, &item_ids, &fluid_ids));

            if let (Some(group), Some(subgroup), Some(order)) =
                (recipe.group, recipe.subgroup, recipe.order)
            {
                subgroups
                    .get_mut(&(group, subgroup))
                    .unwrap()
                    .recipes
                    .push((order, id));
            }
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
                    hidden: item.hidden.unwrap_or(false),
                    rocket_launch_product: item
                        .rocket_launch_product
                        .map(|(n, c)| (item_ids.get(n), c)),
                    fuel_category: item.fuel_category.map(|s| s.to_owned()),
                    fuel_value: item.fuel_value,
                    flags: item.flags.map(|f| f.0),
                },
            );

            if let (Some(group), Some(subgroup), Some(order)) =
                (item.group, item.subgroup, item.order)
            {
                subgroups
                    .get_mut(&(group, subgroup))
                    .unwrap()
                    .items
                    .push((order, id));
            }
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
            if let (Some(group), Some(subgroup), Some(order)) =
                (fluid.group, fluid.subgroup, fluid.order)
            {
                subgroups
                    .get_mut(&(group, subgroup))
                    .unwrap()
                    .fluids
                    .push((order, id));
            }
        }

        for group in deserialized.item_group.values().sorted_by_key(|g| g.order) {
            let mut group = Group {
                name: group.name.to_owned(),
                rows: Vec::new(),
            };

            for subgroup in deserialized
                .item_subgroup
                .values()
                .filter(|sg| sg.group == group.name)
                .sorted_by_key(|sg| sg.order)
            {
                let Some(sg) = subgroups.get(&(group.name.as_str(), subgroup.name)) else {
                    continue;
                };

                group.rows.push(GroupRow {
                    name: subgroup.name.to_owned(),
                    items: sg
                        .items
                        .iter()
                        .sorted_by_key(|(order, _)| *order)
                        .map(|(_, id)| *id)
                        .collect(),
                    fluids: sg
                        .fluids
                        .iter()
                        .sorted_by_key(|(order, _)| *order)
                        .map(|(_, id)| *id)
                        .collect(),
                    recipes: sg
                        .recipes
                        .iter()
                        .sorted_by_key(|(order, _)| *order)
                        .map(|(_, id)| *id)
                        .collect(),
                });
            }

            preset.groups.push(group);
        }

        std::fs::write(
            json_file_name,
            serde_json::to_string_pretty(&preset).unwrap(),
        )
        .unwrap();

        preset
    }
}

macro_rules! id {
    ($name:ident) => {
        #[derive(
            Copy,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            Default,
            serde::Serialize,
            serde::Deserialize,
            Debug,
        )]
        #[serde(transparent)]
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

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Recipe {
    pub id: RecipeId,
    pub name: String,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

impl fxhashmap_values::Keyable<RecipeId> for Recipe {
    fn key(&self) -> RecipeId {
        self.id
    }
}

impl Recipe {
    pub fn from_raw(
        id: RecipeId,
        raw: &json::Recipe,
        item_ids: &FrozenIdGenerator<ItemId>,
        fluid_ids: &FrozenIdGenerator<FluidId>,
    ) -> Self {
        let mut result = Self {
            id,
            name: raw.name.to_owned(),
            ..Default::default()
        };

        if let Some(inputs) = &raw.ingredients {
            for input in inputs.iter() {
                result.inputs.push(Input::from_raw_recipe_ingredient(
                    input, item_ids, fluid_ids,
                ));
            }
        }
        if let Some(outputs) = &raw.results {
            for output in outputs.iter() {
                result
                    .outputs
                    .push(Output::new(output, item_ids, fluid_ids));
            }
        }

        result
    }

    // pub fn icon(&self, preset_name: &str) -> PathBuf {
    //     PathBuf::from(format!("preset/{}/recipe/{}.png", preset_name, self.name))
    // }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Input {
    pub item_or_fluid: ItemOrFluidId,
    pub rate: f32,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub catalyst_amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub minimum_temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
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

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Output {
    pub item_or_fluid: ItemOrFluidId,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub probability: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub amount_min: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub amount_max: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub fluidbox_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub catalyst_amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub temperature: Option<f32>,
}

impl Output {
    pub fn new(
        raw: &json::RecipeResult,
        item_ids: &FrozenIdGenerator<ItemId>,
        fluid_ids: &FrozenIdGenerator<FluidId>,
    ) -> Self {
        Self {
            item_or_fluid: if raw.ty == "item" {
                ItemOrFluidId::Item(item_ids.get(raw.name))
            } else if raw.ty == "fluid" {
                ItemOrFluidId::Fluid(fluid_ids.get(raw.name))
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
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum ItemOrFluidId {
    Item(ItemId),
    Fluid(FluidId),
}

impl Default for ItemOrFluidId {
    fn default() -> Self {
        ItemOrFluidId::Item(ItemId::from(u32::MAX))
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Item {
    pub id: ItemId,
    pub name: String,
    pub stack_size: usize,
    #[serde(skip_serializing_if = "is_false", default)]
    pub hidden: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub rocket_launch_product: Option<(ItemId, usize)>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub fuel_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub fuel_value: Option<Unit>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub flags: Option<Vec<Flags>>,
}

impl fxhashmap_values::Keyable<ItemId> for Item {
    fn key(&self) -> ItemId {
        self.id
    }
}

fn is_false(b: &bool) -> bool {
    !*b
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Fluid {
    pub id: FluidId,
    pub name: String,
}

impl fxhashmap_values::Keyable<FluidId> for Fluid {
    fn key(&self) -> FluidId {
        self.id
    }
}
