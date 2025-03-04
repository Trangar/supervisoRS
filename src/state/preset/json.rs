#![allow(dead_code)]

mod not_used;

use std::ops::Deref;

use not_used::NotUsed;

use rustc_hash::FxHashMap;

#[derive(Debug, Default)]
pub struct Root<'a> {
    pub item: FxHashMap<&'a str, Item<'a>>,
    pub capsule: FxHashMap<&'a str, Capsule<'a>>,
    pub fluid: FxHashMap<&'a str, Fluid<'a>>,
    pub furnace: FxHashMap<&'a str, Furnace<'a>>,
    pub transport_belt: FxHashMap<&'a str, TransportBelt<'a>>,
    pub recipe: FxHashMap<&'a str, Recipe<'a>>,
    pub module: FxHashMap<&'a str, Module<'a>>,

    pub unused: FxHashMap<&'a str, Vec<serde_json::Value>>,
}

impl<'de> serde::Deserialize<'de> for Root<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Root<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "A key-value map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut root = Root::default();
                loop {
                    let key: Option<&'de str> = map.next_key()?;
                    let Some(key) = key else { break };
                    match key {
                        "item" => root.item = map.next_value()?,
                        "capsule" => root.capsule = map.next_value()?,
                        "fluid" => root.fluid = map.next_value()?,
                        "furnace" => root.furnace = map.next_value()?,
                        "transport-belt" => root.transport_belt = map.next_value()?,
                        "recipe" => root.recipe = map.next_value()?,
                        "module" => root.module = map.next_value()?,
                        x => {
                            root.unused.entry(x).or_default().push(map.next_value()?);
                        }
                    }
                }

                Ok(root)
            }
        }

        deserializer.deserialize_map(Visitor)
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct Module<'a> {
    #[serde(rename = "type")]
    pub module_type: &'a str,
    pub name: &'a str,
    pub subgroup: &'a str,
    pub category: &'a str, // TODO: Enum
    pub tier: u8,
    pub order: &'a str,
    pub stack_size: usize,
    pub effect: ModuleEffect,
    #[serde(default)]
    pub limitation: Option<Vec<&'a str>>,
    #[serde(default)]
    pub limitation_blacklist: Option<Vec<&'a str>>,

    #[serde(default)]
    limitation_message_key: Option<&'a str>,
    #[serde(default)]
    localised_name: NotUsed,
    #[serde(default)]
    localised_description: NotUsed,
    #[serde(default)]
    icon: NotUsed,
    #[serde(default)]
    icon_size: NotUsed,
    #[serde(default)]
    icon_mipmaps: NotUsed,
    #[serde(default)]
    beacon_tint: NotUsed,
    #[serde(default)]
    art_style: NotUsed,
    #[serde(default)]
    requires_beacon_alt_mode: NotUsed,
}

#[derive(serde::Deserialize, Debug)]
pub struct ModuleEffect {
    pub consumption: ModuleEffectBonus,
    pub pollution: ModuleEffectBonus,
    #[serde(default)]
    pub speed: Option<ModuleEffectBonus>,
    #[serde(default)]
    pub productivity: Option<ModuleEffectBonus>,
}

#[derive(serde::Deserialize, Debug)]
pub struct ModuleEffectBonus {
    pub bonus: f32,
}

