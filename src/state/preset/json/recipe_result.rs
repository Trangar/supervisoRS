use rustc_hash::{FxHashMap, FxHashSet};
use supervisors_derive::CustomDeserialize;

#[derive(Debug, CustomDeserialize)]
pub struct RecipeResult<'a> {
    pub ty: String,
    pub name: String,
    pub amount: Option<f32>,
    pub probability: Option<f32>,
    pub amount_min: Option<f32>,
    pub amount_max: Option<f32>,
    pub fluidbox_index: Option<u8>,
    pub catalyst_amount: Option<f32>,
    pub temperature: Option<f32>,
    pub ignored_by_stats: Option<usize>,
    pub ignored_by_productivity: Option<usize>,
    pub percent_spoiled: Option<f32>,

    #[remaining]
    pub remaining: FxHashMap<String, serde_json::Value>,
}
