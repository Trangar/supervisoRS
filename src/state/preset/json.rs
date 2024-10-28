#![allow(dead_code)]

mod flags;
mod fluid_boxes;
mod int_or_infinite;
mod prerequisites;
mod recipe_ingredient;
mod recipe_result;
mod unit;
mod vec_or_map;

pub use flags::Flags;
pub use fluid_boxes::FluidBoxes;
pub use int_or_infinite::IntOrInfinite;
pub use prerequisites::Prerequisites;
pub use recipe_ingredient::RecipeIngredient;
pub use recipe_result::RecipeResult;
pub use unit::{Unit, UnitType};
pub use vec_or_map::VecOrMap;

use rustc_hash::{FxHashMap, FxHashSet};
use supervisors_derive::CustomDeserialize;

#[derive(Debug, CustomDeserialize)]
pub struct Root<'a> {
    pub item: FxHashMap<&'a str, Item<'a>>,
    pub capsule: FxHashMap<&'a str, Capsule<'a>>,
    pub fluid: FxHashMap<&'a str, Fluid<'a>>,
    pub assembling_machine: FxHashMap<&'a str, Assembler<'a>>,
    pub furnace: FxHashMap<&'a str, Assembler<'a>>,
    pub boiler: FxHashMap<&'a str, Assembler<'a>>,
    pub transport_belt: FxHashMap<&'a str, TransportBelt<'a>>,
    pub technology: FxHashMap<&'a str, Technology<'a>>,

    pub module: FxHashMap<&'a str, Module<'a>>,
    pub recipe: FxHashMap<&'a str, Recipe<'a>>,

    #[remaining]
    pub remaining: FxHashMap<&'a str, FxHashMap<&'a str, NameAndType<'a>>>,
}

#[derive(Debug, CustomDeserialize)]
pub struct Item<'a> {
    pub ty: &'a str,
    pub name: &'a str,
    pub stack_size: usize,
    pub group: Option<&'a str>,
    pub subgroup: Option<&'a str>,
    pub category: Option<&'a str>,
    pub hidden: Option<bool>,
    pub order: Option<&'a str>,
    pub rocket_launch_product: Option<(&'a str, usize)>,
    pub burnt_fuel_result: Option<&'a str>,
    pub burnt_result: Option<&'a str>,
    pub fuel_category: Option<&'a str>,
    pub fuel_value: Option<Unit>,
    pub flags: Option<VecOrMap<Flags>>,

    #[remaining]
    pub remaining: FxHashMap<&'a str, serde_json::Value>,
}

#[derive(Debug, CustomDeserialize)]
pub struct Capsule<'a> {
    pub ty: &'a str,
    pub name: &'a str,
    pub subgroup: Option<&'a str>,
    pub order: Option<&'a str>,
    pub stack_size: usize,
    pub flags: Option<VecOrMap<Flags>>,

    #[remaining]
    pub remaining: FxHashMap<&'a str, serde_json::Value>,
}

#[derive(Debug, CustomDeserialize)]
pub struct Fluid<'a> {
    pub ty: &'a str,
    pub name: &'a str,
    pub default_temperature: Option<f32>,
    pub min_temperature: Option<f32>,
    pub max_temperature: Option<f32>,
    pub gas_temperature: Option<f32>,
    pub auto_barrel: Option<bool>,
    pub hidden: Option<bool>,
    pub heat_capacity: Option<Unit>,
    pub fuel_value: Option<Unit>,
    pub order: Option<&'a str>,
    pub subgroup: Option<&'a str>,
    pub fuel_category: Option<&'a str>,
    pub flags: Option<VecOrMap<Flags>>,

    #[remaining]
    pub remaining: FxHashMap<&'a str, serde_json::Value>,
}

#[derive(Debug, CustomDeserialize)]
pub struct Assembler<'a> {
    pub ty: &'a str,
    pub name: &'a str,
    pub flags: Option<VecOrMap<Flags>>,
    pub crafting_categories: Option<VecOrMap<&'a str>>,
    pub crafting_speed: Option<f32>,
    pub energy_usage: Option<Unit>,
    pub fast_replaceable_group: Option<&'a str>,
    pub mode: Option<&'a str>,
    pub target_temperature: Option<f32>,
    pub module_specification: Option<ModuleSpecification<'a>>,
    pub allowed_effects: Option<VecOrMap<&'a str>>,
    pub fluid_boxes: Option<FluidBoxes<'a>>,
    pub subgroup: Option<&'a str>,
    pub energy_consumption: Option<Unit>,
    pub energy_source: Option<EnergySource<'a>>,
    pub ingredient_count: Option<usize>,
    pub next_upgrade: Option<&'a str>,
    pub fixed_recipe: Option<&'a str>,
    pub order: Option<&'a str>,
    pub module_slots: Option<usize>,

    #[remaining]
    pub remaining: FxHashMap<&'a str, serde_json::Value>,
}

#[derive(Debug, CustomDeserialize)]
pub struct TransportBelt<'a> {
    pub ty: &'a str,
    pub name: &'a str,
    pub subgroup: Option<&'a str>,
    pub flags: Option<VecOrMap<Flags>>,
    pub fast_replaceable_group: Option<&'a str>,
    pub next_upgrade: Option<&'a str>,
    pub speed: f32,
    pub order: Option<&'a str>,

    #[remaining]
    pub remaining: FxHashMap<&'a str, serde_json::Value>,
}

