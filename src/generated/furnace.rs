pub struct Furnace { 
  minable: FurnaceMinable,
  energy_source: FurnaceEnergySource,
  /// min: -5.5, max: 5.5
  selection_box: f64,
  working_visualisations: FurnaceWorkingVisualisations,
  show_recipe_icon_on_map: bool,
  mined_sound: FurnaceMinedSound,
  /// min: 0, max: 1
  source_inventory_size: i64,
  damaged_trigger_effect: FurnaceDamagedTriggerEffect,
  /// {"py-venting", "py-unbarreling", "co2", "smelting", "biofluid", "py-incineration", "compost", "py-runoff", "py-barreling"}
  crafting_categories: String,
  icons_positioning: FurnaceIconsPositioning,
  repair_sound: FurnaceRepairSound,
  /// {"stone-furnace-remnants", "steel-furnace-remnants", "electric-furnace-remnants", "medium-remnants", "big-remnants", "small-remnants"}
  corpse: String,
  /// min: 100, max: 1200
  max_health: i64,
  /// min: 0, max: 1
  result_inventory_size: i64,
  /// min: -5.2, max: 5.2
  collision_box: f64,
  collision_mask: FurnaceCollisionMask,
  working_sound: FurnaceWorkingSound,
  resistances: FurnaceResistances,
  /// {"player-creation", "placeable-neutral", "placeable-player", "not-in-made-in", "placeable-enemy"}
  flags: String,
  /// {"__pyindustrygraphics__/graphics/icons/py-sinkhole.png", "__pyalienlifegraphics__/graphics/icons/compost-plant-mk04.png", "__pyalienlifegraphics__/graphics/icons/compost-plant-mk03.png", "__pyindustrygraphics__/graphics/icons/burner.png", "__pycoalprocessinggraphics__/graphics/icons/co2-absorber.png", "__base__/graphics/icons/electric-furnace.png", "__pyindustrygraphics__/graphics/icons/py-gas-vent.png", "__pyalienlifegraphics__/graphics/icons/chest-request.png", "__pyalienlifegraphics__/graphics/icons/chest-provider.png", "__base__/graphics/icons/stone-furnace.png", "__base__/graphics/icons/steel-furnace.png", "__pyalienlifegraphics__/graphics/icons/compost-plant-mk02.png", "__pyalienlifegraphics__/graphics/icons/compost-plant-mk01.png", "__pyindustrygraphics__/graphics/icons/barrel-machine.png"}
  icon: String,
  /// {"diagonal-pos", "diagonal-neg"}
  forced_symmetry: String,
  match_animation_speed_to_activity: bool,
  /// {"py-sinkhole", "compost-plant-mk01", "compost-plant-mk01-turd", "compost-plant-mk03", "py-burner", "compost-plant-mk04-turd", "provider-tank", "compost-plant-mk03-turd", "co2-absorber", "py-gas-vent", "steel-furnace", "electric-furnace", "compost-plant-mk02", "barrel-machine-mk01", "compost-plant-mk02-turd", "requester-tank", "stone-furnace", "compost-plant-mk04"}
  name: String,
  fluid_boxes: FurnaceFluidBoxes,
  open_sound: FurnaceOpenSound,
  /// {"steel-furnace", "compost-plant-mk04-turd", "compost-plant-mk03-turd", "compost-plant-mk02-turd", "compost-plant-mk02", "compost-plant-mk03", "compost-plant-mk04"}
  next_upgrade: String,
  effect_receiver: FurnaceEffectReceiver,
  /// {"productivity", "compost-plant", "quality", "efficiency", "speed"}
  allowed_module_categories: String,
  /// {"furnace"}
  type: String,
  /// min: 0.5, max: 8
  crafting_speed: f64,
  /// {"py-gas-vent", "py-sinkhole", "compost-plant", "vessel", "furnace", "py-burner", "barrel-machine", "co2-absorber"}
  fast_replaceable_group: String,
  /// {"d"}
  order: String,
  /// {"stone", "metal-large", "metal"}
  impact_category: String,
  /// {"productivity", "consumption", "pollution", "speed"}
  allowed_effects: String,
  /// min: 64, max: 64
  icon_size: i64,
  /// {"py-alienlife-buildings-mk02", "py-alienlife-buildings-mk03", "py-alienlife-buildings-mk04", "py-alienlife-buildings-mk01"}
  subgroup: String,
  bottleneck_ignore: bool,
  show_recipe_icon: bool,
  /// {"stone-furnace-explosion", "electric-furnace-explosion", "big-explosion", "medium-explosion", "steel-furnace-explosion"}
  dying_explosion: String,
  /// {"entity-description.compost-plant-mk04", "entity-description.compost-plant-mk03", "entity-description.compost-plant-mk01", "entity-description.compost-plant-mk02"}
  localised_description: String,
  /// min: 9, max: 14
  circuit_wire_max_distance: i64,
  /// {"6MW", "10MW", "1MW", "1W", "2MW", "4MW", "200kW", "180kW", "1kW", "500kW", "3MW", "25kW"}
  energy_usage: String,
  /// min: 0, max: 4
  module_slots: i64,
  icon_draw_specification: FurnaceIconDrawSpecification,
  fluid_boxes_off_when_no_fluid_recipe: bool,
  /// {"entity-name.compost-plant-mk03", "entity-name.compost-plant-mk02", "entity-name.compost-plant-mk01", "entity-name.compost-plant-mk04"}
  localised_name: String,
  placeable_by: FurnacePlaceableBy,
  /// {"requester-tank"}
  additional_pastable_entities: String,
  close_sound: FurnaceCloseSound,
  graphics_set: FurnaceGraphicsSet,
  circuit_connector: FurnaceCircuitConnector,