#[derive(serde::Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct Recipe<'a> {
    #[serde(rename = "type")]
    pub recipe_type: &'a str,
    pub name: &'a str,
    pub result_count: f32,
    pub category: &'a str,
    pub group: &'a str,
    pub subgroup: &'a str,
    pub order: &'a str,
    pub ingredients: Option<VecOrMap<RecipeIngredient<'a>>>,
    // pub main_product: Option<&'a str>,
    // #[serde(default)]
    // #[serde(default)]
    // #[serde(default)]
    // pub result: Option<&'a str>,
    // #[serde(default, rename = "mod")]
    // pub mod_: Option<&'a str>,
    // #[serde(default)]
    // pub hide_from_player_crafting: Option<bool>,
    // #[serde(default)]
    // pub results: Option<VecOrMap<RecipeResult<'a>>>,
    // #[serde(default)]
    // pub enabled: Option<bool>,
    // #[serde(default)]
    // pub allow_productivity: Option<bool>,
    // #[serde(default)]
    // pub hidden: Option<bool>,
    // pub always_show_made_in: bool,
    // #[serde(default)]
    // pub normal: Option<EnabledRecipeIngredients<'a>>,
    // #[serde(default)]
    // pub allow_decomposition: Option<bool>,
    // #[serde(default)]
    // pub allow_as_intermediate: Option<bool>,
    // #[serde(default)]
    // pub show_amount_in_title: Option<bool>,
    // #[serde(default)]
    // pub flags: Option<VecOrMap<&'a str>>,

    // #[serde(default)]
    // localised_name: NotUsed,
    // #[serde(default)]
    // localized_name: NotUsed,
    // #[serde(default)]
    // localized_description: NotUsed,
    // #[serde(default)]
    // localised_description: NotUsed,
    // #[serde(default)]
    // expensive: Option<EnabledRecipeIngredients<'a>>,
    // #[serde(default)]
    // emissions_multiplier: Option<f32>,
    // #[serde(default)]
    // requester_paste_multiplier: Option<usize>,
    // #[serde(default)]
    // always_show_products: bool,
    // #[serde(default)]
    // energy_required: Option<f32>,
    // #[serde(default)]
    // icon: NotUsed,
    // #[serde(default)]
    // icons: NotUsed,
    // #[serde(default)]
    // icon_size: NotUsed,
    // #[serde(default)]
    // icon_mipmaps: NotUsed,
    // #[serde(default)]
    // crafting_machine_tint: NotUsed,
    // #[serde(default)]
    // hide_from_stats: Option<bool>,
    // #[serde(default)]
    // allow_intermediates: Option<bool>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct EnabledRecipeIngredients<'a> {
    pub ingredients: Vec<RecipeIngredient<'a>>,
    #[serde(default)]
    pub allow_intermediates: Option<bool>,
    #[serde(default)]
    pub results: Option<Vec<RecipeResult<'a>>>,
    #[serde(default)]
    pub result: Option<&'a str>,
    #[serde(default)]
    pub result_count: Option<f32>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub allow_decomposition: Option<bool>,
    #[serde(default)]
    pub allow_as_intermediate: Option<bool>,
    #[serde(default)]
    pub energy_required: Option<f32>,
    #[serde(default)]
    pub show_amount_in_title: Option<bool>,
    #[serde(default)]
    pub main_product: Option<&'a str>,
    #[serde(default)]
    always_show_made_in: Option<bool>,
    #[serde(default)]
    hidden: Option<bool>,
    #[serde(default)]
    requester_paste_multiplier: Option<usize>,
    #[serde(default)]
    always_show_products: Option<bool>,
    #[serde(default)]
    hide_from_player_crafting: Option<bool>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct RecipeIngredient<'a> {
    #[serde(rename = "type")]
    pub ingredient_type: &'a str,
    pub name: &'a str,
    pub amount: f32,
    #[serde(default)]
    pub catalyst_amount: Option<f32>,
    #[serde(default)]
    pub minimum_temperature: Option<f32>,
    #[serde(default)]
    pub maximum_temperature: Option<f32>,
    #[serde(default)]
    pub fluidbox_index: Option<u8>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct RecipeResult<'a> {
    #[serde(rename = "type")]
    pub result_type: &'a str,
    pub name: &'a str,
    pub amount: f32,

    #[serde(default)]
    pub probability: Option<f32>,
    #[serde(default)]
    pub amount_min: Option<f32>,
    #[serde(default)]
    pub amount_max: Option<f32>,
    #[serde(default)]
    pub fluidbox_index: Option<u8>,
    #[serde(default)]
    pub catalyst_amount: Option<f32>,
    #[serde(default)]
    pub temperature: Option<f32>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct Capsule<'a> {
    #[serde(rename = "type")]
    pub capsule_type: &'a str,
    pub name: &'a str,
}

