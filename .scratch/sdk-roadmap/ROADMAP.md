# MapleStory OpenAPI Rust SDK — Complete Roadmap & Ticket Status

> **Overall Status**: 🚀 **23 / 30 Tickets Completed (77% - M1~M5 Complete, M6 Ready for Agent)**  
> **Repository Baseline**: `main`  
> **Quality Gate**: `cargo test` (0 failures across all crates), `cargo clippy` (0 warnings with `-D warnings`)

---

## 📊 Milestone Summary

| Milestone | Scope | Tickets | Status |
|---|---|---|---|
| **M1: Resilience & Foundation** | 동시성 제어, 지수 백오프 재시도, KST 날짜 검증, 식별자 캐시 | #01, #02, #03, #06 | ✅ Completed |
| **M2: Client Ergonomics & Streaming** | 계층형 도메인 서브클라이언트, 자동 페이지네이션 스트림, 순수 대시보드 조립기, 아바타 클라이언트 | #07, #08, #19, #20 | ✅ Completed |
| **M3: Deep Domain Modeling & Spec Analytics** | 잠재옵션 파서, 세트효과/아티팩트, 유니온 레이더/챔피언, 성장 헬퍼, 종합 애그리게이터, 전투력/보스방무/무릉 분석, 직업별 포스 스케일링, 장비 레이어링 엔진 | #04, #05, #10, #13, #14, #18, #21, #23 | ✅ Completed |
| **M4: Multi-Language Bindings & C-ABI** | Python, Node.js 확장, C-ABI 진단 에러 & 고레버리지 타입형 전투 요약체/코디 | #09, #12, #22 | ✅ Completed |
| **M5: Quality, CI/CD & Release Readiness** | E2E 라이브 API 테스트, 문서화 동기화, 수동 CI 워크플로우, 패키징/릴리스 스크립트 | #11, #15, #16, #17 | ✅ Completed |
| **M6: Advanced Simulators & Ecosystem** | 장비 교체 시뮬레이터, 스타포스/큐브 기댓값 계산기, 헥사 매트릭스 잔여량, 영속 디스크 캐시, 대량 크롤러, WebP/스프라이트시트, CLI 도구 | #24, #25, #26, #27, #28, #29, #30 | 🚀 In Progress (Ready for Agent) |

---

## 📋 Comprehensive Ticket Matrix (01 ~ 30)

