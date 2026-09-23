// src/client/character.rs
//! Client methods for Character tag API endpoints (23 endpoints).
//!
//! Includes the key convenience method [`MapleStoryClient::get_character_by_name`]
//! that performs a single-call lookup by character name.

use crate::{
    client::MapleStoryClient,
    error::{MapleStoryError, Result},
    models::character::{
        CharacterAbility, CharacterAndroidEquipment, CharacterBasic, CharacterBeautyEquipment,
        CharacterCashItemEquipment, CharacterDojang, CharacterHexaMatrix, CharacterHexaMatrixStat,
        CharacterHyperStat, CharacterItemEquipment, CharacterLinkSkill, CharacterOcid,
        CharacterOtherStat, CharacterPetEquipment, CharacterPopularity, CharacterPropensity,
        CharacterSetEffect, CharacterSkill, CharacterStat, CharacterSymbolEquipment,
        CharacterVMatrix, RingExchangeSkillEquipment, RingReserveSkillEquipment,
    },
    models::common::SkillGrade,
};

impl MapleStoryClient {
    // ─── OCID lookup ─────────────────────────────────────────────────────────

    /// Look up a character's OCID (Open Character ID) by name.
    ///
    /// Corresponds to `GET /maplestory/v1/id`.
    ///
    /// # Arguments
    /// * `character_name` — exact in-game character name
    ///
    /// # Errors
    /// Returns [`MapleStoryError::CharacterNotFound`] if no character with
    /// that name exists.
    pub async fn get_character_id(&self, character_name: &str) -> Result<CharacterOcid> {
        let params = [("character_name", character_name.to_string())];
        let result: CharacterOcid = self.get("/maplestory/v1/id", &params).await?;
        if result.ocid.as_deref().map(|s| s.is_empty()).unwrap_or(true) {
            return Err(MapleStoryError::CharacterNotFound {
                name: character_name.to_string(),
            });
        }
        Ok(result)
    }

    /// Get only the OCID string for a character name.
    ///
    /// # Errors
    /// Returns [`MapleStoryError::CharacterNotFound`] if the character does not
    /// exist or the OCID is empty.
    pub async fn get_ocid(&self, character_name: &str) -> Result<String> {
        let result = self.get_character_id(character_name).await?;
        result.ocid.ok_or_else(|| MapleStoryError::CharacterNotFound {
            name: character_name.to_string(),
        })
    }

    // ─── One-call convenience: 닉네임 한방 조회 ──────────────────────────────

    /// **Quick lookup by character name** — retrieves basic character info with a
    /// single method call.
    ///
    /// Internally this performs two API calls:
    /// 1. `GET /maplestory/v1/id` to resolve the character name → OCID
    /// 2. `GET /maplestory/v1/character/basic` using that OCID
    ///
    /// # Arguments
    /// * `character_name` — exact in-game character name
    ///
    /// # Example
    /// ```rust,no_run
    /// use maplestory_openapi::MapleStoryClient;
    ///
    /// # tokio_test::block_on(async {
    /// let client = MapleStoryClient::new("YOUR_API_KEY").unwrap();
    ///
    /// // One call — no manual OCID handling needed
    /// let info = client.get_character_by_name("Artale캐릭터").await.unwrap();
    /// println!("Lv.{:?} {} ({})",
    ///     info.character_level,
    ///     info.character_name.as_deref().unwrap_or("?"),
    ///     info.world_name.as_deref().unwrap_or("?"),
    /// );
    /// # });
    /// ```
    pub async fn get_character_by_name(&self, character_name: &str) -> Result<CharacterBasic> {
        let ocid = self.get_ocid(character_name).await?;
        self.get_character_basic(&ocid, None).await
    }

    // ─── Basic ───────────────────────────────────────────────────────────────