#[derive(serde::Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct Item<'a> {
    #[serde(rename = "type")]
    pub item_type: &'a str,
    pub name: &'a str,
    pub stack_size: usize,
    #[serde(default)]
    pub group: Option<&'a str>,
    #[serde(default)]
    pub subgroup: Option<&'a str>,
    #[serde(default)]
    pub category: Option<&'a str>,
    #[serde(default)]
    pub hidden: Option<bool>,
    #[serde(default)]
    pub order: Option<&'a str>,
    #[serde(default)]
    pub parameter: Option<bool>,

    #[serde(default)]
    pub rocket_launch_product: Option<(&'a str, usize)>,

    #[serde(default)]
    pub burnt_fuel_result: Option<&'a str>,
    #[serde(default)]
    pub burnt_result: Option<&'a str>,

    #[serde(default)]
    icon: Option<&'a str>,
    #[serde(default)]
    icon_gfx: Option<&'a str>,
    #[serde(default)]
    icon_letter: Option<&'a str>,
    #[serde(default)]
    flags: Option<VecOrMap<&'a str>>,
    #[serde(default)]
    icon_mipmaps: Option<u8>,
    #[serde(default)]
    place_as_tile: Option<NotUsed>,
    #[serde(default)]
    fuel_value: Option<Unit>,
    #[serde(default)]
    fuel_category: Option<&'a str>, // TODO: Enum "chemical"
    #[serde(default)]
    fuel_acceleration_multiplier: Option<f32>,
    #[serde(default)]
    acceleration_multiplier: Option<f32>,
    #[serde(default)]
    top_speed_multiplier: Option<f32>,
    #[serde(default)]
    fuel_top_speed_multiplier: Option<f32>,
    #[serde(default)]
    fuel_emissions_multiplier: Option<f32>,
    #[serde(default)]
    dark_background_icon: Option<&'a str>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct Fluid<'a> {
    #[serde(rename = "type")]
    pub fluid_type: &'a str,
    pub name: &'a str,
    pub icon: &'a str,
    pub icon_size: f32,
    pub default_temperature: f32,

    #[serde(default)]
    pub min_temperature: Option<f32>,
    #[serde(default)]
    pub max_temperature: Option<f32>,
    #[serde(default)]
    pub gas_temperature: Option<f32>,
    #[serde(default)]
    pub auto_barrel: Option<bool>,
    #[serde(default)]
    pub hidden: Option<bool>,
    #[serde(default)]
    pub heat_capacity: Option<Unit>,
    #[serde(default)]
    pub fuel_value: Option<Unit>,
    #[serde(default)]
    pub order: Option<&'a str>,
    #[serde(default)]
    pub subgroup: Option<&'a str>,
    #[serde(default)]
    pub fuel_category: Option<&'a str>,

    base_color: NotUsed,
    flow_color: NotUsed,

    #[serde(default)]
    icon_mipmaps: Option<u8>,
    #[serde(default)]
    icons: Option<NotUsed>,
    #[serde(default)]
    emissions_multiplier: Option<f32>,
    #[serde(default)]
    pressure_to_speed_ratio: Option<f32>,
    #[serde(default)]
    flow_to_energy_ratio: Option<f32>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct Furnace<'a> {
    #[serde(rename = "type")]
    pub furnace_type: &'a str,
    pub name: &'a str,
    pub flags: VecOrMap<&'a str>,
    pub crafting_categories: VecOrMap<&'a str>,
    pub crafting_speed: f32,
    pub energy_usage: Unit,
    pub fast_replaceable_group: &'a str,

    #[serde(default)]
    pub module_specification: Option<ModuleSpecification>,
    #[serde(default)]
    pub allowed_effects: Option<VecOrMap<&'a str>>,
    #[serde(default)]
    pub fluid_boxes: Option<FluidBoxes<'a>>,
    #[serde(default)]
    pub subgroup: Option<&'a str>,

    icon: NotUsed,
    icon_size: NotUsed,
    minable: NotUsed,
    result_inventory_size: NotUsed,
    source_inventory_size: NotUsed,
    max_health: NotUsed,
    corpse: NotUsed,
    dying_explosion: NotUsed,
    collision_box: NotUsed,
    selection_box: NotUsed,
    #[serde(default)]
    animation: NotUsed,
    vehicle_impact_sound: NotUsed,
    #[serde(default)]
    working_sound: NotUsed,
    energy_source: NotUsed,
    #[serde(default)]
    water_reflection: NotUsed,
    collision_mask: NotUsed,

    #[serde(default)]
    localised_description: NotUsed,
    #[serde(default)]
    open_sound: NotUsed,
    #[serde(default)]
    close_sound: NotUsed,
    #[serde(default)]
    show_recipe_icon: NotUsed,
    #[serde(default)]
    damaged_trigger_effect: NotUsed,
    #[serde(default)]
    icon_mipmaps: NotUsed,
    #[serde(default)]
    resistances: NotUsed,
    #[serde(default)]
    scale_entity_info_icon: NotUsed,
    #[serde(default)]
    return_ingredients_on_change: NotUsed,
    #[serde(default)]
    working_visualisations: NotUsed,
    #[serde(default)]
    se_allow_in_space: NotUsed,
    #[serde(default)]
    show_recipe_icon_on_map: NotUsed,
    #[serde(default)]
    alert_icon_shift: NotUsed,
    #[serde(default)]
    drawing_box: NotUsed,
    #[serde(default)]
    always_draw_idle_animation: NotUsed,
    #[serde(default)]
    idle_animation: NotUsed,
    #[serde(default)]
    bottleneck_ignore: NotUsed,
    #[serde(default)]
    circuit_wire_max_distance: NotUsed,
    #[serde(default)]
    selection_priority: NotUsed,
    #[serde(default)]
    localised_name: NotUsed,
}

#[derive(serde::Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct TransportBelt<'a> {
    #[serde(rename = "type")]
    pub transport_belt_type: &'a str,
    pub name: &'a str,
    #[serde(default)]
    pub subgroup: Option<&'a str>,
    pub flags: VecOrMap<&'a str>,
    pub fast_replaceable_group: &'a str,
    #[serde(default)]
    pub next_upgrade: Option<&'a str>,
    pub speed: f32,
    #[serde(default)]
    pub order: Option<&'a str>,

    #[serde(default)]
    icon: NotUsed,
    #[serde(default)]
    icons: NotUsed,
    icon_size: NotUsed,
    #[serde(default)]
    icon_mipmaps: NotUsed,
    minable: NotUsed,
    max_health: NotUsed,
    corpse: NotUsed,
    #[serde(default)]
    dying_explosion: NotUsed,
    resistances: NotUsed,
    collision_box: NotUsed,
    selection_box: NotUsed,
    #[serde(default)]
    damaged_trigger_effect: NotUsed,
    #[serde(default)]
    open_sound: NotUsed,
    #[serde(default)]
    close_sound: NotUsed,
    #[serde(default)]
    working_sound: NotUsed,
    animation_speed_coefficient: NotUsed,
    belt_animation_set: NotUsed,
    related_underground_belt: NotUsed,
    connector_frame_sprites: NotUsed,
    circuit_wire_connection_points: NotUsed,
    circuit_wire_max_distance: NotUsed,
    circuit_connector_sprites: NotUsed,
    collision_mask: NotUsed,
    #[serde(default)]
    localised_description: NotUsed,
    #[serde(default)]
    localised_name: NotUsed,
    #[serde(default)]
    animations: NotUsed,
}

