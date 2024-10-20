#![allow(dead_code)]

mod not_used;

use std::ops::Deref;

use not_used::NotUsed;

use rustc_hash::FxHashMap;

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct Root<'a> {
    pub item: FxHashMap<&'a str, Item<'a>>,
    pub capsule: FxHashMap<&'a str, Capsule<'a>>,
    pub fluid: FxHashMap<&'a str, Fluid<'a>>,
    pub furnace: FxHashMap<&'a str, Furnace<'a>>,
    #[serde(rename = "transport-belt")]
    pub transport_belt: FxHashMap<&'a str, TransportBelt<'a>>,

    font: NotUsed,
    #[serde(rename = "noise-layer")]
    noise_layer: NotUsed,
    #[serde(rename = "gui-style")]
    gui_style: NotUsed,
    #[serde(rename = "utility-constants")]
    utility_constants: NotUsed,
    #[serde(rename = "utility-sounds")]
    utility_sounds: NotUsed,
    sprite: NotUsed,
    #[serde(rename = "utility-sprites")]
    utility_sprites: NotUsed,
    #[serde(rename = "god-controller")]
    god_controller: NotUsed,
    #[serde(rename = "editor-controller")]
    editor_controller: NotUsed,
    #[serde(rename = "spectator-controller")]
    spectator_controller: NotUsed,
    #[serde(rename = "noise-expression")]
    noise_expression: NotUsed,
    #[serde(rename = "mouse-cursor")]
    mouse_cursor: NotUsed,
    #[serde(rename = "virtual-signal")]
    virtual_signal: NotUsed,
    #[serde(rename = "flying-text")]
    flying_text: NotUsed,
    tile: NotUsed,
    #[serde(rename = "ambient-sound")]
    ambient_sound: NotUsed,
    #[serde(rename = "wind-sound")]
    wind_sound: NotUsed,
    container: NotUsed,
    explosion: NotUsed,
    #[serde(rename = "character-corpse")]
    character_corpse: NotUsed,
    #[serde(rename = "optimized-particle")]
    optimized_particle: NotUsed,
    #[serde(rename = "character")]
    character: NotUsed,
    fish: NotUsed, // Sorry
    radar: NotUsed,
    lamp: NotUsed,
    arrow: NotUsed,
    #[serde(rename = "entity-ghost")]
    entity_ghost: NotUsed,
    #[serde(rename = "tile-ghost")]
    tile_ghost: NotUsed,
    #[serde(rename = "deconstructible-tile-proxy")]
    deconstructible_tile_proxy: NotUsed,
    #[serde(rename = "item-request-proxy")]
    item_request_proxy: NotUsed,
    cliff: NotUsed,
    wall: NotUsed,
    #[serde(rename = "highlight-box")]
    highlight_box: NotUsed,
    splitter: NotUsed,
    #[serde(rename = "underground-belt")]
    underground_belt: NotUsed,
    loader: NotUsed,
    #[serde(rename = "loader-1x1")]
    loader_1x1: NotUsed,
    car: NotUsed,
    gate: NotUsed,
    #[serde(rename = "train-stop")]
    train_stop: NotUsed,
    #[serde(rename = "rail-signal")]
    rail_signal: NotUsed,
    #[serde(rename = "rail-chain-signal")]
    rail_chain_signal: NotUsed,
    #[serde(rename = "solar-panel")]
    solar_panel: NotUsed,
    accumulator: NotUsed,
    #[serde(rename = "electric-energy-interface")]
    electric_energy_interface: NotUsed,
    #[serde(rename = "player-port")]
    player_port: NotUsed,
    #[serde(rename = "land-mine")]
    land_mine: NotUsed,
    #[serde(rename = "logistic-container")]
    logistic_container: NotUsed,
    #[serde(rename = "rocket-silo")]
    rocket_silo: NotUsed,
    #[serde(rename = "rocket-silo-rocket")]
    rocket_silo_rocket: NotUsed,
    #[serde(rename = "rocket-silo-rocket-shadow")]
    rocket_silo_rocket_shadow: NotUsed,
    roboport: NotUsed,
    #[serde(rename = "storage-tank")]
    storage_tank: NotUsed,
    pump: NotUsed,
    market: NotUsed,
    #[serde(rename = "smoke-with-trigger")]
    smoke_with_trigger: NotUsed,
    sticker: NotUsed,
    #[serde(rename = "heat-pipe")]
    heat_pipe: NotUsed,
    #[serde(rename = "simple-entity-with-force")]
    simple_entity_with_force: NotUsed,
    #[serde(rename = "simple-entity-with-owner")]
    simple_entity_with_owner: NotUsed,
    #[serde(rename = "flame-thrower-explosion")]
    flame_thrower_explosion: NotUsed,
    #[serde(rename = "artillery-flare")]
    artillery_flare: NotUsed,
    #[serde(rename = "speech-bubble")]
    speech_bubble: NotUsed,
    #[serde(rename = "spider-vehicle")]
    spider_vehicle: NotUsed,
    #[serde(rename = "spider-leg")]
    spider_leg: NotUsed,
    #[serde(rename = "infinity-container")]
    infinity_container: NotUsed,
    #[serde(rename = "infinity-pipe")]
    infinity_pipe: NotUsed,
    #[serde(rename = "heat-interface")]
    heat_interface: NotUsed,
    #[serde(rename = "linked-container")]
    linked_container: NotUsed,
    #[serde(rename = "linked-belt")]
    linked_belt: NotUsed,
    #[serde(rename = "custom-input")]
    custom_input: NotUsed,
    fire: NotUsed,
    #[serde(rename = "mining-drill")]
    mining_drill: NotUsed,
    #[serde(rename = "particle-source")]
    particle_source: NotUsed,
    stream: NotUsed,
    turret: NotUsed,
    #[serde(rename = "ammo-turret")]
    ammo_turret: NotUsed,
    corpse: NotUsed,
    #[serde(rename = "electric-turret")]
    electric_turret: NotUsed,
    #[serde(rename = "artillery-turret")]
    artillery_turret: NotUsed,
    #[serde(rename = "unit-spawner")]
    unit_spawner: NotUsed,
    #[serde(rename = "straight-rail")]
    straight_rail: NotUsed,
    #[serde(rename = "curved-rail")]
    curved_rail: NotUsed,
    locomotive: NotUsed,
    #[serde(rename = "cargo-wagon")]
    cargo_wagon: NotUsed,
    #[serde(rename = "fluid-wagon")]
    fluid_wagon: NotUsed,
    #[serde(rename = "artillery-wagon")]
    artillery_wagon: NotUsed,
    #[serde(rename = "simple-entity")]
    simple_entity: NotUsed,
    #[serde(rename = "rail-remnants")]
    rail_remnants: NotUsed,
    tree: NotUsed,
    #[serde(rename = "trivial-smoke")]
    trivial_smoke: NotUsed,
    #[serde(rename = "combat-robot")]
    combat_robot: NotUsed,
    #[serde(rename = "construction-robot")]
    construction_robot: NotUsed,
    #[serde(rename = "logistic-robot")]
    logistic_robot: NotUsed,
    #[serde(rename = "repair-tool")]
    repair_tool: NotUsed,
    #[serde(rename = "copy-paste-tool")]
    copy_paste_tool: NotUsed,
    blueprint: NotUsed,
    tool: NotUsed,
    #[serde(rename = "item-with-entity-data")]
    item_with_entity_data: NotUsed,
    #[serde(rename = "rail-planner")]
    rail_planner: NotUsed,
    #[serde(rename = "upgrade-item")]
    upgrade_item: NotUsed,
    #[serde(rename = "deconstruction-item")]
    deconstruction_item: NotUsed,
    #[serde(rename = "blueprint-book")]
    blueprint_book: NotUsed,
    #[serde(rename = "spidertron-remote")]
    spidertron_remote: NotUsed,
    #[serde(rename = "selection-tool")]
    selection_tool: NotUsed,
    #[serde(rename = "item-with-tags")]
    item_with_tags: NotUsed,
    #[serde(rename = "item-with-label")]
    item_with_label: NotUsed,
    #[serde(rename = "item-with-inventory")]
    item_with_inventory: NotUsed,
    ammo: NotUsed,
    gun: NotUsed,
    armor: NotUsed,
    #[serde(rename = "mining-tool")]
    mining_tool: NotUsed,
    #[serde(rename = "item-group")]
    item_group: NotUsed, // Maybe I actually want to use this
    #[serde(rename = "item-subgroup")]
    item_subgroup: NotUsed, // Maybe I actually want to use this
    #[serde(rename = "autoplace-control")]
    autoplace_control: NotUsed,
    #[serde(rename = "map-settings")]
    map_settings: NotUsed,
    #[serde(rename = "map-gen-presets")]
    map_gen_presets: NotUsed,
    #[serde(rename = "tile-effect")]
    tile_effect: NotUsed,
    #[serde(rename = "optimized-decorative")]
    optimized_decorative: NotUsed,
    #[serde(rename = "damage-type")]
    damage_type: NotUsed,
    #[serde(rename = "ammo-category")]
    ammo_category: NotUsed,
    #[serde(rename = "fuel-category")]
    fuel_category: NotUsed,
    #[serde(rename = "recipe-category")]
    recipe_category: NotUsed, // Maybe I actually want to use this
    #[serde(rename = "resource-category")]
    resource_category: NotUsed, // Maybe I actually want to use this
    #[serde(rename = "module-category")]
    module_category: NotUsed, // Maybe I actually want to use this
    #[serde(rename = "equipment-grid")]
    equipment_grid: NotUsed,
    #[serde(rename = "equipment-category")]
    equipment_category: NotUsed,
    shortcut: NotUsed,
    #[serde(rename = "trigger-target-type")]
    trigger_target_type: NotUsed,
    projectile: NotUsed,
    #[serde(rename = "artillery-projectile")]
    artillery_projectile: NotUsed,
    beam: NotUsed,
    #[serde(rename = "tips-and-tricks-item")]
    tips_and_tricks_item: NotUsed,
    #[serde(rename = "tips-and-tricks-item-category")]
    tips_and_tricks_item_category: NotUsed,
    #[serde(rename = "build-entity-achievement")]
    build_entity_achievement: NotUsed,
    #[serde(rename = "research-achievement")]
    research_achievement: NotUsed,
    #[serde(rename = "finish-the-game-achievement")]
    finish_the_game_achievement: NotUsed,
    #[serde(rename = "group-attack-achievement")]
    group_attack_achievement: NotUsed,
    #[serde(rename = "construct-with-robots-achievement")]
    construct_with_robots_achievement: NotUsed,
    #[serde(rename = "deconstruct-with-robots-achievement")]
    deconstruct_with_robots_achievement: NotUsed,
    #[serde(rename = "deliver-by-robots-achievement")]
    deliver_by_robots_achievement: NotUsed,
    #[serde(rename = "train-path-achievement")]
    train_path_achievement: NotUsed,
    #[serde(rename = "player-damaged-achievement")]
    player_damaged_achievement: NotUsed,
    #[serde(rename = "produce-achievement")]
    produce_achievement: NotUsed,
    #[serde(rename = "produce-per-hour-achievement")]
    produce_per_hour_achievement: NotUsed,
    #[serde(rename = "dont-use-entity-in-energy-production-achievement")]
    dont_use_entity_in_energy_production_achievement: NotUsed,
    #[serde(rename = "kill-achievement")]
    kill_achievement: NotUsed,
    #[serde(rename = "combat-robot-count")]
    combat_robot_count: NotUsed,
    #[serde(rename = "dont-craft-manually-achievement")]
    dont_craft_manually_achievement: NotUsed,
    #[serde(rename = "dont-build-entity-achievement")]
    dont_build_entity_achievement: NotUsed,
    achievement: NotUsed,
    #[serde(rename = "fluid-turret")]
    fluid_turret: NotUsed,
    #[serde(rename = "arithmetic-combinator")]
    arithmetic_combinator: NotUsed,
    #[serde(rename = "decider-combinator")]
    decider_combinator: NotUsed,
    #[serde(rename = "constant-combinator")]
    constant_combinator: NotUsed,
    #[serde(rename = "power-switch")]
    power_switch: NotUsed,
    #[serde(rename = "programmable-speaker")]
    programmable_speaker: NotUsed,
    tutorial: NotUsed,
    #[serde(rename = "night-vision-equipment")]
    night_vision_equipment: NotUsed,
    #[serde(rename = "energy-shield-equipment")]
    energy_shield_equipment: NotUsed,
    #[serde(rename = "battery-equipment")]
    battery_equipment: NotUsed,
    #[serde(rename = "solar-panel-equipment")]
    solar_panel_equipment: NotUsed,
    #[serde(rename = "generator-equipment")]
    generator_equipment: NotUsed,
    #[serde(rename = "active-defense-equipment")]
    active_defense_equipment: NotUsed,
    #[serde(rename = "movement-bonus-equipment")]
    movement_bonus_equipment: NotUsed,
    #[serde(rename = "roboport-equipment")]
    roboport_equipment: NotUsed,
    #[serde(rename = "belt-immunity-equipment")]
    belt_immunity_equipment: NotUsed,
    smoke: NotUsed,
    particle: NotUsed,
    #[serde(rename = "leaf-particle")]
    leaf_particle: NotUsed,
    sound: NotUsed,
    animation: NotUsed,

    boiler: NotUsed, // TODO
    #[serde(rename = "electric-pole")]
    electric_pole: NotUsed, // TODO
    generator: NotUsed, // TODO
    #[serde(rename = "offshore-pump")]
    offshore_pump: NotUsed, // TODO
    inserter: NotUsed, // TODO
    #[serde(rename = "item-entity")]
    item_entity: NotUsed, // maybe TODO?
    pipe: NotUsed,   // maybe TODO?
    #[serde(rename = "pipe-to-ground")]
    pipe_to_ground: NotUsed, // maybe TODO?
    #[serde(rename = "assembling-machine")]
    assembling_machine: NotUsed, // TODO
    lab: NotUsed,    // TODO
    beacon: NotUsed, // TODO
    reactor: NotUsed, // TODO
    unit: NotUsed,   // TODO
    #[serde(rename = "burner-generator")]
    burner_generator: NotUsed, // TODO
    resource: NotUsed, // TODO
    pub module: FxHashMap<&'a str, Module<'a>>,
    pub recipe: FxHashMap<&'a str, Recipe<'a>>,
    technology: NotUsed, // TODO
}

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
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
#[serde(deny_unknown_fields)]
pub struct ModuleEffect {
    pub consumption: ModuleEffectBonus,
    pub pollution: ModuleEffectBonus,
    #[serde(default)]
    pub speed: Option<ModuleEffectBonus>,
    #[serde(default)]
    pub productivity: Option<ModuleEffectBonus>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ModuleEffectBonus {
    pub bonus: f32,
}

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct Recipe<'a> {
    #[serde(rename = "type")]
    pub recipe_type: &'a str,
    pub name: &'a str,
    #[serde(default)]
    pub category: Option<&'a str>,
    #[serde(default)]
    pub group: Option<&'a str>,
    #[serde(default)]
    pub subgroup: Option<&'a str>,
    #[serde(default)]
    pub order: Option<&'a str>,
    #[serde(default)]
    pub main_product: Option<&'a str>,
    #[serde(default)]
    pub ingredients: Option<VecOrMap<RecipeIngredient<'a>>>,
    #[serde(default)]
    pub result: Option<&'a str>,
    #[serde(default, rename = "mod")]
    pub mod_: Option<&'a str>,
    #[serde(default)]
    pub hide_from_player_crafting: Option<bool>,
    #[serde(default)]
    pub result_count: Option<f32>,
    #[serde(default)]
    pub results: Option<VecOrMap<RecipeResult<'a>>>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub allow_productivity: Option<bool>,
    #[serde(default)]
    pub hidden: Option<bool>,
    pub always_show_made_in: bool,
    #[serde(default)]
    pub normal: Option<EnabledRecipeIngredients<'a>>,
    #[serde(default)]
    pub allow_decomposition: Option<bool>,
    #[serde(default)]
    pub allow_as_intermediate: Option<bool>,
    #[serde(default)]
    pub show_amount_in_title: Option<bool>,
    #[serde(default)]
    pub flags: Option<VecOrMap<&'a str>>,

