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
        CharacterSymbolEquipment, CharacterVMatrix, DashboardOptions, DashboardSection,
        DashboardSectionError, RingExchangeSkillEquipment, RingReserveSkillEquipment,
    },
    models::common::{DateParam, SkillGrade},
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
        if let Some(cached_ocid) = self.identity_cache.get_ocid(character_name).await {
            tracing::debug!("OCID cache hit for '{}'", character_name);
            return Ok(cached_ocid);
        }

        let result = self.get_character_id(character_name).await?;
        let ocid = result
            .ocid
            .ok_or_else(|| MapleStoryError::CharacterNotFound {
                name: character_name.to_string(),
            })?;

        self.identity_cache.set_ocid(character_name, &ocid).await;

        Ok(ocid)
    }

    /// Internal helper to resolve an OCID by character name and execute a closure with it.
    async fn with_ocid<T, F, Fut>(&self, character_name: &str, f: F) -> Result<T>
    where
        F: FnOnce(String) -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let ocid = self.get_ocid(character_name).await?;
        f(ocid).await
    }

    // ─── Convenience: 닉네임 간편 조회 메서드군 ───────────────────────────────

    /// **Quick lookup by character name** — retrieves basic character info with a
    /// single method call.
    pub async fn get_character_by_name(&self, character_name: &str) -> Result<CharacterBasic> {
        self.with_ocid(character_name, |ocid| async move {
            self.get_character_basic(&ocid, None).await
        })
        .await
    }

    /// Retrieve character comprehensive stats directly by character name.
    pub async fn get_character_stat_by_name(&self, character_name: &str) -> Result<CharacterStat> {
        self.with_ocid(character_name, |ocid| async move {
            self.get_character_stat(&ocid, None).await
        })
        .await
    }

    /// Retrieve character equipped items directly by character name.
    pub async fn get_character_item_equipment_by_name(
        &self,
        character_name: &str,
    ) -> Result<CharacterItemEquipment> {
        self.with_ocid(character_name, |ocid| async move {
            self.get_character_item_equipment(&ocid, None).await
        })
        .await
    }

    /// Retrieve character cash items directly by character name.
    pub async fn get_character_cashitem_equipment_by_name(
        &self,
        character_name: &str,
    ) -> Result<CharacterCashItemEquipment> {
        self.with_ocid(character_name, |ocid| async move {
            self.get_character_cashitem_equipment(&ocid, None).await
        })
        .await
    }

    /// Retrieve character beauty (hair, face, skin) directly by character name.
    pub async fn get_character_beauty_equipment_by_name(
        &self,
        character_name: &str,
    ) -> Result<CharacterBeautyEquipment> {
        self.with_ocid(character_name, |ocid| async move {
            self.get_character_beauty_equipment(&ocid, None).await
        })
        .await
    }

    /// Retrieve character popularity directly by character name.
    pub async fn get_character_popularity_by_name(
        &self,
        character_name: &str,
    ) -> Result<CharacterPopularity> {
        self.with_ocid(character_name, |ocid| async move {
            self.get_character_popularity(&ocid, None).await
        })
        .await
    }

    /// Retrieve character Mu Lung Dojo record directly by character name.
    pub async fn get_character_dojang_by_name(
        &self,
        character_name: &str,
    ) -> Result<CharacterDojang> {
        self.with_ocid(character_name, |ocid| async move {
            self.get_character_dojang(&ocid, None).await
        })
        .await
    }

    /// Retrieve character skills directly by character name.
    pub async fn get_character_skill_by_name(
        &self,
        character_name: &str,
        grade: SkillGrade,
    ) -> Result<CharacterSkill> {
        self.with_ocid(character_name, |ocid| async move {
            self.get_character_skill(&ocid, grade, None).await
        })
        .await
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
        date: impl Into<DateParam>,
    ) -> Result<CharacterCoordination> {
        let date_param = date.into();
        let date_ref = date_param.as_deref();
        let ((basic_res, beauty_res), (cash_res, equip_res)) = futures_util::future::join(
            futures_util::future::join(
                self.get_character_basic(ocid, date_ref),
                self.get_character_beauty_equipment(ocid, date_ref),
            ),
            futures_util::future::join(
                self.get_character_cashitem_equipment(ocid, date_ref),
                self.get_character_item_equipment(ocid, date_ref),
            ),
        )
        .await;

        let basic = basic_res?;
        let beauty = beauty_res?;
        let cash = cash_res?;
        let equip = equip_res?;

        Ok(CharacterCoordination::from_full_parts(
            &basic, &beauty, &cash, &equip,
        ))
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
        self.get_character_coordination_by_name(character_name)
            .await
    }

    /// Alias for [`get_character_coordination_by_name`](Self::get_character_coordination_by_name).
    pub async fn get_character_appearance_by_name(
        &self,
        character_name: &str,
    ) -> Result<CharacterAppearance> {
        self.get_character_coordination_by_name(character_name)
            .await
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
        date: impl Into<DateParam>,
    ) -> Result<CharacterProfile> {
        let date_param = date.into();
        let date_ref = date_param.as_deref();
        let ((basic, stat), (equip, cash), (beauty, pop, dojang)) = futures_util::future::join3(
            futures_util::future::join(
                self.get_character_basic(ocid, date_ref),
                self.get_character_stat(ocid, date_ref),
            ),
            futures_util::future::join(
                self.get_character_item_equipment(ocid, date_ref),
                self.get_character_cashitem_equipment(ocid, date_ref),
            ),
            futures_util::future::join3(
                self.get_character_beauty_equipment(ocid, date_ref),
                self.get_character_popularity(ocid, date_ref),
                self.get_character_dojang(ocid, date_ref),
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

    /// Retrieve a comprehensive all-in-one character dashboard using custom [`DashboardOptions`].
    ///
    /// Allows callers to select specific sections, reducing network round-trips and
    /// preserving rate limits. Diagnostic errors from non-critical endpoints are
    /// recorded in [`CharacterDashboard::errors`].
    pub async fn get_character_dashboard_with_options(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
        options: &DashboardOptions,
    ) -> Result<CharacterDashboard> {
        let date_param = date.into();
        let date = date_param.as_deref();
        // Pipeline guild lookup: starts as soon as `basic` completes, running concurrently
        // with other character endpoints.
        let basic_and_guild_fut = async {
            let basic_res = self.get_character_basic(ocid, date).await;
            let guild_res = match &basic_res {
                Ok(basic) if options.is_enabled(DashboardSection::Guild) => {
                    match (
                        basic.character_guild_name.as_deref(),
                        basic.world_name.as_deref(),
                    ) {
                        (Some(guild_name), Some(world_name))
                            if !guild_name.is_empty() && !world_name.is_empty() =>
                        {
                            Some(self.get_guild_by_name(guild_name, world_name, date).await)
                        }
                        _ => None,
                    }
                }
                _ => None,
            };
            (basic_res, guild_res)
        };

        let stat_fut = async {
            if options.is_enabled(DashboardSection::Stat) {
                Some(self.get_character_stat(ocid, date).await)
            } else {
                None
            }
        };

        let equip_fut = async {
            if options.is_enabled(DashboardSection::ItemEquipment) {
                Some(self.get_character_item_equipment(ocid, date).await)
            } else {
                None
            }
        };

        let cash_fut = async {
            if options.is_enabled(DashboardSection::CashItemEquipment) {
                Some(self.get_character_cashitem_equipment(ocid, date).await)
            } else {
                None
            }
        };

        let beauty_fut = async {
            if options.is_enabled(DashboardSection::BeautyEquipment) {
                Some(self.get_character_beauty_equipment(ocid, date).await)
            } else {
                None
            }
        };

        let symbols_fut = async {
            if options.is_enabled(DashboardSection::Symbols) {
                Some(self.get_character_symbol_equipment(ocid, date).await)
            } else {
                None
            }
        };

        let ability_fut = async {
            if options.is_enabled(DashboardSection::Ability) {
                Some(self.get_character_ability(ocid, date).await)
            } else {
                None
            }
        };

        let hyper_stat_fut = async {
            if options.is_enabled(DashboardSection::HyperStat) {
                Some(self.get_character_hyper_stat(ocid, date).await)
            } else {
                None
            }
        };

        let hexa_matrix_fut = async {
            if options.is_enabled(DashboardSection::HexaMatrix) {
                Some(self.get_character_hexamatrix(ocid, date).await)
            } else {
                None
            }
        };

        let hexa_matrix_stat_fut = async {
            if options.is_enabled(DashboardSection::HexaMatrixStat) {
                Some(self.get_character_hexamatrix_stat(ocid, date).await)
            } else {
                None
            }
        };

        let v_matrix_fut = async {
            if options.is_enabled(DashboardSection::VMatrix) {
                Some(self.get_character_vmatrix(ocid, date).await)
            } else {
                None
            }
        };

        let set_effect_fut = async {
            if options.is_enabled(DashboardSection::SetEffect) {
                Some(self.get_character_set_effect(ocid, date).await)
            } else {
                None
            }
        };

        let pet_fut = async {
            if options.is_enabled(DashboardSection::PetEquipment) {
                Some(self.get_character_pet_equipment(ocid, date).await)
            } else {
                None
            }
        };

        let android_fut = async {
            if options.is_enabled(DashboardSection::AndroidEquipment) {
                Some(self.get_character_android_equipment(ocid, date).await)
            } else {
                None
            }
        };

        let link_fut = async {
            if options.is_enabled(DashboardSection::LinkSkill) {
                Some(self.get_character_link_skill(ocid, date).await)
            } else {
                None
            }
        };

        let propensity_fut = async {
            if options.is_enabled(DashboardSection::Propensity) {
                Some(self.get_character_propensity(ocid, date).await)
            } else {
                None
            }
        };

        let popularity_fut = async {
            if options.is_enabled(DashboardSection::Popularity) {
                Some(self.get_character_popularity(ocid, date).await)
            } else {
                None
            }
        };

        let dojang_fut = async {
            if options.is_enabled(DashboardSection::Dojang) {
                Some(self.get_character_dojang(ocid, date).await)
            } else {
                None
            }
        };

        let union_fut = async {
            if options.is_enabled(DashboardSection::Union) {
                Some(self.get_user_union(ocid, date).await)
            } else {
                None
            }
        };

        let union_artifact_fut = async {
            if options.is_enabled(DashboardSection::UnionArtifact) {
                Some(self.get_user_union_artifact(ocid, date).await)
            } else {
                None
            }
        };

        let (
            (basic_res, guild_res),
            stat_res,
            equip_res,
            cash_res,
            beauty_res,
            symbols_res,
            ability_res,
            hyper_stat_res,
            hexa_matrix_res,
            hexa_matrix_stat_res,
            v_matrix_res,
            set_effect_res,
            pet_res,
            android_res,
            link_res,
            propensity_res,
            popularity_res,
            dojang_res,
            union_res,
            union_artifact_res,
        ) = futures_util::join!(
            basic_and_guild_fut,
            stat_fut,
            equip_fut,
            cash_fut,
            beauty_fut,
            symbols_fut,
            ability_fut,
            hyper_stat_fut,
            hexa_matrix_fut,
            hexa_matrix_stat_fut,
            v_matrix_fut,
            set_effect_fut,
            pet_fut,
            android_fut,
            link_fut,
            propensity_fut,
            popularity_fut,
            dojang_fut,
            union_fut,
            union_artifact_fut,
        );

        let basic = basic_res?;
        let mut errors = std::collections::HashMap::new();

        let stat = match stat_res {
            Some(Ok(s)) => s,
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::Stat,
                    DashboardSectionError::from_error(DashboardSection::Stat, &e),
                );
                CharacterStat {
                    date: None,
                    character_class: basic.character_class.clone(),
                    final_stat: None,
                    remain_ap: None,
                }
            }
            None => CharacterStat {
                date: None,
                character_class: basic.character_class.clone(),
                final_stat: None,
                remain_ap: None,
            },
        };

        let item_equipment = match equip_res {
            Some(Ok(eq)) => eq,
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::ItemEquipment,
                    DashboardSectionError::from_error(DashboardSection::ItemEquipment, &e),
                );
                CharacterItemEquipment {
                    date: None,
                    character_gender: None,
                    character_class: basic.character_class.clone(),
                    preset_no: None,
                    item_equipment: None,
                    item_equipment_preset_1: None,
                    item_equipment_preset_2: None,
                    item_equipment_preset_3: None,
                    title: None,
                    dragon_equipment: None,
                    mechanic_equipment: None,
                }
            }
            None => CharacterItemEquipment {
                date: None,
                character_gender: None,
                character_class: basic.character_class.clone(),
                preset_no: None,
                item_equipment: None,
                item_equipment_preset_1: None,
                item_equipment_preset_2: None,
                item_equipment_preset_3: None,
                title: None,
                dragon_equipment: None,
                mechanic_equipment: None,
            },
        };

        let cash_item_equipment = match cash_res {
            Some(Ok(c)) => c,
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::CashItemEquipment,
                    DashboardSectionError::from_error(DashboardSection::CashItemEquipment, &e),
                );
                CharacterCashItemEquipment::default()
            }
            None => CharacterCashItemEquipment::default(),
        };

        let beauty_equipment = match beauty_res {
            Some(Ok(b)) => b,
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::BeautyEquipment,
                    DashboardSectionError::from_error(DashboardSection::BeautyEquipment, &e),
                );
                CharacterBeautyEquipment::default()
            }
            None => CharacterBeautyEquipment::default(),
        };

        let coordination = CharacterCoordination::from_full_parts(
            &basic,
            &beauty_equipment,
            &cash_item_equipment,
            &item_equipment,
        );

        let symbols = match symbols_res {
            Some(Ok(s)) => Some(s),
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::Symbols,
                    DashboardSectionError::from_error(DashboardSection::Symbols, &e),
                );
                None
            }
            None => None,
        };

        let ability = match ability_res {
            Some(Ok(a)) => Some(a),
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::Ability,
                    DashboardSectionError::from_error(DashboardSection::Ability, &e),
                );
                None
            }
            None => None,
        };

        let hyper_stat = match hyper_stat_res {
            Some(Ok(h)) => Some(h),
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::HyperStat,
                    DashboardSectionError::from_error(DashboardSection::HyperStat, &e),
                );
                None
            }
            None => None,
        };

        let hexa_matrix = match hexa_matrix_res {
            Some(Ok(h)) => Some(h),
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::HexaMatrix,
                    DashboardSectionError::from_error(DashboardSection::HexaMatrix, &e),
                );
                None
            }
            None => None,
        };

        let hexa_matrix_stat = match hexa_matrix_stat_res {
            Some(Ok(h)) => Some(h),
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::HexaMatrixStat,
                    DashboardSectionError::from_error(DashboardSection::HexaMatrixStat, &e),
                );
                None
            }
            None => None,
        };

        let v_matrix = match v_matrix_res {
            Some(Ok(v)) => Some(v),
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::VMatrix,
                    DashboardSectionError::from_error(DashboardSection::VMatrix, &e),
                );
                None
            }
            None => None,
        };

        let set_effect = match set_effect_res {
            Some(Ok(s)) => Some(s),
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::SetEffect,
                    DashboardSectionError::from_error(DashboardSection::SetEffect, &e),
                );
                None
            }
            None => None,
        };

        let pet_equipment = match pet_res {
            Some(Ok(p)) => Some(p),
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::PetEquipment,
                    DashboardSectionError::from_error(DashboardSection::PetEquipment, &e),
                );
                None
            }
            None => None,
        };

        let android_equipment = match android_res {
            Some(Ok(a)) => Some(a),
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::AndroidEquipment,
                    DashboardSectionError::from_error(DashboardSection::AndroidEquipment, &e),
                );
                None
            }
            None => None,
        };

        let link_skill = match link_res {
            Some(Ok(l)) => Some(l),
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::LinkSkill,
                    DashboardSectionError::from_error(DashboardSection::LinkSkill, &e),
                );
                None
            }
            None => None,
        };

        let propensity = match propensity_res {
            Some(Ok(p)) => Some(p),
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::Propensity,
                    DashboardSectionError::from_error(DashboardSection::Propensity, &e),
                );
                None
            }
            None => None,
        };

        let popularity = match popularity_res {
            Some(Ok(p)) => Some(p),
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::Popularity,
                    DashboardSectionError::from_error(DashboardSection::Popularity, &e),
                );
                None
            }
            None => None,
        };

        let dojang = match dojang_res {
            Some(Ok(d)) => Some(d),
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::Dojang,
                    DashboardSectionError::from_error(DashboardSection::Dojang, &e),
                );
                None
            }
            None => None,
        };

        let union = match union_res {
            Some(Ok(u)) => Some(u),
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::Union,
                    DashboardSectionError::from_error(DashboardSection::Union, &e),
                );
                None
            }
            None => None,
        };

        let union_artifact = match union_artifact_res {
            Some(Ok(u)) => Some(u),
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::UnionArtifact,
                    DashboardSectionError::from_error(DashboardSection::UnionArtifact, &e),
                );
                None
            }
            None => None,
        };

        let guild = match guild_res {
            Some(Ok(g)) => Some(g),
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::Guild,
                    DashboardSectionError::from_error(DashboardSection::Guild, &e),
                );
                None
            }
            None => None,
        };

        Ok(CharacterDashboard {
            basic,
            stat,
            item_equipment,
            cash_item_equipment,
            beauty_equipment,
            coordination,
            symbols,
            ability,
            hyper_stat,
            hexa_matrix,
            hexa_matrix_stat,
            v_matrix,
            set_effect,
            pet_equipment,
            android_equipment,
            link_skill,
            propensity,
            popularity,
            dojang,
            union,
            union_artifact,
            guild,
            errors,
        })
    }

    /// Retrieve a comprehensive all-in-one character dashboard (올인원 캐릭터 종합 대시보드).
    ///
    /// Fetches all specs, equipment, symbols, abilities, 6th HEXA matrix,
    /// 5th V matrix, union, artifacts, and guild information concurrently.
    ///
    /// Non-critical or tier-dependent endpoints (like 6th job for low level chars,
    /// or guild info for guildless chars) gracefully fall back to `None`, with
    /// failure details recorded in [`CharacterDashboard::errors`].
    pub async fn get_character_dashboard(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterDashboard> {
        self.get_character_dashboard_with_options(ocid, date, &DashboardOptions::all())
            .await
    }

    /// Retrieve an all-in-one character dashboard with custom options directly by character name.
    pub async fn get_character_dashboard_with_options_by_name(
        &self,
        character_name: &str,
        date: impl Into<DateParam>,
        options: &DashboardOptions,
    ) -> Result<CharacterDashboard> {
        let ocid = self.get_ocid(character_name).await?;
        self.get_character_dashboard_with_options(&ocid, date, options)
            .await
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
        self.get_character_dashboard_with_options_by_name(
            character_name,
            None,
            &DashboardOptions::all(),
        )
        .await
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
        self.render_avatar_gif(&avatar_code, preset, width, height)
            .await
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
        date: impl Into<DateParam>,
    ) -> Result<CharacterBasic> {
        self.character().get_basic(ocid, date).await
    }

    // ─── Popularity ──────────────────────────────────────────────────────────

    /// Get character popularity.
    ///
    /// Corresponds to `GET /maplestory/v1/character/popularity`.
    pub async fn get_character_popularity(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterPopularity> {
        self.character().get_popularity(ocid, date).await
    }

    // ─── Stat ────────────────────────────────────────────────────────────────

    /// Get character comprehensive stats.
    ///
    /// Corresponds to `GET /maplestory/v1/character/stat`.
    pub async fn get_character_stat(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterStat> {
        self.character().get_stat(ocid, date).await
    }

    // ─── Hyper Stat ──────────────────────────────────────────────────────────

    /// Get character hyper stat information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/hyper-stat`.
    pub async fn get_character_hyper_stat(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterHyperStat> {
        self.character().get_hyper_stat(ocid, date).await
    }

    // ─── Propensity ──────────────────────────────────────────────────────────

    /// Get character propensity (성향) information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/propensity`.
    pub async fn get_character_propensity(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterPropensity> {
        self.character().get_propensity(ocid, date).await
    }

    // ─── Ability ─────────────────────────────────────────────────────────────

    /// Get character ability (어빌리티) information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/ability`.
    pub async fn get_character_ability(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterAbility> {
        self.character().get_ability(ocid, date).await
    }

    // ─── Item Equipment ───────────────────────────────────────────────────────

    /// Get equipped item information (장착 장비).
    ///
    /// Corresponds to `GET /maplestory/v1/character/item-equipment`.
    pub async fn get_character_item_equipment(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterItemEquipment> {
        self.character().get_item_equipment(ocid, date).await
    }

    // ─── Cash Item Equipment ─────────────────────────────────────────────────

    /// Get equipped cash item information (장착 캐시 장비).
    ///
    /// Corresponds to `GET /maplestory/v1/character/cashitem-equipment`.
    pub async fn get_character_cashitem_equipment(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterCashItemEquipment> {
        self.character().get_cashitem_equipment(ocid, date).await
    }

    // ─── Symbol Equipment ─────────────────────────────────────────────────────

    /// Get equipped symbol information (장착 심볼).
    ///
    /// Corresponds to `GET /maplestory/v1/character/symbol-equipment`.
    pub async fn get_character_symbol_equipment(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterSymbolEquipment> {
        self.character().get_symbol_equipment(ocid, date).await
    }

    // ─── Set Effect ───────────────────────────────────────────────────────────

    /// Get applied set effect information (세트 효과).
    ///
    /// Corresponds to `GET /maplestory/v1/character/set-effect`.
    pub async fn get_character_set_effect(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterSetEffect> {
        self.character().get_set_effect(ocid, date).await
    }

    // ─── Beauty Equipment ─────────────────────────────────────────────────────

    /// Get beauty (헤어/성형/피부) equipment information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/beauty-equipment`.
    pub async fn get_character_beauty_equipment(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterBeautyEquipment> {
        self.character().get_beauty_equipment(ocid, date).await
    }

    // ─── Android Equipment ────────────────────────────────────────────────────

    /// Get android equipment information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/android-equipment`.
    pub async fn get_character_android_equipment(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterAndroidEquipment> {
        self.character().get_android_equipment(ocid, date).await
    }

    // ─── Pet Equipment ────────────────────────────────────────────────────────

    /// Get pet equipment information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/pet-equipment`.
    pub async fn get_character_pet_equipment(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterPetEquipment> {
        self.character().get_pet_equipment(ocid, date).await
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
        date: impl Into<DateParam>,
    ) -> Result<CharacterSkill> {
        self.character().get_skill(ocid, grade, date).await
    }

    // ─── Link Skill ───────────────────────────────────────────────────────────

    /// Get link skill information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/link-skill`.
    pub async fn get_character_link_skill(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterLinkSkill> {
        self.character().get_link_skill(ocid, date).await
    }

    // ─── V Matrix ─────────────────────────────────────────────────────────────

    /// Get V-Matrix information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/vmatrix`.
    pub async fn get_character_vmatrix(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterVMatrix> {
        self.character().get_vmatrix(ocid, date).await
    }

    // ─── HEXA Matrix ──────────────────────────────────────────────────────────

    /// Get HEXA Matrix core information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/hexamatrix`.
    pub async fn get_character_hexamatrix(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterHexaMatrix> {
        self.character().get_hexamatrix(ocid, date).await
    }

    // ─── HEXA Matrix Stat ─────────────────────────────────────────────────────

    /// Get HEXA stat core information.
    ///
    /// Corresponds to `GET /maplestory/v1/character/hexamatrix-stat`.
    pub async fn get_character_hexamatrix_stat(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterHexaMatrixStat> {
        self.character().get_hexamatrix_stat(ocid, date).await
    }

    // ─── Dojang ───────────────────────────────────────────────────────────────

    /// Get Mu Lung Dojo (무릉도장) best record.
    ///
    /// Corresponds to `GET /maplestory/v1/character/dojang`.
    pub async fn get_character_dojang(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterDojang> {
        self.character().get_dojang(ocid, date).await
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
        date: impl Into<DateParam>,
    ) -> Result<CharacterOtherStat> {
        self.character().get_other_stat(ocid, date).await
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
        date: impl Into<DateParam>,
    ) -> Result<RingExchangeSkillEquipment> {
        self.character()
            .get_ring_exchange_skill_equipment(ocid, date)
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
        date: impl Into<DateParam>,
    ) -> Result<RingReserveSkillEquipment> {
        self.character()
            .get_ring_reserve_skill_equipment(ocid, date)
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

// ─── CharacterClient ─────────────────────────────────────────────────────────

/// Domain sub-client for Character tag API endpoints.
#[derive(Clone, Copy)]
pub struct CharacterClient<'a> {
    pub(crate) client: &'a MapleStoryClient,
}

impl<'a> CharacterClient<'a> {
    /// Create a new `CharacterClient` referencing a [`MapleStoryClient`].
    pub fn new(client: &'a MapleStoryClient) -> Self {
        Self { client }
    }

    /// Look up a character's OCID by name.
    pub async fn get_id(&self, character_name: &str) -> Result<CharacterOcid> {
        self.client.get_character_id(character_name).await
    }

    /// Get only the OCID string for a character name (consults identity cache).
    pub async fn get_ocid(&self, character_name: &str) -> Result<String> {
        self.client.get_ocid(character_name).await
    }

    /// Retrieve basic information of a character.
    pub async fn get_basic(&self, ocid: &str, date: impl Into<DateParam>) -> Result<CharacterBasic> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get("/maplestory/v1/character/basic", &params)
            .await
    }

    /// Retrieve popularity of a character.
    pub async fn get_popularity(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterPopularity> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get("/maplestory/v1/character/popularity", &params)
            .await
    }

    /// Retrieve comprehensive stat information of a character.
    pub async fn get_stat(&self, ocid: &str, date: impl Into<DateParam>) -> Result<CharacterStat> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get("/maplestory/v1/character/stat", &params)
            .await
    }

    /// Retrieve hyper stat information of a character.
    pub async fn get_hyper_stat(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterHyperStat> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get("/maplestory/v1/character/hyper-stat", &params)
            .await
    }

    /// Retrieve propensity information of a character.
    pub async fn get_propensity(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterPropensity> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get("/maplestory/v1/character/propensity", &params)
            .await
    }

    /// Retrieve ability information of a character.
    pub async fn get_ability(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterAbility> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get("/maplestory/v1/character/ability", &params)
            .await
    }

    /// Retrieve item equipment information of a character.
    pub async fn get_item_equipment(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterItemEquipment> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get("/maplestory/v1/character/item-equipment", &params)
            .await
    }

    /// Retrieve cash item equipment information of a character.
    pub async fn get_cashitem_equipment(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterCashItemEquipment> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get("/maplestory/v1/character/cashitem-equipment", &params)
            .await
    }

    /// Retrieve symbol equipment information of a character.
    pub async fn get_symbol_equipment(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterSymbolEquipment> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get("/maplestory/v1/character/symbol-equipment", &params)
            .await
    }

    /// Retrieve set effect information of a character.
    pub async fn get_set_effect(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterSetEffect> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get("/maplestory/v1/character/set-effect", &params)
            .await
    }

    /// Retrieve beauty (hair, face, skin) equipment information of a character.
    pub async fn get_beauty_equipment(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterBeautyEquipment> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get("/maplestory/v1/character/beauty-equipment", &params)
            .await
    }

    /// Retrieve android equipment information of a character.
    pub async fn get_android_equipment(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterAndroidEquipment> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get("/maplestory/v1/character/android-equipment", &params)
            .await
    }

    /// Retrieve pet equipment information of a character.
    pub async fn get_pet_equipment(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterPetEquipment> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get("/maplestory/v1/character/pet-equipment", &params)
            .await
    }

    /// Retrieve skill information for a specific grade of a character.
    pub async fn get_skill(
        &self,
        ocid: &str,
        skill_grade: SkillGrade,
        date: impl Into<DateParam>,
    ) -> Result<CharacterSkill> {
        let mut params = build_params(ocid, date.into().as_deref())?;
        params.push(("character_skill_grade", skill_grade.to_string()));
        self.client
            .get("/maplestory/v1/character/skill", &params)
            .await
    }

    /// Retrieve link skill information of a character.
    pub async fn get_link_skill(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterLinkSkill> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get("/maplestory/v1/character/link-skill", &params)
            .await
    }

    /// Retrieve V-Matrix core information of a character.
    pub async fn get_vmatrix(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterVMatrix> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get("/maplestory/v1/character/vmatrix", &params)
            .await
    }

    /// Retrieve HEXA Matrix core information of a character.
    pub async fn get_hexamatrix(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterHexaMatrix> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get("/maplestory/v1/character/hexamatrix", &params)
            .await
    }

    /// Retrieve HEXA Matrix stat node information of a character.
    pub async fn get_hexamatrix_stat(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterHexaMatrixStat> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get("/maplestory/v1/character/hexamatrix-stat", &params)
            .await
    }

    /// Retrieve Mu Lung Dojang best record of a character.
    pub async fn get_dojang(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterDojang> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get("/maplestory/v1/character/dojang", &params)
            .await
    }

    /// Retrieve other stats of a character.
    pub async fn get_other_stat(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterOtherStat> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get("/maplestory/v1/character/other-stat", &params)
            .await
    }

    /// Retrieve ring exchange skill equipment information.
    pub async fn get_ring_exchange_skill_equipment(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<RingExchangeSkillEquipment> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get(
                "/maplestory/v1/character/ring-exchange-skill-equipment",
                &params,
            )
            .await
    }

    /// Retrieve ring reserve skill equipment information.
    pub async fn get_ring_reserve_skill_equipment(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<RingReserveSkillEquipment> {
        let params = build_params(ocid, date.into().as_deref())?;
        self.client
            .get(
                "/maplestory/v1/character/ring-reserve-skill-equipment",
                &params,
            )
            .await
    }

    /// Retrieve synthesized coordination model of a character.
    pub async fn get_coordination(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterCoordination> {
        self.client.get_character_coordination(ocid, date).await
    }

    /// Retrieve synthesized coordination model directly by character name.
    pub async fn get_coordination_by_name(
        &self,
        character_name: &str,
    ) -> Result<CharacterCoordination> {
        self.client
            .get_character_coordination_by_name(character_name)
            .await
    }

    /// Retrieve basic information of a character directly by character name.
    pub async fn get_by_name(&self, character_name: &str) -> Result<CharacterBasic> {
        self.client.get_character_by_name(character_name).await
    }

    /// Retrieve profile information of a character.
    pub async fn get_profile(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterProfile> {
        self.client.get_character_profile(ocid, date).await
    }

    /// Retrieve profile information directly by character name.
    pub async fn get_profile_by_name(&self, character_name: &str) -> Result<CharacterProfile> {
        self.client
            .get_character_profile_by_name(character_name)
            .await
    }

    /// Retrieve character dashboard with default options.
    pub async fn get_dashboard(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterDashboard> {
        self.client.get_character_dashboard(ocid, date).await
    }

    /// Retrieve character dashboard with configurable section masking.
    pub async fn get_dashboard_with_options(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
        options: &DashboardOptions,
    ) -> Result<CharacterDashboard> {
        self.client
            .get_character_dashboard_with_options(ocid, date, options)
            .await
    }

    /// Retrieve character dashboard directly by character name.
    pub async fn get_dashboard_by_name(&self, character_name: &str) -> Result<CharacterDashboard> {
        self.client
            .get_character_dashboard_by_name(character_name)
            .await
    }

    /// Retrieve character dashboard directly by character name with options.
    pub async fn get_dashboard_with_options_by_name(
        &self,
        character_name: &str,
        date: impl Into<DateParam>,
        options: &DashboardOptions,
    ) -> Result<CharacterDashboard> {
        self.client
            .get_character_dashboard_with_options_by_name(character_name, date, options)
            .await
    }
}