#[derive(serde::Deserialize, Debug)]
pub struct ModuleSpecification {
    pub module_slots: u8,
}

#[derive(Debug, Default)]
pub struct FluidBoxes<'a> {
    pub boxes: Vec<FluidBox<'a>>,
    pub off_when_no_fluid_recipe: bool,
}

impl<'de: 'a, 'a> serde::de::Deserialize<'de> for FluidBoxes<'a> {
    fn deserialize<D>(deserializer: D) -> Result<FluidBoxes<'a>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a> {
            _marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'de: 'a, 'a> serde::de::Visitor<'de> for Visitor<'a> {
            type Value = FluidBoxes<'a>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence or a map of fluid boxes")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut boxes = Vec::new();
                while let Some(value) = seq.next_element()? {
                    boxes.push(value);
                }
                Ok(FluidBoxes {
                    boxes,
                    off_when_no_fluid_recipe: false,
                })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut result = FluidBoxes::default();
                while let Some(key) = map.next_key::<&'de str>()? {
                    if key == "off_when_no_fluid_recipe" {
                        result.off_when_no_fluid_recipe = map.next_value::<bool>()?;
                        continue;
                    }
                    if key.parse::<i32>().is_err() {
                        panic!("Unknown key {key:?}");
                    }
                    let value = map.next_value::<FluidBox<'a>>()?;
                    result.boxes.push(value);
                }
                Ok(result)
            }
        }

        deserializer.deserialize_any(Visitor {
            _marker: std::marker::PhantomData,
        })
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct FluidBox<'a> {
    pub production_type: &'a str,

    #[serde(default)]
    pub filter: Option<&'a str>,
    #[serde(default)]
    pub minimum_temperature: Option<f32>,

    #[serde(default)]
    pipe_covers: NotUsed,
    #[serde(default)]
    pipe_picture: NotUsed,
    base_area: NotUsed,
    base_level: NotUsed,
    pipe_connections: NotUsed,
    #[serde(default)]
    secondary_draw_orders: Option<NotUsed>,
    #[serde(default)]
    hide_connection_info: Option<NotUsed>,
}