    #[serde(default)]
    localised_name: NotUsed,
    #[serde(default)]
    localized_name: NotUsed,
    #[serde(default)]
    localized_description: NotUsed,
    #[serde(default)]
    localised_description: NotUsed,
    #[serde(default)]
    expensive: Option<EnabledRecipeIngredients<'a>>,
    #[serde(default)]
    emissions_multiplier: Option<f32>,
    #[serde(default)]
    requester_paste_multiplier: Option<usize>,
    #[serde(default)]
    always_show_products: bool,
    #[serde(default)]
    energy_required: Option<f32>,
    #[serde(default)]
    icon: NotUsed,
    #[serde(default)]
    icons: NotUsed,
    #[serde(default)]
    icon_size: NotUsed,
    #[serde(default)]
    icon_mipmaps: NotUsed,
    #[serde(default)]
    crafting_machine_tint: NotUsed,
    #[serde(default)]
    hide_from_stats: Option<bool>,
    #[serde(default)]
    allow_intermediates: Option<bool>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
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

#[derive(Debug)]
pub struct RecipeIngredient<'a> {
    pub ingredient_type: &'a str,
    pub name: &'a str,
    pub amount: f32,
    pub catalyst_amount: Option<f32>,
    pub minimum_temperature: Option<f32>,
    pub maximum_temperature: Option<f32>,
}

