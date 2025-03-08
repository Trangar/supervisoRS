#![allow(dead_code)]

// mod flags;
// mod fluid_boxes;
// mod int_or_infinite;
// mod prerequisites;
// mod recipe_ingredient;
// mod recipe_result;
// mod unit;
// mod vec_or_map;

// pub use flags::Flags;
// pub use recipe_ingredient::RecipeIngredient;
// pub use recipe_result::RecipeResult;
// pub use unit::Unit;

mod fxhashmap_values;

use rustc_hash::FxHashMap;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Root {
    pub item: FxHashMap<String, Item>,
    pub fluid: FxHashMap<String, Fluid>,
    pub recipe: FxHashMap<String, Recipe>,

    #[serde(rename = "item-group")]
    pub item_group: FxHashMap<String, ItemGroup>,
    #[serde(rename = "item-subgroup")]
    pub item_subgroup: FxHashMap<String, ItemSubgroup>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ItemGroup {
    pub name: String,
    pub order: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ItemSubgroup {
    pub name: String,
    pub order: String,
    pub group: String,
}

pub trait Groupable {
    fn group(&self) -> Option<&String>;
    fn subgroup(&self) -> Option<&String>;
    fn order(&self) -> Option<&String>;
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Item {
    pub name: String,
    pub group: Option<String>,
    pub subgroup: Option<String>,
    pub order: Option<String>,
    pub category: Option<String>,
    // pub hidden: Option<bool>,
    // pub rocket_launch_product: Option<(String, usize)>,
    // pub burnt_fuel_result: Option<String>,
    // pub burnt_result: Option<String>,
    // pub fuel_category: Option<String>,
    // pub fuel_value: Option<Unit>,
    // pub flags: Option<VecOrMap<Flags>>,

    // #[remaining]
    // pub remaining: FxHashMap<String, serde_json::Value>,
}

impl Groupable for Item {
    fn group(&self) -> Option<&String> {
        self.group.as_ref()
    }
    fn subgroup(&self) -> Option<&String> {
        self.subgroup.as_ref()
    }
    fn order(&self) -> Option<&String> {
        self.order.as_ref()
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Fluid {
    pub name: String,
    pub group: Option<String>,
    pub subgroup: Option<String>,
    pub order: Option<String>,
    // pub default_temperature: Option<f32>,
    // pub min_temperature: Option<f32>,
    // pub max_temperature: Option<f32>,
    // pub gas_temperature: Option<f32>,
    // pub auto_barrel: Option<bool>,
    // pub hidden: Option<bool>,
    // pub heat_capacity: Option<Unit>,
    // pub fuel_value: Option<Unit>,
    // pub fuel_category: Option<String>,
    // pub flags: Option<VecOrMap<Flags>>,

    // #[remaining]
    // pub remaining: FxHashMap<String, serde_json::Value>,
}
impl Groupable for Fluid {
    fn group(&self) -> Option<&String> {
        self.group.as_ref()
    }
    fn subgroup(&self) -> Option<&String> {
        self.subgroup.as_ref()
    }
    fn order(&self) -> Option<&String> {
        self.order.as_ref()
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Recipe {
    pub name: String,
    pub category: Option<String>,
    pub group: Option<String>,
    pub subgroup: Option<String>,
    pub order: Option<String>,
    // pub main_product: Option<String>,
    // pub ingredients: Option<VecOrMap<RecipeIngredient>>,

    // pub result: Option<String>,
    // pub result_count: Option<f32>,
    // pub results: Option<VecOrMap<RecipeResult>>,

    // pub mod_: Option<String>,
    // pub hide_from_player_crafting: Option<bool>,
    // pub enabled: Option<bool>,
    // pub allow_productivity: Option<bool>,
    // pub hidden: Option<bool>,
    // pub always_show_made_in: Option<bool>,
    // pub allow_decomposition: Option<bool>,
    // pub allow_as_intermediate: Option<bool>,
    // pub allow_intermediates: Option<bool>,
    // pub energy_required: Option<f32>,
    // pub show_amount_in_title: Option<bool>,
    // pub flags: Option<VecOrMap<Flags>>,

    // #[remaining]
    // pub remaining: FxHashMap<String, serde_json::Value>,
}

impl Groupable for Recipe {
    fn group(&self) -> Option<&String> {
        self.group.as_ref()
    }
    fn subgroup(&self) -> Option<&String> {
        self.subgroup.as_ref()
    }
    fn order(&self) -> Option<&String> {
        self.order.as_ref()
    }
}

// macro_rules! id {
//     ($name:ident) => {
//         #[derive(
//             Copy,
//             Clone,
//             PartialEq,
//             Eq,
//             PartialOrd,
//             Ord,
//             Hash,
//             Default,
//             serde::Serialize,
//             serde::Deserialize,
//         )]
//         #[serde(transparent)]
//         pub struct $name(pub u32);

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
