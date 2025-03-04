pub struct Resource { 
  minable: ResourceMinable,
  /// min: -11.5, max: 11.5
  selection_box: f64,
  infinite: bool,
  /// min: 1024, max: 1024
  tree_removal_max_distance: i64,
  /// min: 3.6, max: 3.6
  effect_darkness_multiplier: f64,
  factoriopedia_simulation: ResourceFactoriopediaSimulation,
  /// min: 5, max: 5
  effect_animation_period: i64,
  stateless_visualisation: ResourceStatelessVisualisation,
  /// min: 0.7, max: 0.7
  tantimonium_removal_probability: f64,
  starting_area: bool,
  /// min: -11.45, max: 11.45
  collision_box: f64,
  /// min: 1024, max: 1024
  tantimonium_removal_max_distance: i64,
  collision_mask: ResourceCollisionMask,
  draw_stateless_visualisation_under_building: bool,
  /// min: 4, max: 15
  infinite_depletion_amount: i64,
  /// min: 0.3, max: 0.3
  max_effect_alpha: f64,
  /// min: 40, max: 40
  selection_priority: i64,
  /// min: 0.2, max: 0.2
  min_effect_alpha: f64,
  /// {"placeable-neutral"}
  flags: String,
  /// {"__pyraworesgraphics__/graphics/icons/ore-nickel.png", "__pypetroleumhandlinggraphics__/graphics/icons/ores/oil-sand.png", "__pyraworesgraphics__/graphics/icons/ores/coal-rock.png", "__pyraworesgraphics__/graphics/icons/ore-zinc.png", "__pyraworesgraphics__/graphics/icons/ore-chromium.png", "__pyalienlifegraphics__/graphics/icons/rennea-ore.png", "__pyraworesgraphics__/graphics/icons/ores/quartz-rock.png", "__pyraworesgraphics__/graphics/icons/ores/raw-coal.png", "__pyhightechgraphics__/graphics/icons/mip/re-01.png", "__base__/graphics/icons/iron-ore.png", "__pyalienlifegraphics__/graphics/icons/grod.png", "__pypetroleumhandlinggraphics__/graphics/icons/oil-mk02.png", "__pypetroleumhandlinggraphics__/graphics/icons/oil-mk03.png", "__pyraworesgraphics__/graphics/icons/ores/phosphate-rock-02.png", "__pyalienlifegraphics__/graphics/icons/tuuphra.png", "__pyalienlifegraphics__/graphics/icons/ralesia-ore.png", "__pyhightechgraphics__/graphics/icons/ores/ancient-remains.png", "__pyalienlifegraphics__/graphics/icons/cadaveric-arum.png", "__pyfusionenergygraphics__/graphics/icons/ores/molybdenum.png", "__pycoalprocessinggraphics__/graphics/icons/mip/nexelit-ore-3.png", "__base__/graphics/icons/copper-ore.png", "__pycoalprocessinggraphics__/graphics/icons/raw-borax.png", "__pyraworesgraphics__/graphics/icons/ores/chromium-rock.png", "__pyalternativeenergygraphics__/graphics/icons/antimonium-ore.png", "__pyraworesgraphics__/graphics/icons/ore-aluminium.png", "__pypetroleumhandlinggraphics__/graphics/icons/ores/tar-patch.png", "__pyalienlifegraphics__/graphics/icons/yotoi.png", "__pyraworesgraphics__/graphics/icons/ores/zinc-rock.png", "__pyraworesgraphics__/graphics/icons/ore-lead.png", "__pyraworesgraphics__/graphics/icons/ores/ore-tin.png", "__base__/graphics/icons/stone.png", "__pyhightechgraphics__/graphics/icons/ores/rare-earth-bolide.png", "__pyraworesgraphics__/graphics/icons/ores/uranium-rock.png", "__pyraworesgraphics__/graphics/icons/ores/copper-rock.png", "__pycoalprocessinggraphics__/graphics/icons/niobium-ore.png", "__pyfusionenergygraphics__/graphics/icons/ores/volcanic-pipe.png", "__pyraworesgraphics__/graphics/icons/ores/ore-quartz.png", "__pyfusionenergygraphics__/graphics/icons/ores/regolite-resource.png", "__pyraworesgraphics__/graphics/icons/ores/tin-rock.png", "__pyraworesgraphics__/graphics/icons/ores/salt-rock.png", "__pyraworesgraphics__/graphics/icons/ores/lead-rock.png", "__pyalienlifegraphics__/graphics/icons/kicalk.png", "__pypetroleumhandlinggraphics__/graphics/icons/ores/sulfur-patch.png", "__pyalienlifegraphics__/graphics/icons/yotoi-fruit.png", "__pyraworesgraphics__/graphics/icons/ores/iron-rock.png", "__pypetroleumhandlinggraphics__/graphics/icons/oil-mk04.png", "__pyraworesgraphics__/graphics/icons/ores/titanium-rock.png", "__pyraworesgraphics__/graphics/icons/ores/aluminium-rock.png", "__pyalienlifegraphics__/graphics/icons/mip/bio/30.png", "__base__/graphics/icons/coal.png", "__pyraworesgraphics__/graphics/icons/ores/nickel-rock.png", "__pyraworesgraphics__/graphics/icons/ores/nexelit-rock.png", "__base__/graphics/icons/crude-oil-resource.png", "__pyalternativeenergygraphics__/graphics/icons/mova.png", "__pyalternativeenergygraphics__/graphics/icons/geothermal-crack.png", "__pypetroleumhandlinggraphics__/graphics/icons/oil-mk01.png", "__pyraworesgraphics__/graphics/icons/ore-titanium.png", "__base__/graphics/icons/uranium-ore.png"}
  icon: String,
  /// {"natural-gas-mk01", "rare-earth-bolide", "titanium-rock", "yotoi-tree", "natural-gas-mk03", "ore-aluminium", "rennea-flowers", "oil-sand", "ralesia-flowers", "ore-chromium", "coal", "borax", "ore-tin", "sulfur-patch", "chromium-rock", "iron-ore", "bitumen-seep", "lead-rock", "arum", "oil-mk04", "regolites", "raw-coal", "tar-patch", "uranium-ore", "grod-flower", "ree", "nexelit-rock", "nickel-rock", "ore-quartz", "volcanic-pipe", "oil-mk03", "molybdenum-ore", "iron-rock", "oil-mk02", "coal-rock", "phosphate-rock-02", "stone", "copper-ore", "kicalk-tree", "copper-rock", "ore-bioreserve", "natural-gas-mk04", "ore-nickel", "oil-mk01", "zinc-rock", "yotoi-tree-fruit", "aluminium-rock", "ore-lead", "tuuphra-tuber", "ore-zinc", "antimonium", "uranium-rock", "tin-rock", "ore-titanium", "mova", "quartz-rock", "salt-rock", "natural-gas-mk02", "ore-nexelit", "niobium", "geothermal-crack", "phosphate-rock", "crude-oil"}
  name: String,
  /// min: 300000, max: 300000
  normal: i64,
  /// {"mineable-fluids"}
  subgroup: String,
  /// min: 1, max: 1
  effect_animation_period_deviation: i64,
  /// {"resource"}
  type: String,
  /// min: 0, max: 20000
  stage_counts: i64,
  /// {"a-b-e", "a-b-b", "a-b-a"}
  order: String,
  /// min: 60000, max: 60000
  minimum: i64,
  /// {"oil-mk03", "iron-rock", "phosphate", "titanium-rock", "coal-rock", "phosphate-rock-02", "yotoi-tree", "rennea-flowers", "kicalk-tree", "oil-sand", "ralesia-flowers", "copper-rock", "ore-bioreserve", "rare-earth", "borax", "sulfur-patch", "zinc-rock", "chromium-rock", "oil-mk01", "basic-with-fluid", "yotoi-tree-fruit", "bitumen-seep", "molybdenum", "aluminium-rock", "lead-rock", "uranium", "arum", "oil-mk04", "regolite", "tar-patch", "tuuphra-tuber", "antimonium", "grod-flower", "ree", "uranium-rock", "nexelit-rock", "tin-rock", "basic-fluid", "mova", "natural-gas", "nickel-rock", "quartz-rock", "salt-rock", "ore-nexelit", "volcanic-pipe", "niobium", "geothermal-crack", "oil-mk02"}
  category: String,
  /// min: 32, max: 64
  icon_size: i64,
  stages: ResourceStages,
  mining_visualisation_tint: ResourceMiningVisualisationTint,
  map_grid: bool,
  stages_effect: ResourceStagesEffect,
  /// min: 0.7, max: 0.8
  tree_removal_probability: f64,
  driving_sound: ResourceDrivingSound,
  walking_sound: ResourceWalkingSound,