| # | Ticket Name | Core Deliverables | Test Surface | Status |
|---|---|---|---|:---:|
| **01** | [In-Flight Concurrency Limiter](issues/01-concurrency-limiter.md) | `Semaphore` 기반 동시 요청 수 제한 (`max_concurrency`), 대시보드 20개 팬아웃 제어 | `test_client_concurrency_limiter_builder_and_default`, `test_concurrency_limiter_throttles_in_flight_permits` | ✅ Done |
| **02** | [Exponential Backoff Retry](issues/02-exponential-backoff-retry.md) | HTTP 429/503 발생 시 jitter 포함 자동 지수 백오프 재시도 (`max_retries`) | `test_client_retry_and_recovery_on_rate_limit_429`, `test_client_retry_and_recovery_on_transient_503` | ✅ Done |
| **03** | [Flexible Date Input & KST Validation](issues/03-flexible-date-input-kst-validation.md) | `MapleDate`, `DateParam`, KST 오전 01:00 갱신 기준 유효성 검증 및 일관된 포맷팅 | `test_maple_date_conversions`, `test_maple_date_validation_boundaries`, `test_maple_date_latest_available_and_kst_refresh` | ✅ Done |
| **04** | [Equipment Potential Parser](issues/04-equipment-potential-parser.md) | 콜론/비콜론 정규식 옵션 파서 (`ParsedPotentialOption`), `StatType` 자동 분류 및 장비 총 스탯 합산 | `test_equipment_potential_parser_and_total_stat`, `test_equipment_helpers` | ✅ Done |
| **05** | [Union Artifact & Set Effect Helpers](issues/05-union-artifact-set-effect-helpers.md) | 아티팩트 효과 수치 합산 (`total_artifact_stats_typed`), 장비 세트효과 활성화 계산 (`active_set_effects`) | `test_union_artifact_helpers`, `test_set_effect_helpers` | ✅ Done |
| **06** | [Pluggable Identity Cache & Guild Caching](issues/06-pluggable-identity-cache-guild.md) | `IdentityCache` 트레이트, `InMemoryIdentityCache`, 캐릭터 OCID 및 길드 OGuildID 캐싱 | `test_identity_cache_custom_and_guild_caching` | ✅ Done |
| **07** | [Hierarchical Domain Namespaces](issues/07-hierarchical-domain-namespaces.md) | `client.character()`, `guild()`, `union()`, `ranking()`, `history()`, `notice()`, `user()` 등 계층 구조 | `test_hierarchical_domain_namespaces` | ✅ Done |
| **08** | [Auto-Paginating History Streams](issues/08-auto-paginating-history-streams.md) | 스타포스, 큐브, 잠재능력, 소울 히스토리 커서 기반 비동기 `Stream` 자동 페이지네이션 | `test_history_auto_paginating_stream` | ✅ Done |
| **09** | [Multi-Language Bindings Expansion](issues/09-multilang-bindings-expansion.md) | Python(`PyMapleStoryClient`) 및 Node.js(`JsMapleStoryClient`) 도메인 엔드포인트 전면 확장 | PyO3 / NAPI-RS 바인딩 빌드 및 단위 테스트 | ✅ Done |
| **10** | [Rich Domain Aggregators](issues/10-rich-domain-aggregators.md) | `GuildProfile`(길드원 전체 프로필/전투력 병렬 취합), `AccountSummary`(계정 캐릭터/업적/유니온 결합) | `test_guild_profile_and_account_summary_aggregators` | ✅ Done |
| **11** | [Live API & E2E Verification](issues/11-live-api-and-e2e-verification.md) | 실제 넥슨 OpenAPI 통합 테스트 및 오프라인 Fixture 리플레이 폴백 | `test_live_rankings_and_notices`, `test_live_lookup_and_coordination_for_target_characters` 등 6종 | ✅ Done |
| **12** | [C-ABI & Native Wrappers Expansion](issues/12-c-abi-and-native-wrappers-expansion.md) | C-ABI 핸들 검증, 스레드 로컬 진단 에러(`maplestory_last_error_message`), C 헤더 완비 | `test_null_pointer_records_diagnostic_error_message`, `test_c_abi_notice_union_ranking_guild_and_account_summary` | ✅ Done |
| **13** | [Union Raider & Champion Domain Helpers](issues/13-union-raider-and-champion-helpers.md) | 유니온 공격대원 배치/점령 스탯 합산, 활성 프리셋 조회, 챔피언 배지 스탯 정량 분석 | `test_union_raider_and_champion_helpers` | ✅ Done |
| **14** | [Character Progression Domain Helpers](issues/14-character-progression-domain-helpers.md) | HEXA 코어 레벨 조회, 헥사스탯 메인/서브 추출, V매트릭스 만렙 코어, 어빌리티/하이퍼스탯 프리셋 | `test_character_progression_domain_helpers` | ✅ Done |
| **15** | [Documentation & Examples Synchronization](issues/15-docs-and-examples-sync.md) | README 및 docs 3종 개정, 도메인 서브클라이언트 및 신규 애그리게이터 예제 동기화 | Rust Doc-tests (12개 전수 통과) | ✅ Done |
| **16** | [Manual-Only CI/CD Workflows](issues/16-manual-ci-cd-workflows.md) | `.github/workflows/ci.yml` `workflow_dispatch` 전용 수동 트리거 및 크로스 플랫폼 매트릭스 | GitHub Actions YAML linter / schema check | ✅ Done |
| **17** | [Packaging & Release Readiness](issues/17-packaging-and-release-readiness.md) | `Cargo.toml`, npm `package.json`, `pyproject.toml` 메타데이터 최신화 및 `scripts/verify-release.ps1` | `verify-release.ps1` 드라이런 검증 | ✅ Done |
| **18** | [Battle Analysis & Combat Analytics](issues/18-battle-analysis-and-combat-analytics.md) | `CharacterCombatAnalytics`, 300%/380% 보스 유효 방무 계수, 크리티컬/보스뎀 팩터, 무릉 층수 추정 | `test_character_combat_analytics` | ✅ Done |
| **19** | [Pure Dashboard Assembler](issues/19-pure-dashboard-assembler.md) | 순수 동기 `CharacterDashboard::assemble`, 네트워크 없는 부분 실패/폴백 분리, 보일러플레이트 단축 | `test_pure_character_dashboard_assemble_and_partial_failures` | ✅ Done |
| **20** | [Dedicated Avatar Client](issues/20-dedicated-avatar-client.md) | `client.avatar()` (`AvatarClient`), 멀티프레임 GIF 생성 및 프레임 페칭 독립 서브클라이언트 | `test_avatar_client_pipeline_and_custom_frames` | ✅ Done |
| **21** | [Decoupled Combat Analytics & Job Force Scaling](issues/21-combat-analytics-job-force-scaling.md) | `from_stats(...)` 독립 생성자, 제논(+48)/데몬어벤져(+2100/+4200) 직업별 심볼 스케일링 수식 | `test_combat_analytics_decoupled_and_job_specific_force_scaling` | ✅ Done |
| **22** | [C-ABI High-Leverage Typed Summaries](issues/22-c-abi-high-leverage-summaries.md) | C 구조체 `MapleStoryCombatSummary`, OCID/닉네임 기반 C-ABI 함수 제공, 무파싱 고속 연동 | `test_c_abi_notice_union_ranking_guild_and_account_summary` | ✅ Done |
| **23** | [Equip Layering Engine](issues/23-equip-layer-resolver.md) | `EquipLayerResolver::resolve`, 캐시장비 우선, 한벌옷 하의 가림, 투명 장비 감지, 모루 외형 대체 | `test_equip_layer_resolver_direct_and_custom_rules` | ✅ Done |
| **24** | [Gear Swap Simulation & Stat Diff](issues/24-gear-swap-simulation-and-stat-diff.md) | 장비 교체 시 세트 효과/스탯/전투력 변동치 역산 시뮬레이터 (`EquipmentDiff`, `StatDiff`) | 순수 단위 테스트 (세트 효과 변동, 스탯 역산) | ⏳ Ready |
| **25** | [Starforce & Cube Expected Cost Calculator](issues/25-starforce-cube-expected-cost-calculator.md) | 스타포스(마르코프 체인) 및 큐브 목표 달성 소요 비용/확률/기댓값 계산 엔진 | 마르코프 체인 비용 수렴 및 컷오프 확률 테스트 | ✅ Done |
| **26** | [HEXA Matrix Progression Calculator](issues/26-hexa-matrix-progression-calculator.md) | 6차 전직 헥사 매트릭스 코어/스탯별 30레벨 만렙 달성 잔여 솔 에르다/조각 수량 계산 | 코어 타입별 재료 합산 단위 테스트 | ⏳ Ready |
| **27** | [Persistent Disk Cache Adapter](issues/27-persistent-disk-cache-adapter.md) | 파일 시스템 기반 영속 식별자 캐시 (`DiskIdentityCache`) 구현체 (TTL, 동시성 보장) | 파일 캐시 히트/미스 및 프로세스 재시작 시뮬레이션 | ⏳ Ready |
| **28** | [Batch Character & Guild Crawler](issues/28-batch-character-and-guild-crawler.md) | 길드원/계정 캐릭터 대량 병렬 수집기 (`client.guild().fetch_roster_profiles()`) | ConcurrencyLimiter 연동 및 부분 실패 내구성 테스트 | ⏳ Ready |
| **29** | [WebP & Spritesheet Avatar Engine](issues/29-avatar-webp-and-spritesheet-engine.md) | 아바타 WebP 애니메이션 인코더 및 2D 스프라이트시트 PNG + JSON 메타데이터 패커 | WebP 및 스프라이트시트 이미지 생성 테스트 | ⏳ Ready |
| **30** | [Maple CLI Terminal Utility](issues/30-maple-cli-lightweight-terminal-tool.md) | 터미널 대시보드, 아바타 다운로드, 캐릭터 비교, 스타포스 계산 독립 CLI 크레이트 (`maple-cli`) | CLI 인수 파싱 및 통합 테스트 | ⏳ Ready |

---

## 🔍 Quality & Architecture Metrics

```
[Rust Core]
  - maplestory-openapi
    * 7 unit tests (avatar code, look query builder, calendar validation, base URL)
    * 15 deserialization tests (all Nexon response structures)
    * 30 helpers & domain tests (concurrency, retry, parsing, progression, analytics, layering)
    * 6 live API & offline fixture replay tests
    * 12 rustdoc doctests
  - Result: 70 tests passed, 0 failures.

[C-ABI]
  - maplestory-openapi-c
    * Memory safety & null pointer diagnostic error handling
    * End-to-end mock server tests (combat summary struct, coordination, rankings, guilds)
  - Result: 2 tests passed, 0 failures.

[Multi-Language Native Bindings]
  - Node.js (napi-rs): 빌드 및 동기화 완료
  - Python (pyo3): 빌드 및 동기화 완료
  - C# (.NET): C-ABI P/Invoke 래퍼 동기화 완료
  - WASM (wasm-bindgen): 웹 타깃 빌드 검증 완료

[Clippy Linter]
  - `cargo clippy --all-targets --all-features -- -D warnings`: 0 warnings
```
