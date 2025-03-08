mod id_generator;
mod json;

use id_generator::IdGenerator;
use instant::Instant;
use itertools::Itertools;

use rustc_hash::FxHashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Preset {
    pub name: String,
    pub groups: Vec<Group>,
    pub items: FxHashMap<ItemId, Item>,
    pub fluids: FxHashMap<FluidId, Fluid>,
    pub recipes: FxHashMap<RecipeId, Recipe>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Group {
    pub name: String,
    pub order: String,
    pub rows: Vec<GroupRow>,
}
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GroupRow {
    pub name: String,
    pub order: String,
    pub items: Vec<ItemId>,
    pub fluids: Vec<FluidId>,
    pub recipes: Vec<RecipeId>,
}

impl Preset {
    pub fn load(name: &str) -> Preset {
        let start = Instant::now();
        let json_file_name = format!("preset/{}/preset.json", name);

        if std::fs::exists(&json_file_name).unwrap_or(false) {
            let data =
                std::fs::read_to_string(&json_file_name).expect("Failed to read preset.json");
            let data = data.leak::<'static>();
            match serde_json::from_str(data) {
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
        let data = data.leak::<'static>();

        let deserialized = serde_json::from_str::<json::Root>(data)
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
            groups: Vec::new(),
            items: FxHashMap::default(),
            fluids: FxHashMap::default(),
            recipes: FxHashMap::default(),
        };

        let item_ids = IdGenerator::<ItemId>::from_iter(deserialized.item.keys().cloned());
        let fluid_ids = IdGenerator::<FluidId>::from_iter(deserialized.fluid.keys().cloned());
        let recipe_ids = IdGenerator::<RecipeId>::from_iter(deserialized.recipe.keys().cloned());

        for (name, _item) in &deserialized.item {
            let id = item_ids.get(name);
            preset.items.insert(
                id,
                Item {
                    id,
                    name: name.clone(),
                },
            );
        }

        for (name, _fluid) in &deserialized.fluid {
            let id = fluid_ids.get(name);
            preset.fluids.insert(
                id,
                Fluid {
                    id,
                    name: name.clone(),
                },
            );
        }

        for (name, _recipe) in &deserialized.recipe {
            let id = recipe_ids.get(name);
            preset.recipes.insert(
                id,
                Recipe {
                    id,
                    name: name.clone(),
                },
            );
        }

        // TODO: Optimize this
        // I'm pretty sure we can do this in 1 loop for each entry in `json` and using an intermediate format
        for (group_name, group) in &deserialized.item_group {
            let mut group = Group {
                name: group_name.clone(),
                order: group.order.clone(),
                rows: Vec::new(),
            };
            for (subgroup_name, subgroup) in &deserialized.item_subgroup {
                if &subgroup.group != group_name {
                    continue;
                }
                group.rows.push(GroupRow {
                    name: subgroup_name.clone(),
                    order: subgroup.order.clone(),
                    fluids: deserialized
                        .fluid
                        .values()
                        .filter(|f| f.subgroup.as_ref() == Some(subgroup_name))
                        .sorted_by_key(|f| &f.order)
                        .map(|f| fluid_ids.get(&f.name))
                        .collect(),
                    items: deserialized
                        .item
                        .values()
                        .filter(|i| i.subgroup.as_ref() == Some(subgroup_name))
                        .sorted_by_key(|i| &i.order)
                        .map(|i| item_ids.get(&i.name))
                        .collect(),
                    recipes: deserialized
                        .recipe
                        .values()
                        .filter(|r| r.subgroup.as_ref() == Some(subgroup_name))
                        .sorted_by_key(|r| &r.order)
                        .map(|r| recipe_ids.get(&r.name))
                        .collect(),
                });
            }
            group.rows.sort_by_key(|r| r.order.clone());
            println!("Group {:?} has rows: ", group.name);
            for row in &group.rows {
                println!(
                    "  {} ({}): {} items, {} fluids, {} recipes",
                    row.name,
                    row.order,
                    row.items.len(),
                    row.fluids.len(),
                    row.recipes.len()
                );
            }
            preset.groups.push(group);
        }
        preset.groups.sort_by_key(|g| g.order.clone());

        std::fs::write(
            json_file_name,
            serde_json::to_string_pretty(&preset).unwrap(),
        )
        .unwrap();

        preset
    }

    pub(crate) fn icon_for_fluid(&self, fluid: &Fluid) -> String {
        format!(
            "preset/{}/script-output/fluid/{}.png",
            self.name, fluid.name
        )
    }

    pub(crate) fn icon_for_item(&self, item: &Item) -> String {
        format!("preset/{}/script-output/item/{}.png", self.name, item.name)
    }

    pub(crate) fn icon_for_recipe(&self, recipe: &Recipe) -> String {
        format!(
            "preset/{}/script-output/recipe/{}.png",
            self.name, recipe.name
        )
    }

    pub(crate) fn icon_for_tab_group(&self, g: &Group) -> String {
        format!(
            "preset/{}/script-output/item-group/{}.png",
            self.name, g.name
        )
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Recipe {
    pub id: RecipeId,
    pub name: String,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Item {
    pub id: ItemId,
    pub name: String,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Fluid {
    pub id: FluidId,
    pub name: String,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct ItemId(u64);
impl From<u64> for ItemId {
    fn from(u: u64) -> Self {
        Self(u)
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct FluidId(u64);
impl From<u64> for FluidId {
    fn from(u: u64) -> Self {
        Self(u)
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct RecipeId(u64);
impl From<u64> for RecipeId {
    fn from(u: u64) -> Self {
        Self(u)
    }
}