impl<'a, 'de: 'a> serde::de::Deserialize<'de> for RecipeIngredient<'a> {
    fn deserialize<D>(deserializer: D) -> Result<RecipeIngredient<'a>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a> {
            _marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'a, 'de: 'a> serde::de::Visitor<'de> for Visitor<'a> {
            type Value = RecipeIngredient<'a>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map of recipe ingredients")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let item_name = seq.next_element::<&'de str>()?;
                let item_amount = seq.next_element::<f32>()?;
                Ok(RecipeIngredient {
                    ingredient_type: "item",
                    name: item_name.ok_or_else(|| serde::de::Error::missing_field("item name"))?,
                    amount: item_amount
                        .ok_or_else(|| serde::de::Error::missing_field("item amount"))?,
                    catalyst_amount: None,
                    minimum_temperature: None,
                    maximum_temperature: None,
                })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut ingredient_type = None;
                let mut name = None;
                let mut amount = None;
                let mut catalyst_amount = None;
                let mut minimum_temperature = None;
                let mut maximum_temperature = None;
                while let Some(key) = map.next_key::<&'de str>()? {
                    match key {
                        "type" => {
                            ingredient_type = Some(map.next_value()?);
                        }
                        "name" => {
                            name = Some(map.next_value()?);
                        }
                        "amount" => {
                            amount = Some(map.next_value()?);
                        }
                        "catalyst_amount" => {
                            catalyst_amount = Some(map.next_value()?);
                        }
                        "minimum_temperature" => {
                            minimum_temperature = Some(map.next_value()?);
                        }
                        "maximum_temperature" => {
                            maximum_temperature = Some(map.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                key,
                                &[
                                    "type",
                                    "name",
                                    "amount",
                                    "catalyst_amount",
                                    "minimum_temperature",
                                    "maximum_temperature",
                                ],
                            ));
                        }
                    }
                }
                Ok(RecipeIngredient {
                    ingredient_type: ingredient_type.unwrap_or("item"),
                    name: name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    amount: amount.ok_or_else(|| serde::de::Error::missing_field("amount"))?,
                    catalyst_amount,
                    minimum_temperature,
                    maximum_temperature,
                })
            }
        }

        deserializer.deserialize_any(Visitor {
            _marker: std::marker::PhantomData,
        })
    }
}

