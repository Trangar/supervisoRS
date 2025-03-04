pub struct ItemGroup { 
  icons: ItemGroupIcons,
  /// {"blueprint-sandboxes", "fusion-energy", "logistics", "production", "py-hightech", "enemies", "environment", "ee-tools", "intermediate-products", "py-petroleum-handling", "py-industry", "other", "signals", "py-alienlife", "tiles", "py-rawores", "py-alternativeenergy", "fluids", "coal-processing", "effects", "combat"}
  name: String,
  /// {"0"}
  order_in_recipe: String,
  /// {"item-group"}
  type: String,
  /// {"u", "t2", "v", "z", "t"}
  inventory_order: String,
  /// {"t2", "l", "v", "f", "u", "c", "i", "z", "zzzzz", "t", "g", "h", "y", "a", "e", "b"}
  order: String,
  /// min: 64, max: 128
  icon_size: i64,
  /// {"__base__/graphics/item-group/military.png", "__pyalienlifegraphics__/graphics/technology/item-group.png", "__core__/graphics/icons/category/unsorted.png", "__base__/graphics/item-group/logistics.png", "__pycoalprocessinggraphics__/graphics/technology/item-group.png", "__pyhightechgraphics__/graphics/technology/item-group.png", "__pyraworesgraphics__/graphics/technology/item-group.png", "__blueprint-sandboxes__/graphics/icon-x64.png", "__core__/graphics/icons/category/environment.png", "__pyindustrygraphics__/graphics/technology/item-group.png", "__core__/graphics/icons/category/enemies.png", "__pypetroleumhandlinggraphics__/graphics/technology/item-group.png", "__base__/graphics/item-group/fluids.png", "__base__/graphics/item-group/production.png", "__base__/graphics/item-group/intermediate-products.png", "__core__/graphics/icons/category/tiles-editor.png", "__base__/graphics/item-group/signals.png", "__pyalternativeenergygraphics__/graphics/icons/group/item-group.png", "__base__/graphics/item-group/effects.png", "__pyfusionenergygraphics__/graphics/technology/item-group.png"}
  icon: String,
