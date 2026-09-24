// src/client/character.rs
//! Client methods for Character tag API endpoints (23 endpoints).
//!
//! Includes the key convenience method [`MapleStoryClient::get_character_by_name`]
//! that performs a single-call lookup by character name.

use crate::{
    client::MapleStoryClient,
    error::{MapleStoryError, Result},
    models::character::{
        CharacterAbility, CharacterAndroidEquipment, CharacterAppearance, CharacterBasic,
        CharacterBeautyEquipment, CharacterCashItemEquipment, CharacterCoordination,
        CharacterDashboard, CharacterDojang, CharacterHexaMatrix, CharacterHexaMatrixStat,
        CharacterHyperStat, CharacterItemEquipment, CharacterLinkSkill, CharacterOcid,
        CharacterOtherStat, CharacterPetEquipment, CharacterPopularity, CharacterProfile,
        CharacterPropensity, CharacterSetEffect, CharacterSkill, CharacterStat,
        CharacterSymbolEquipment, CharacterVMatrix, RingExchangeSkillEquipment,
        RingReserveSkillEquipment,
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
    /// Automatically utilizes the in-memory cache if the `cache` feature is enabled.
    ///
    /// # Errors
    /// Returns [`MapleStoryError::CharacterNotFound`] if the character does not
    /// exist or the OCID is empty.
    pub async fn get_ocid(&self, character_name: &str) -> Result<String> {
        #[cfg(feature = "cache")]
        if let Some(cache) = &self.ocid_cache {
            if let Some(cached_ocid) = cache.get(character_name).await {
                tracing::debug!("OCID cache hit for '{}'", character_name);
                return Ok(cached_ocid);
            }
        }

        let result = self.get_character_id(character_name).await?;
        let ocid = result
            .ocid
            .ok_or_else(|| MapleStoryError::CharacterNotFound {
                name: character_name.to_string(),
            })?;

        #[cfg(feature = "cache")]
        if let Some(cache) = &self.ocid_cache {
            cache.insert(character_name.to_string(), ocid.clone()).await;
        }

        Ok(ocid)
    }

    // ─── Convenience: 닉네임 간편 조회 메서드군 ───────────────────────────────

    /// **Quick lookup by character name** — retrieves basic character info with a
    /// single method call.
    pub async fn get_character_by_name(&self, character_name: &str) -> Result<CharacterBasic> {
        let ocid = self.get_ocid(character_name).await?;
        self.get_character_basic(&ocid, None).await
    }

    /// Retrieve character comprehensive stats directly by character name.
    pub async fn get_character_stat_by_name(&self, character_name: &str) -> Result<CharacterStat> {
        let ocid = self.get_ocid(character_name).await?;
        self.get_character_stat(&ocid, None).await
    }

    /// Retrieve character equipped items directly by character name.
    pub async fn get_character_item_equipment_by_name(
        &self,
        character_name: &str,
    ) -> Result<CharacterItemEquipment> {
        let ocid = self.get_ocid(character_name).await?;
        self.get_character_item_equipment(&ocid, None).await
    }

    /// Retrieve character cash items directly by character name.
    pub async fn get_character_cashitem_equipment_by_name(
        &self,
        character_name: &str,
    ) -> Result<CharacterCashItemEquipment> {
        let ocid = self.get_ocid(character_name).await?;
        self.get_character_cashitem_equipment(&ocid, None).await
    }

    /// Retrieve character beauty (hair, face, skin) directly by character name.
    pub async fn get_character_beauty_equipment_by_name(
        &self,
        character_name: &str,
    ) -> Result<CharacterBeautyEquipment> {
        let ocid = self.get_ocid(character_name).await?;
        self.get_character_beauty_equipment(&ocid, None).await
    }

    /// Retrieve character popularity directly by character name.
    pub async fn get_character_popularity_by_name(
        &self,
        character_name: &str,
    ) -> Result<CharacterPopularity> {
        let ocid = self.get_ocid(character_name).await?;
        self.get_character_popularity(&ocid, None).await
    }

    /// Retrieve character Mu Lung Dojo record directly by character name.
    pub async fn get_character_dojang_by_name(
        &self,
        character_name: &str,
    ) -> Result<CharacterDojang> {
        let ocid = self.get_ocid(character_name).await?;
        self.get_character_dojang(&ocid, None).await
    }

    /// Retrieve character skills directly by character name.
    pub async fn get_character_skill_by_name(
        &self,
        character_name: &str,
        grade: SkillGrade,
    ) -> Result<CharacterSkill> {
        let ocid = self.get_ocid(character_name).await?;
        self.get_character_skill(&ocid, grade, None).await
    }

    // ─── Coordination & Outfit (코디 / 외형 합성 조회) ──────────────────────────

    /// Retrieve final visible coordination (코디 정보) for a character.
    /// Retrieve synthesized character coordination (최종 코디/외형 정보) by OCID.
    ///
    /// Fetches basic character info, beauty equipment, cash equipment, and normal equipment in parallel,
    /// and synthesizes them according to in-game display rules:
    /// - Cash equipment takes precedence over normal equipment (with base fallback)
    /// - Normal equipment displays anvil (신비의 모루) shape if applied
    /// - Sub-weapons and transparent items are tagged with visibility helpers
    /// - Coloring prism items retain applied recolored icons and color metadata
    /// - Avatar code (1024-char) and customizable pose URLs are automatically provided
    pub async fn get_character_coordination(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterCoordination> {
        let ((basic_res, beauty_res), (cash_res, equip_res)) = futures_util::future::join(
            futures_util::future::join(
                self.get_character_basic(ocid, date),
                self.get_character_beauty_equipment(ocid, date),
            ),
            futures_util::future::join(
                self.get_character_cashitem_equipment(ocid, date),
                self.get_character_item_equipment(ocid, date),
            ),
        )
        .await;

        let basic = basic_res?;
        let beauty = beauty_res?;
        let cash = cash_res?;
        let equip = equip_res?;

        Ok(CharacterCoordination::from_full_parts(&basic, &beauty, &cash, &equip))
    }

    /// Retrieve final visible coordination (코디 정보) directly by character name.
    pub async fn get_character_coordination_by_name(
        &self,
        character_name: &str,
    ) -> Result<CharacterCoordination> {
        let ocid = self.get_ocid(character_name).await?;
        self.get_character_coordination(&ocid, None).await
    }

    /// Retrieve complete visual look and coordination directly by character name.
    ///
    /// Convenient one-stop method returning character metadata, synthesized visible equipment,
    /// coloring prism items, beauty details, and dynamic avatar pose builders.
    pub async fn get_character_look_by_name(
        &self,
        character_name: &str,
    ) -> Result<CharacterCoordination> {
        self.get_character_coordination_by_name(character_name).await
    }

    /// Alias for [`get_character_coordination_by_name`](Self::get_character_coordination_by_name).
    pub async fn get_character_appearance_by_name(
        &self,
        character_name: &str,
    ) -> Result<CharacterAppearance> {
        self.get_character_coordination_by_name(character_name).await
    }

    // ─── Comprehensive Profile (원클릭 종합 프로필 조회) ─────────────────────

    /// Retrieve a comprehensive aggregated profile of a character.
    ///
    /// Concurrently queries basic info, stats, normal equipment, cash items,
    /// beauty settings, popularity, and Mu Lung Dojo records, and synthesizes
    /// coordination look data into a single [`CharacterProfile`].
    pub async fn get_character_profile(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterProfile> {
        let ((basic, stat), (equip, cash), (beauty, pop, dojang)) = futures_util::future::join3(
            futures_util::future::join(
                self.get_character_basic(ocid, date),
                self.get_character_stat(ocid, date),
            ),
            futures_util::future::join(
                self.get_character_item_equipment(ocid, date),
                self.get_character_cashitem_equipment(ocid, date),
            ),
            futures_util::future::join3(
                self.get_character_beauty_equipment(ocid, date),
                self.get_character_popularity(ocid, date),
                self.get_character_dojang(ocid, date),
            ),
        )
        .await;

        let basic = basic?;
        let stat = stat?;
        let equip = equip?;
        let cash = cash?;
        let beauty = beauty?;
        let popularity = pop.ok();
        let dojang = dojang.ok();

        let coordination = CharacterCoordination::from_full_parts(&basic, &beauty, &cash, &equip);

        Ok(CharacterProfile {
            basic,
            stat,
            item_equipment: equip,
            cash_item_equipment: cash,
            beauty_equipment: beauty,
            coordination,
            popularity,
            dojang,
        })
    }

    /// Retrieve a comprehensive aggregated profile directly by character name.
    pub async fn get_character_profile_by_name(
        &self,
        character_name: &str,
    ) -> Result<CharacterProfile> {
        let ocid = self.get_ocid(character_name).await?;
        self.get_character_profile(&ocid, None).await
    }

    /// Retrieve a comprehensive all-in-one character dashboard (올인원 캐릭터 종합 대시보드).
    ///
    /// Fetches all specs, equipment, symbols, abilities, 6th HEXA matrix,
    /// 5th V matrix, union, artifacts, and guild information concurrently.
    ///
    /// Non-critical or tier-dependent endpoints (like 6th job for low level chars,
    /// or guild info for guildless chars) gracefully fall back to `None`.
    pub async fn get_character_dashboard(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterDashboard> {
        let (
            basic,
            stat,
            equip,
            cash,
            beauty,
            symbols,
            ability,
            hyper_stat,
            hexa_matrix,
            hexa_matrix_stat,
            v_matrix,
            set_effect,
            pet,
            android,
            link,
            propensity,
            popularity,
            dojang,
            union,
            union_artifact,
        ) = futures_util::join!(
            self.get_character_basic(ocid, date),
            self.get_character_stat(ocid, date),
            self.get_character_item_equipment(ocid, date),
            self.get_character_cashitem_equipment(ocid, date),
            self.get_character_beauty_equipment(ocid, date),
            self.get_character_symbol_equipment(ocid, date),
            self.get_character_ability(ocid, date),
            self.get_character_hyper_stat(ocid, date),
            self.get_character_hexamatrix(ocid, date),
            self.get_character_hexamatrix_stat(ocid, date),
            self.get_character_vmatrix(ocid, date),
            self.get_character_set_effect(ocid, date),
            self.get_character_pet_equipment(ocid, date),
            self.get_character_android_equipment(ocid, date),
            self.get_character_link_skill(ocid, date),
            self.get_character_propensity(ocid, date),
            self.get_character_popularity(ocid, date),
            self.get_character_dojang(ocid, date),
            self.get_user_union(ocid, date),
            self.get_user_union_artifact(ocid, date),
        );

        let basic = basic?;
        let stat = stat?;
        let equip = equip?;
        let cash = cash?;
        let beauty = beauty?;

        let coordination = CharacterCoordination::from_full_parts(&basic, &beauty, &cash, &equip);

        // Fetch guild information if the character is in a guild
        let guild = match (
            basic.character_guild_name.as_deref(),
            basic.world_name.as_deref(),
        ) {
            (Some(guild_name), Some(world_name)) if !guild_name.is_empty() && !world_name.is_empty() => {
                self.get_guild_by_name(guild_name, world_name, date).await.ok()
            }
            _ => None,
        };

        Ok(CharacterDashboard {
            basic,
            stat,
            item_equipment: equip,
            cash_item_equipment: cash,
            beauty_equipment: beauty,
            coordination,
            symbols: symbols.ok(),
            ability: ability.ok(),
            hyper_stat: hyper_stat.ok(),
            hexa_matrix: hexa_matrix.ok(),
            hexa_matrix_stat: hexa_matrix_stat.ok(),
            v_matrix: v_matrix.ok(),
            set_effect: set_effect.ok(),
            pet_equipment: pet.ok(),
            android_equipment: android.ok(),
            link_skill: link.ok(),
            propensity: propensity.ok(),
            popularity: popularity.ok(),
            dojang: dojang.ok(),
            union: union.ok(),
            union_artifact: union_artifact.ok(),
            guild,
        })
    }

    /// Retrieve a comprehensive all-in-one character dashboard directly by character name.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use maplestory_openapi::MapleStoryClient;
    /// # tokio_test::block_on(async {
    /// let client = MapleStoryClient::new("YOUR_API_KEY").unwrap();
    /// let dash = client.get_character_dashboard_by_name("온앤온").await.unwrap();
    /// println!("전투력: {}", dash.combat_power());
    /// println!("스타포스: {}", dash.total_starforce());
    /// println!("아케인포스: {}", dash.arcane_force());
    /// println!("유니온: {:?}", dash.union_level());
    /// println!("길드: {:?} (노블 {}p)", dash.guild_name(), dash.noblesse_skill_points());
    /// # });
    /// ```
    pub async fn get_character_dashboard_by_name(
        &self,
        character_name: &str,
    ) -> Result<CharacterDashboard> {
        let ocid = self.get_ocid(character_name).await?;
        self.get_character_dashboard(&ocid, None).await
    }

    /// Fetches all frames of an animation preset for the given avatar code,
    /// and renders them into an animated GIF byte buffer.
    ///
    /// Available when the `animation` feature is enabled.
    ///
    /// # Arguments
    /// * `avatar_code` - 1024-character hex avatar code or character look URL
    /// * `preset` - animation preset such as `AnimationPreset::Walk`, `AnimationPreset::Wink`, etc.
    /// * `width` - optional width in pixels (e.g. `Some(96)`)
    /// * `height` - optional height in pixels (e.g. `Some(96)`)
    #[cfg(feature = "animation")]
    pub async fn render_avatar_gif(
        &self,
        avatar_code: &crate::avatar::AvatarCode,
        preset: &crate::avatar::AnimationPreset,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Vec<u8>> {
        let frame_specs = avatar_code.animation_frame_urls(preset, width, height);

        let fetch_futures = frame_specs.into_iter().map(|(url, delay_ms)| {
            let http = self.http.clone();
            async move {
                let resp = http.get(&url).send().await?;
                let status = resp.status();
                if !status.is_success() {
                    return Err(crate::MapleStoryError::Other(format!(
                        "Failed to fetch avatar frame from {}: HTTP {}",
                        url, status
                    )));
                }
                let bytes = resp.bytes().await?.to_vec();
                Ok::<_, crate::MapleStoryError>((bytes, delay_ms))
            }
        });

        let results = futures_util::future::join_all(fetch_futures).await;
        let mut frames_data = Vec::with_capacity(results.len());
        for res in results {
            frames_data.push(res?);
        }

        crate::avatar::encode_animated_gif(&frames_data)
    }

    /// Fetches the character's current look and renders an animated GIF directly by character name.
    ///
    /// Available when the `animation` feature is enabled.
    ///
    /// # Arguments
    /// * `character_name` - In-game character nickname
    /// * `preset` - Animation preset (`Walk`, `Wink`, `Stand`, etc.)
    /// * `width` - optional width in pixels (e.g. `Some(96)`)
    /// * `height` - optional height in pixels (e.g. `Some(96)`)
    #[cfg(feature = "animation")]
    pub async fn render_character_gif_by_name(
        &self,
        character_name: &str,
        preset: &crate::avatar::AnimationPreset,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Vec<u8>> {
        let basic = self.get_character_by_name(character_name).await?;
        let img_url = basic.character_image.ok_or_else(|| {
            crate::MapleStoryError::Other(format!(
                "Character '{}' does not have a valid character image URL",
                character_name
            ))
        })?;
        let avatar_code = crate::avatar::AvatarCode::from_url_or_code(&img_url)?;
        self.render_avatar_gif(&avatar_code, preset, width, height).await
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