#[derive(Debug)]
pub struct RecipeResult<'a> {
    pub result_type: &'a str,
    pub item: &'a str,
    pub amount: Option<f32>,
    pub probability: Option<f32>,
    pub amount_min: Option<f32>,
    pub amount_max: Option<f32>,
    pub fluidbox_index: Option<u8>,
    pub catalyst_amount: Option<f32>,
    pub temperature: Option<f32>,
}

impl<'a, 'de: 'a> serde::de::Deserialize<'de> for RecipeResult<'a> {
    fn deserialize<D>(deserializer: D) -> Result<RecipeResult<'a>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a> {
            _marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'a, 'de: 'a> serde::de::Visitor<'de> for Visitor<'a> {
            type Value = RecipeResult<'a>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map of recipe results")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let name = seq
                    .next_element::<&'de str>()?
                    .ok_or_else(|| serde::de::Error::missing_field("result seq name"))?;
                let amount = seq
                    .next_element::<f32>()?
                    .ok_or_else(|| serde::de::Error::missing_field("result seq amount"))?;
                Ok(RecipeResult {
                    result_type: "item",
                    item: name,
                    amount: Some(amount),
                    probability: None,
                    fluidbox_index: None,
                    catalyst_amount: None,
                    amount_min: None,
                    amount_max: None,
                    temperature: None,
                })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut result_type = None;
                let mut name = None;
                let mut amount = None;
                let mut fluidbox_index = None;
                let mut catalyst_amount = None;
                let mut probability = None;
                let mut amount_min = None;
                let mut amount_max = None;
                let mut temperature = None;
                while let Some(key) = map.next_key::<&'de str>()? {
                    match key {
                        "type" => {
                            result_type = Some(map.next_value()?);
                        }
                        "name" => {
                            name = Some(map.next_value()?);
                        }
                        "amount" => {
                            amount = Some(map.next_value()?);
                        }
                        "fluidbox_index" => {
                            fluidbox_index = Some(map.next_value()?);
                        }
                        "catalyst_amount" => {
                            catalyst_amount = Some(map.next_value()?);
                        }
                        "probability" => {
                            probability = Some(map.next_value()?);
                        }
                        "amount_min" => {
                            amount_min = Some(map.next_value()?);
                        }
                        "amount_max" => {
                            amount_max = Some(map.next_value()?);
                        }
                        "temperature" => {
                            temperature = Some(map.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                key,
                                &[
                                    "type",
                                    "name",
                                    "amount",
                                    "fluidbox_index",
                                    "catalyst_amount",
                                    "probability",
                                    "amount_min",
                                    "amount_max",
                                    "temperature",
                                ],
                            ));
                        }
                    }
                }

                Ok(RecipeResult {
                    result_type: result_type.unwrap_or("item"),
                    item: name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    amount,
                    fluidbox_index,
                    catalyst_amount,
                    probability,
                    amount_min,
                    amount_max,
                    temperature,
                })
            }
        }

        deserializer.deserialize_any(Visitor {
            _marker: std::marker::PhantomData,
        })
    }
}
#[derive(serde::Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct Capsule<'a> {
    #[serde(rename = "type")]
    pub capsule_type: &'a str,
    pub name: &'a str,
}

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
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
    #[serde(default)]
    pictures: NotUsed,
    #[serde(default)]
    wire_count: Option<u8>,
    #[serde(default)]
    place_result: Option<&'a str>,

    #[serde(default)]
    localised_description: NotUsed,
    #[serde(default)]
    localised_name: NotUsed,

    #[serde(default)]
    icons: NotUsed,
    #[serde(default)]
    icon_size: NotUsed,
    #[serde(default)]
    placed_as_equipment_result: NotUsed,
    #[serde(default)]
    default_request_amount: usize,
}

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
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
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
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
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
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
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
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

    W,
    KW,
    MW,
    GW,
}

impl UnitType {
    pub fn all() -> &'static [&'static str] {
        &["kJ", "MJ", "GJ", "W", "kW", "MW", "GW"]
    }
}

impl std::str::FromStr for UnitType {
    type Err = ();

    fn from_str(s: &str) -> Result<UnitType, ()> {
        match s.trim().to_uppercase().as_str() {
            "KJ" => Ok(UnitType::KJ),
            "MJ" => Ok(UnitType::MJ),
            "GJ" => Ok(UnitType::GJ),
            "W" => Ok(UnitType::W),
            "KW" => Ok(UnitType::KW),
            "MW" => Ok(UnitType::MW),
            "GW" => Ok(UnitType::GW),
            _ => Err(()),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
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