#[derive(Debug, CustomDeserialize)]
pub struct Technology<'a> {
    pub ty: &'a str,
    pub name: &'a str,
    pub hidden: Option<bool>,
    pub order: Option<&'a str>,
    pub prerequisites: Option<Prerequisites<'a>>,
    pub mod_: Option<&'a str>,
    pub enabled: Option<bool>,
    pub max_level: Option<IntOrInfinite>,

    #[remaining]
    pub remaining: FxHashMap<&'a str, serde_json::Value>,
}

#[derive(Debug, CustomDeserialize)]
pub struct Recipe<'a> {
    pub ty: &'a str,
    pub name: &'a str,
    pub category: Option<&'a str>,
    pub group: Option<&'a str>,
    pub subgroup: Option<&'a str>,
    pub order: Option<&'a str>,

    pub main_product: Option<&'a str>,
    pub ingredients: Option<VecOrMap<RecipeIngredient<'a>>>,

    pub result: Option<&'a str>,
    pub result_count: Option<f32>,

    pub results: Option<VecOrMap<RecipeResult<'a>>>,

    pub mod_: Option<&'a str>,
    pub hide_from_player_crafting: Option<bool>,
    pub enabled: Option<bool>,
    pub allow_productivity: Option<bool>,
    pub hidden: Option<bool>,
    pub always_show_made_in: Option<bool>,
    pub normal: Option<EnabledRecipeIngredients<'a>>,
    pub allow_decomposition: Option<bool>,
    pub allow_as_intermediate: Option<bool>,
    pub allow_intermediates: Option<bool>,
    pub energy_required: Option<f32>,
    pub show_amount_in_title: Option<bool>,
    pub flags: Option<VecOrMap<Flags>>,

    #[remaining]
    pub remaining: FxHashMap<&'a str, serde_json::Value>,
}

#[derive(Debug, CustomDeserialize)]
pub struct EnergySource<'a> {
    pub ty: &'a str,
    pub effectivity: Option<f32>,
    pub fuel_inventory_size: Option<f32>,
    pub emissions_per_minute: Option<f32>,
    pub max_temperature: Option<f32>,
    pub min_temperature: Option<f32>,
    pub specific_heat: Option<Unit>,
    pub max_transfer: Option<Unit>,
    pub fuel_categories: Option<VecOrMap<&'a str>>,
    pub fuel_category: Option<&'a str>,
    pub usage_priority: Option<&'a str>,

    #[remaining]
    pub remaining: FxHashMap<&'a str, serde_json::Value>,
}

#[derive(Debug, CustomDeserialize)]
pub struct ModuleSpecification<'a> {
    pub module_slots: u8,

    #[remaining]
    pub remaining: FxHashMap<&'a str, serde_json::Value>,
}

#[derive(Debug, CustomDeserialize)]
pub struct FluidBox<'a> {
    pub ty: Option<&'a str>,

    pub filter: Option<&'a str>,
    pub minimum_temperature: Option<f32>,

    #[remaining]
    pub remaining: FxHashMap<&'a str, serde_json::Value>,
}

#[derive(Debug, CustomDeserialize)]
pub struct NameAndType<'a> {
    pub name: &'a str,
    pub ty: &'a str,

    #[remaining]
    pub remaining: FxHashMap<&'a str, serde_json::Value>,
}

#[derive(Debug, CustomDeserialize)]
pub struct Module<'a> {
    pub ty: &'a str,
    pub name: &'a str,
    pub subgroup: &'a str,
    pub category: &'a str,
    pub tier: u8,
    pub order: &'a str,
    pub stack_size: Option<usize>,
    pub effect: ModuleEffect<'a>,
    pub limitation: Option<Vec<&'a str>>,
    pub limitation_blacklist: Option<Vec<&'a str>>,

    #[remaining]
    pub remaining: FxHashMap<&'a str, serde_json::Value>,
}

#[derive(Debug, CustomDeserialize)]
pub struct ModuleEffect<'a> {
    pub consumption: ModuleEffectBonus<'a>,
    pub pollution: ModuleEffectBonus<'a>,
    pub speed: Option<ModuleEffectBonus<'a>>,
    pub productivity: Option<ModuleEffectBonus<'a>>,

    #[remaining]
    pub remaining: FxHashMap<&'a str, serde_json::Value>,
}

#[derive(Debug, CustomDeserialize)]
pub struct ModuleEffectBonus<'a> {
    pub bonus: f32,

    #[remaining]
    pub remaining: FxHashMap<&'a str, serde_json::Value>,
}

#[derive(Debug, CustomDeserialize)]
pub struct EnabledRecipeIngredients<'a> {
    pub ingredients: Vec<RecipeIngredient<'a>>,
    pub allow_intermediates: Option<bool>,
    pub results: Option<Vec<RecipeResult<'a>>>,
    pub result: Option<&'a str>,
    pub result_count: Option<f32>,
    pub enabled: Option<bool>,
    pub allow_decomposition: Option<bool>,
    pub allow_as_intermediate: Option<bool>,
    pub energy_required: Option<f32>,
    pub show_amount_in_title: Option<bool>,
    pub main_product: Option<&'a str>,

    #[remaining]
    pub remaining: FxHashMap<&'a str, serde_json::Value>,
}