#[derive(Debug)]
pub struct Unit {
    amount: f32,
    unit_type: UnitType,
}

impl<'de> serde::de::Deserialize<'de> for Unit {
    fn deserialize<D>(deserializer: D) -> Result<Unit, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Unit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string unit")
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let idx = v
                    .find(|c: char| !c.is_ascii_digit() && c != '.')
                    .unwrap_or(v.len());

                let amount = v[..idx].parse().map_err(serde::de::Error::custom)?;
                let unit_type = v[idx..].parse().map_err(|_| {
                    serde::de::Error::unknown_variant(&v[idx..].trim(), UnitType::all())
                })?;

                Ok(Unit { amount, unit_type })
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnitType {
    KJ,
    MJ,
    GJ,
    YJ,

    W,
    KW,
    MW,
    GW,
}

impl UnitType {
    pub fn all() -> &'static [&'static str] {
        &["kJ", "MJ", "GJ", "Yj", "W", "kW", "MW", "GW"]
    }
}

impl std::str::FromStr for UnitType {
    type Err = ();

    fn from_str(s: &str) -> Result<UnitType, ()> {
        match s.trim().to_uppercase().as_str() {
            "KJ" => Ok(UnitType::KJ),
            "MJ" => Ok(UnitType::MJ),
            "GJ" => Ok(UnitType::GJ),
            "YJ" => Ok(UnitType::YJ),
            "W" => Ok(UnitType::W),
            "KW" => Ok(UnitType::KW),
            "MW" => Ok(UnitType::MW),
            "GW" => Ok(UnitType::GW),
            _ => Err(()),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct Icon<'a> {
    #[serde(default)]
    size: Option<f32>,
    #[serde(default)]
    filename: Option<&'a str>,
    #[serde(default)]
    scale: Option<f32>,
    #[serde(default)]
    mipmap_count: Option<u8>,
    #[serde(default)]
    icon: Option<&'a str>,
    #[serde(default)]
    icon_size: Option<f32>,
    #[serde(default)]
    shift: Option<[f32; 2]>,
}

#[derive(Debug)]
pub struct VecOrMap<T>(pub Vec<T>);

impl<T> Deref for VecOrMap<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de, T> serde::de::Deserialize<'de> for VecOrMap<T>
where
    T: serde::de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<VecOrMap<T>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<T> {
            _marker: std::marker::PhantomData<T>,
        }

        impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
        where
            T: serde::de::Deserialize<'de>,
        {
            type Value = VecOrMap<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence or an empty map")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut vec = Vec::new();
                while let Some(value) = seq.next_element()? {
                    vec.push(value);
                }
                Ok(VecOrMap(vec))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let vec = Vec::new();
                if map.next_key::<&'de str>()?.is_some() {
                    return Err(serde::de::Error::custom("expected an empty map"));
                }
                Ok(VecOrMap(vec))
            }
        }

        deserializer.deserialize_any(Visitor {
            _marker: std::marker::PhantomData,
        })
    }
}
