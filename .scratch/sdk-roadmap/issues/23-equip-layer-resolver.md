# 23: Equip Layering Engine (`EquipLayerResolver`)

**What to build:** Encapsulate equipment layering, cash priority rules, overall (`한벌옷`) slot precedence, transparent item detection, and anvil overrides into a dedicated domain engine `EquipLayerResolver`.

**Blocked by:** None

**Status:** done

- [x] Implemented `EquipLayerResolver::resolve(beauty, cash_equip, normal_equip) -> CharacterCoordination` in `src/models/character.rs`.
- [x] Internalized layering rules:
  - Cash items take priority over normal equipment in the same slot.
  - Overall (`한벌옷`) covers both top and bottom slots, skipping bottom in normal equipment.
  - Transparent item detection (`is_transparent`) for `투명*` cash items.
  - Anvil items (`item_shape_name` / `item_shape_icon`) override base item appearance.
- [x] Delegated `CharacterCoordination::from_parts` and `from_equipment_and_cash` to `EquipLayerResolver::resolve`.
- [x] Added automated unit tests in `test_equip_layer_resolver_direct_and_custom_rules` verifying transparent items, anvil items, and full cash override layering.