    /// Get basic character information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/basic`.
    ///
    /// # Arguments
    /// * `ocid` — character OCID from [`get_character_id`](Self::get_character_id)
    /// * `date` — optional `"YYYY-MM-DD"` reference date (KST)
    pub async fn get_character_basic(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterBasic> {
        let params = build_params(ocid, date)?;
        self.get("/maplestory/v1/character/basic", &params).await
    }

    // ─── Popularity ──────────────────────────────────────────────────────────

    /// Get character popularity.
    ///
    /// Corresponds to `GET /maplestory/v1/character/popularity`.
    pub async fn get_character_popularity(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterPopularity> {
        let params = build_params(ocid, date)?;
        self.get("/maplestory/v1/character/popularity", &params)
            .await
    }

    // ─── Stat ────────────────────────────────────────────────────────────────

    /// Get character comprehensive stats.
    ///
    /// Corresponds to `GET /maplestory/v1/character/stat`.
    pub async fn get_character_stat(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterStat> {
        let params = build_params(ocid, date)?;
        self.get("/maplestory/v1/character/stat", &params).await
    }

    // ─── Hyper Stat ──────────────────────────────────────────────────────────

    /// Get character hyper stat information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/hyper-stat`.
    pub async fn get_character_hyper_stat(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterHyperStat> {
        let params = build_params(ocid, date)?;
        self.get("/maplestory/v1/character/hyper-stat", &params)
            .await
    }

    // ─── Propensity ──────────────────────────────────────────────────────────

    /// Get character propensity (성향) information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/propensity`.
    pub async fn get_character_propensity(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterPropensity> {
        let params = build_params(ocid, date)?;
        self.get("/maplestory/v1/character/propensity", &params)
            .await
    }

    // ─── Ability ─────────────────────────────────────────────────────────────

    /// Get character ability (어빌리티) information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/ability`.
    pub async fn get_character_ability(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterAbility> {
        let params = build_params(ocid, date)?;
        self.get("/maplestory/v1/character/ability", &params).await
    }

    // ─── Item Equipment ───────────────────────────────────────────────────────

    /// Get equipped item information (장착 장비).
    ///
    /// Corresponds to `GET /maplestory/v1/character/item-equipment`.
    pub async fn get_character_item_equipment(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterItemEquipment> {
        let params = build_params(ocid, date)?;
        self.get("/maplestory/v1/character/item-equipment", &params)
            .await
    }

    // ─── Cash Item Equipment ─────────────────────────────────────────────────

    /// Get equipped cash item information (장착 캐시 장비).
    ///
    /// Corresponds to `GET /maplestory/v1/character/cashitem-equipment`.
    pub async fn get_character_cashitem_equipment(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterCashItemEquipment> {
        let params = build_params(ocid, date)?;
        self.get("/maplestory/v1/character/cashitem-equipment", &params)
            .await
    }

    // ─── Symbol Equipment ─────────────────────────────────────────────────────

    /// Get equipped symbol information (장착 심볼).
    ///
    /// Corresponds to `GET /maplestory/v1/character/symbol-equipment`.
    pub async fn get_character_symbol_equipment(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterSymbolEquipment> {
        let params = build_params(ocid, date)?;
        self.get("/maplestory/v1/character/symbol-equipment", &params)
            .await
    }

    // ─── Set Effect ───────────────────────────────────────────────────────────

    /// Get applied set effect information (세트 효과).
    ///
    /// Corresponds to `GET /maplestory/v1/character/set-effect`.
    pub async fn get_character_set_effect(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterSetEffect> {
        let params = build_params(ocid, date)?;
        self.get("/maplestory/v1/character/set-effect", &params)
            .await
    }

    // ─── Beauty Equipment ─────────────────────────────────────────────────────

    /// Get beauty (헤어/성형/피부) equipment information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/beauty-equipment`.
    pub async fn get_character_beauty_equipment(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterBeautyEquipment> {
        let params = build_params(ocid, date)?;
        self.get("/maplestory/v1/character/beauty-equipment", &params)
            .await
    }

    // ─── Android Equipment ────────────────────────────────────────────────────

    /// Get android equipment information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/android-equipment`.
    pub async fn get_character_android_equipment(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterAndroidEquipment> {
        let params = build_params(ocid, date)?;
        self.get("/maplestory/v1/character/android-equipment", &params)
            .await
    }

    // ─── Pet Equipment ────────────────────────────────────────────────────────

    /// Get pet equipment information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/pet-equipment`.
    pub async fn get_character_pet_equipment(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterPetEquipment> {
        let params = build_params(ocid, date)?;
        self.get("/maplestory/v1/character/pet-equipment", &params)
            .await
    }

    // ─── Skill ───────────────────────────────────────────────────────────────

    /// Get character skill information for a given job advancement tier.
    ///
    /// Corresponds to `GET /maplestory/v1/character/skill`.
    ///
    /// # Arguments
    /// * `ocid` — character OCID
    /// * `grade` — job advancement tier ([`SkillGrade`])
    /// * `date` — optional `"YYYY-MM-DD"` reference date (KST)
    ///
    /// # Example
    /// ```rust,no_run
    /// use maplestory_openapi::{MapleStoryClient, SkillGrade};
    ///
    /// # tokio_test::block_on(async {
    /// let client = MapleStoryClient::new("YOUR_API_KEY").unwrap();
    /// let ocid = client.get_ocid("캐릭터명").await.unwrap();
    /// let skills = client.get_character_skill(&ocid, SkillGrade::Fourth, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_character_skill(
        &self,
        ocid: &str,
        grade: SkillGrade,
        date: Option<&str>,
    ) -> Result<CharacterSkill> {
        let mut params = build_params(ocid, date)?;
        params.push(("character_skill_grade", grade.to_string()));
        self.get("/maplestory/v1/character/skill", &params).await
    }

    // ─── Link Skill ───────────────────────────────────────────────────────────

    /// Get link skill information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/link-skill`.
    pub async fn get_character_link_skill(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterLinkSkill> {
        let params = build_params(ocid, date)?;
        self.get("/maplestory/v1/character/link-skill", &params)
            .await
    }

    // ─── V Matrix ─────────────────────────────────────────────────────────────

    /// Get V-Matrix information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/vmatrix`.
    pub async fn get_character_vmatrix(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterVMatrix> {
        let params = build_params(ocid, date)?;
        self.get("/maplestory/v1/character/vmatrix", &params).await
    }

    // ─── HEXA Matrix ──────────────────────────────────────────────────────────

    /// Get HEXA Matrix core information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/hexamatrix`.
    pub async fn get_character_hexamatrix(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterHexaMatrix> {
        let params = build_params(ocid, date)?;
        self.get("/maplestory/v1/character/hexamatrix", &params)
            .await
    }

    // ─── HEXA Matrix Stat ─────────────────────────────────────────────────────

    /// Get HEXA stat core information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/hexamatrix-stat`.
    pub async fn get_character_hexamatrix_stat(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterHexaMatrixStat> {
        let params = build_params(ocid, date)?;
        self.get("/maplestory/v1/character/hexamatrix-stat", &params)
            .await
    }

    // ─── Dojang ───────────────────────────────────────────────────────────────

    /// Get Mu Lung Dojo (무릉도장) best record.
    ///
    /// Corresponds to `GET /maplestory/v1/character/dojang`.
    pub async fn get_character_dojang(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterDojang> {
        let params = build_params(ocid, date)?;
        self.get("/maplestory/v1/character/dojang", &params).await
    }

    // ─── Other Stat ───────────────────────────────────────────────────────────

    /// Get other stat influencing factors.
    ///
    /// Corresponds to `GET /maplestory/v1/character/other-stat`.
    ///
    /// > **Available from**: 2025-08-21
    pub async fn get_character_other_stat(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterOtherStat> {
        let params = build_params(ocid, date)?;
        self.get("/maplestory/v1/character/other-stat", &params)
            .await
    }

    // ─── Ring Exchange ────────────────────────────────────────────────────────

    /// Get ring exchange skill equipment information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/ring-exchange-skill-equipment`.
    ///
    /// > **Available**: 2025-08-21 ~ 2026-03-18
    pub async fn get_character_ring_exchange_skill_equipment(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<RingExchangeSkillEquipment> {
        let params = build_params(ocid, date)?;
        self.get(
            "/maplestory/v1/character/ring-exchange-skill-equipment",
            &params,
        )
        .await
    }

    // ─── Ring Reserve ─────────────────────────────────────────────────────────

    /// Get ring reserve skill equipment information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/ring-reserve-skill-equipment`.
    ///
    /// > **Available from**: 2026-03-19
    pub async fn get_character_ring_reserve_skill_equipment(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<RingReserveSkillEquipment> {
        let params = build_params(ocid, date)?;
        self.get(
            "/maplestory/v1/character/ring-reserve-skill-equipment",
            &params,
        )
        .await
    }
}

// ─── Helper ───────────────────────────────────────────────────────────────────

/// Build the common query parameters `[("ocid", ...), ("date", ...)]`.
fn build_params(ocid: &str, date: Option<&str>) -> Result<Vec<(&'static str, String)>> {
    let mut params: Vec<(&str, String)> = vec![("ocid", ocid.to_string())];
    if let Some(d) = date {
        params.push(("date", MapleStoryClient::validate_date(d)?));
    }
    Ok(params)
}
