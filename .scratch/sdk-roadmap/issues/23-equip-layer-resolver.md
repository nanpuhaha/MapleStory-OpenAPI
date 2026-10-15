# Ticket 23: Equip Layering Engine (`EquipLayerResolver`)

## Background & Motivation
`CharacterCoordination` currently handles slot matching and visible equipment layering in an ad-hoc loop inside `from_equipment_and_cash`. It mixes slot mapping, transparent item heuristics, and anvil overrides.

## Requirements
1. Encapsulate layering logic in `EquipLayerResolver`:
   - Pure domain resolver in `src/models/character.rs` (or a dedicated submodule if appropriate).
   - Rules:
     - Cash item priority over Normal item in the same slot.
     - Transparent cash item (`투명`) allows underlying normal equipment appearance to shine through if not hidden, or properly sets visibility flags.
     - Anvil item (`item_shape_name`) overrides normal base item appearance.
     - Cash weapon / weapon type resolution.
2. `CharacterCoordination::from_equipment_and_cash` delegates to `EquipLayerResolver::resolve`.
3. Unit tests covering transparent items, anvil items, and full cash override layering.
