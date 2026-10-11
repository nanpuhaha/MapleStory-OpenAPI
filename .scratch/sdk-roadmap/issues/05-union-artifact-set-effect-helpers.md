# 05: Add domain aggregation helpers to Union Artifact and Set Effect models

**What to build:** Deepen the anemic `UserUnionArtifact` and `CharacterSetEffect` models with domain query methods that aggregate active artifact stat totals, detect expired crystals, and calculate active set thresholds across equipped gear, removing manual loop orchestration from consumer applications.

**Blocked by:** None (can start immediately)

**Status:** done

- [x] Add `active_effects(&self) -> impl Iterator<Item = &UnionArtifactEffect>` on `UserUnionArtifact` filtering out inactive or expired effects.
- [x] Add `expired_crystals(&self) -> Vec<&UnionArtifactCrystal>` identifying crystals whose expiration timestamp has passed.
- [x] Add `total_artifact_stats(&self) -> HashMap<String, f64>` summarizing active bonus numbers (e.g. Boss Damage %, Critical Damage %, Ignore Monster Armor %).
- [x] Add `is_set_active(&self, set_name: &str, required_count: i64) -> bool` and `active_set_options(&self) -> Vec<&SetEffectOption>` on `CharacterSetEffect`.
- [x] Unit tests verify artifact expiration logic, stat summation across multiple active crystals, and set threshold validation against mock JSON fixtures.
