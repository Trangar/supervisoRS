use rustc_hash::{FxHashMap, FxHashSet};
use supervisors_derive::CustomDeserialize;

#[derive(Debug, CustomDeserialize)]
pub struct RecipeIngredient<'a> {
    pub ty: String,
    pub name: String,
    pub amount: f32,
    pub catalyst_amount: Option<f32>,
    pub minimum_temperature: Option<f32>,
    pub maximum_temperature: Option<f32>,
    pub fluidbox_index: Option<f32>,
    pub ignored_by_stats: Option<usize>,

    #[remaining]
    pub remaining: FxHashMap<String, serde_json::Value>,
}
