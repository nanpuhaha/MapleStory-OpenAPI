# MapleStory OpenAPI — Rust Client

[![Crates.io](https://img.shields.io/crates/v/maplestory-openapi.svg)](https://crates.io/crates/maplestory-openapi)
[![Docs.rs](https://docs.rs/maplestory-openapi/badge.svg)](https://docs.rs/maplestory-openapi)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![CI](https://github.com/yourusername/maplestory-openapi-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/maplestory-openapi-rust/actions)

[NEXON MapleStory OpenAPI](https://openapi.nexon.com/ko/game/maplestory/)의 고성능·인체공학적(Ergonomic) Rust 클라이언트 라이브러리이자 **다국어 네이티브 SDK 코어(Python, Node.js, C#, Go, WASM, 모바일)**입니다.  
공식 OpenAPI 스펙을 100% 준수하면서도, 실제 메이플스토리 캐릭터 정보/스펙 분석 사이트·디스코드 봇·데이터 분석 도구를 개발할 때 필요한 **실무 편의 기능(코디/외형 자동 합성, 전투력 도메인 헬퍼, 인메모리 캐시, 올인원 종합 스펙 대시보드, 아바타 모션 GIF 생성)**을 강력하게 지원합니다.

> 🔑 **API Key 발급**: [NEXON Developers](https://openapi.nexon.com/)에서 손쉽게 발급받을 수 있습니다.  
> 📖 **빠른 링크**: [📚 공식 기술 문서 전체 보기](docs/README.md) | [🤖 디스코드 봇 레시피](docs/recipes-and-examples.md) | [❓ FAQ & 트러블슈팅](docs/error-handling-and-faq.md) | [🤝 기여 가이드](CONTRIBUTING.md)

---

## 💡 왜 이 라이브러리를 써야 할까요? (Why This Library?)

넥슨 공식 API를 직접 HTTP fetch로 호출할 때 겪게 되는 수많은 번거로움과 기술적 난관들을 완벽하게 해결했습니다:

| 구분 | 넥슨 OpenAPI 직접 호출 (Raw REST) | `maplestory-openapi` 사용 시 |
|---|---|---|
| **캐릭터 조회** | 닉네임 -> `GET /id` -> `ocid` 추출 -> 본 API 호출 (최소 2-Step) | `get_character_by_name("온앤온")` **단 1줄 호출** |
| **종합 스펙 대시보드** | 스펙, 장비, 유니온, 길드 등 20개 엔드포인트 수동 병합 (수백 줄 코드) | `get_character_dashboard_by_name()` **단 1줄 병렬 자동 취합** |
| **코디 / 외형** | 뷰티, 캐시장비, 일반장비 파편화 / 모루, 프리즘, 한벌옷 계산 불가 | `CharacterCoordination` **인게임 렌더링 규칙 100% 자동 합성** |
| **아바타 렌더러** | 정적 PNG 이미지 URL만 제공 | 걷기/윙크/점프 **무손실 투명 배경 애니메이션 GIF 자동 인코딩** |
| **API 호출 한도** | 동일 캐릭터 반복 조회 시 초당 한도(Rate Limit 429) 초과 빈번 | `moka` 기반 **인메모리 OCID 캐시로 트래픽 50% 절감 (0ms)** |
| **언어 지원** | 언어마다 직접 모델링 및 HTTP 클라이언트 구현 필요 | **Python, Node.js, C#, Go, WASM, iOS, Android 공식 SDK 제공** |

---

## 📑 목차 (Table of Contents)
- [🌟 핵심 특징 (Key Highlights)](#-핵심-특징-key-highlights)
- [🌐 다국어 지원 생태계 (Multi-Language Ecosystem)](#-다국어-지원-생태계-multi-language-ecosystem)
- [📦 Rust 크레이트 설치 (Rust Installation)](#-rust-크레이트-설치-rust-installation)
- [🚀 빠른 시작 (Quick Start)](#-빠른-시작-quick-start)
  - [1. 환경 변수로부터 클라이언트 생성](#1-환경-변수로부터-클라이언트-생성-from_env)
  - [2. 코디/외형(Coordination) 정보 조회](#2-코디외형coordination-정보-조회)
  - [3. 전투력 및 주요 스탯 헬퍼 활용](#3-전투력-및-주요-스탯-헬퍼-활용)
  - [4. 원스톱 종합 프로필 조회](#4-원스톱-종합-프로필-get_character_profile_by_name)
  - [5. 🏆 상용 정보 사이트 올인원 대시보드](#5--상용-정보-사이트-수준의-올인원-대시보드-get_character_dashboard_by_name)
- [🛠 에러 처리 및 트러블슈팅](#-정밀한-에러-처리-error-handling)
- [💡 넥슨 OpenAPI 실무 팁 & FAQ](#-넥슨-openapi-실무-주의사항-및-팁-gotchas)
- [📋 지원 API 전체 목록](#-지원-api-전체-목록)
- [🏃 예제 실행](#-예제-실행)
  - [🎬 아바타 애니메이션 GIF 생성](#-아바타-애니메이션-gif-생성-animation-기능)
- [💻 언어별 빠른 시작 (Python, Node.js, C#, Go, WASM, 모바일)](#-언어별-빠른-시작-multi-language-quick-starts)
- [🤝 기여 가이드 및 라이선스](#-기여-가이드-contributing)

---

## 🌟 핵심 특징 (Key Highlights)

- ⚡ **닉네임 원스톱 조회**: OCID를 먼저 수동으로 조회할 필요 없이, 인게임 닉네임으로 기본 정보·스탯·장비·코디 등을 단 1줄로 조회 (`get_character_by_name`, `get_character_stat_by_name` 등).
- 🏆 **올인원 종합 스펙 대시보드 (`get_character_dashboard_by_name`)**:
  - 메이플스카우터·츄츄지지·메애기·메짱 등 상용 캐릭터 정보 사이트에서 제공하는 **모든 정보(기본 스펙, 전투력, 스타포스 합, 심볼 포스, 유니온/아티팩트, 소속 길드/노블레스 스킬 자동 연계, 6차 헥사 코어/스탯, 5차 V매트릭스, 장비 세트효과, 펫, 안드로이드, 성향, 코디/아바타 렌더러)**를 단 1번의 닉네임 호출로 병렬 취합.
- 👗 **최종 코디/외형 합성 (Coordination Look)**:
  - 인게임 노출 규칙 완벽 지원: **캐시 장비가 있으면 캐시 아이템 우선 노출, 없는 부위는 일반 장비 노출**.
  - **신비의 모루** 외형 변경 자동 반영 (`is_anvil: true`, 모루 외형명 및 아이콘 자동 우선 노출).
  - **한벌옷** 착용 시 하의가 가려지는 인게임 레이어 규칙 반영.
  - **투명 장비/비노출 보조무기 감지** (`is_transparent`, `is_visually_rendered()`).
  - 컬러링 프리즘 염색 정보 및 마스터라벨/스페셜라벨 정보 제공.
- 🎨 **아바타 Look URL 파서 & 동적 렌더러 빌더 (`AvatarCode`)**:
  - `open.api.nexon.com` 및 `avatar.maplestory.nexon.com`의 1024자리 Base16 아바타 코드 상호 변환.
  - 표정(`emotion`), 포즈(`action`), 무기 모션(`wmotion`), 크기 커스텀 쿼리 빌더 (`AvatarLookQuery`).
  - 512바이트 원본 바이너리 블록 언패킹 지원 (`AvatarCode::to_bytes()`).
- ⚔️ **풍부한 도메인 헬퍼**:
  - `dash.combat_power()` / `stat.combat_power()` (전투력 정밀 파싱)
  - `dash.total_starforce()` (전체 장착 장비 스타포스 합계)
  - `dash.arcane_force()`, `dash.authentic_force()` (아케인/어센틱 심볼 포스 합계)
  - `dash.noblesse_skill_points()` (소속 길드 노블레스 스킬 포인트 총합)
  - `stat.stat_attack()` (최소/최대 스탯공격력 튜플)
  - `stat.boss_damage()`, `stat.ignore_monster_armor()` (보스 데미지, 방무 등)
  - `equip.weapon()`, `equip.hat()`, `equip.find_slot("무기")` (부위별 장비 즉시 검색)
- 🏰 **길드 & 유니온 모듈 완비**:
  - 길드 ID 및 기본 정보, 노블레스 스킬 포인트 계산 (`get_guild_by_name`, `get_guild_basic`).
  - 유니온 등급/레벨, 공격대 배치, 아티팩트 크리스탈/효과 (`get_user_union`, `get_user_union_raider`, `get_user_union_artifact`).
- 🪶 **가벼운 의존성 발자국**: 라이브러리 레벨의 불필요한 `tokio` 런타임 강제를 제거하여, 초경량 컴파일 및 런타임-애그노스틱 비동기 환경 지원.
- 💾 **인메모리 OCID 캐싱 (`cache` feature)**: `moka` 기반의 고성능 LRU+TTL 캐시 탑재. 거의 불변인 `닉네임 -> OCID` 매핑을 캐싱하여 API 호출 횟수를 절반으로 절약하고 레이트 리밋을 방어.
- 🔒 **타입 안전성**: 넥슨 공식 엔드포인트 전체에 대한 엄격한 모델링 및 상세한 에러 핸들링.
- 🌐 **다국어 네이티브 바인딩 완비**: Rust 코어 엔진을 기반으로 Python, Node.js/TypeScript, C# .NET, Go, WebAssembly, 모바일(iOS/Android)까지 100% 동일한 기능 지원.

---

## 🌐 다국어 지원 생태계 (Multi-Language Ecosystem)

Rust의 압도적인 성능과 메모리 안전성을 모든 언어 환경에서 누리실 수 있도록 공식 네이티브 바인딩을 제공합니다.

| 언어 / 플랫폼 | 기술 스택 | 경로 | 설명 |
|---|---|---|---|
| **Rust** | 네이티브 크레이트 | [`maplestory-openapi`](./maplestory-openapi) | 공식 고성능 코어 엔진 |
| **Python** | PyO3 + Maturin | [`bindings/python`](./bindings/python) | 데이터 분석, 머신러닝, Discord.py 봇 연동 |
| **Node.js / TS** | napi-rs | [`bindings/nodejs`](./bindings/nodejs) | 백엔드(NestJS/Express), Next.js, Discord.js 봇 |
| **WebAssembly** | wasm-bindgen | [`bindings/wasm`](./bindings/wasm) | 웹 브라우저 클라이언트, Cloudflare Workers |
| **C# / .NET** | P/Invoke (.NET 8+) | [`bindings/dotnet`](./bindings/dotnet) | Unity 게임 클라이언트, ASP.NET Core 백엔드 |
| **Go** | cgo | [`bindings/go`](./bindings/go) | 대규모 트래픽 마이크로서비스, Gin/Fiber API 서버 |
| **iOS (Swift)** | C-ABI Native FFI | [`bindings/mobile/ios`](./bindings/mobile) | Swift Package Manager / iOS 네이티브 앱 |
| **Android (Kotlin)** | JNI Native FFI | [`bindings/mobile/android`](./bindings/mobile) | Android NDK / Kotlin 네이티브 앱 |
| **C / C++** | C-ABI Dynamic Lib | [`bindings/c`](./bindings/c) | C 헤더(`maplestory_openapi.h`) 및 `.dll`/`.so` |

---

## 📦 Rust 크레이트 설치 (Rust Installation)

`Cargo.toml`에 추가:

```toml
[dependencies]
maplestory-openapi = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

### 기능 플래그 (Feature Flags)

| Feature | 설명 | 기본값 |
|---------|------|--------|
| `default-tls` | 시스템 네이티브 TLS 백엔드 사용 (`reqwest/default-tls`) | ✅ 켜짐 |
| `rustls-tls` | 순수 Rust 기반 `rustls` TLS 백엔드 사용 | ❌ 꺼짐 |
| `cache` | `moka` 기반 인메모리 OCID 자동 캐싱 활성화 | ❌ 꺼짐 |

캐싱을 활성화하려면:
```toml
[dependencies]
maplestory-openapi = { version = "0.1", features = ["cache"] }
```

---

## 🚀 빠른 시작 (Quick Start)

### 1. 환경 변수로부터 클라이언트 생성 (`from_env`)

`.env` 파일이나 시스템 환경변수에 `NEXON_API_KEY`를 설정해두면 코드 1줄로 초기화할 수 있습니다:

```rust
use maplestory_openapi::MapleStoryClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // NEXON_API_KEY 환경변수에서 자동으로 키 로드
    let client = MapleStoryClient::from_env()?;

    // 닉네임으로 기본 정보 조회
    let info = client.get_character_by_name("온앤온").await?;
    println!("Lv.{} {} ({})", 
        info.character_level.unwrap_or(0),
        info.character_name.as_deref().unwrap_or("?"),
        info.world_name.as_deref().unwrap_or("?")
    );

    Ok(())
}
```

---

### 2. 코디/외형(Coordination) 정보 조회

캐시 장비와 일반 장비, 신비의 모루 외형을 인게임 규칙에 맞춰 자동으로 합성해줍니다:

```rust
use maplestory_openapi::MapleStoryClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MapleStoryClient::from_env()?;

    // 닉네임으로 최종 노출 코디/외형 조회
    let look = client.get_character_coordination_by_name("온앤온").await?;

    // 1. 헤어 / 성형 / 피부
    if let Some(hair) = &look.hair {
        println!("헤어: {}", hair.hair_name.as_deref().unwrap_or("?"));
    }
    if let Some(face) = &look.face {
        println!("성형: {}", face.face_name.as_deref().unwrap_or("?"));
    }

    // 2. 최종 겉에 보이는 슬롯별 장비 목록
    for item in &look.visible_items {
        println!(
            "[{}] {} (캐시템 여부: {}, 모루 여부: {})",
            item.slot, item.name, item.is_cash, item.is_anvil
        );
    }

    // 3. 도메인 헬퍼 접근자
    if let Some(hat) = look.hat() {
        println!("모자: {}", hat.name);
    }
    if let Some(weapon) = look.weapon() {
        println!("무기: {}", weapon.name);
    }

    Ok(())
}
```

---

### 3. 전투력 및 주요 스탯 헬퍼 활용

```rust
use maplestory_openapi::MapleStoryClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MapleStoryClient::from_env()?;

    let stat = client.get_character_stat_by_name("온앤온").await?;

    // 전투력 직접 가져오기
    if let Some(cp) = stat.combat_power() {
        println!("전투력: {}", cp);
    }

    // 스탯공격력 (최소, 최대)
    let (min_att, max_att) = stat.stat_attack();
    println!("스공: {:?} ~ {:?}", min_att, max_att);

    // 보스 데미지 / 방무
    println!("보스 데미지: {:?}%", stat.boss_damage());
    println!("방어율 무시: {:?}%", stat.ignore_monster_armor());

    // 특정 스탯 이름으로 직접 조회
    if let Some(str_val) = stat.get_stat("STR") {
        println!("STR: {}", str_val);
    }

    Ok(())
}
```

---

### 4. 원스톱 종합 프로필 (`get_character_profile_by_name`)

기본 정보, 전투력, 장착 장비, 코디, 인기도, 무릉도장을 병렬로 한 번에 조회합니다:

```rust
use maplestory_openapi::MapleStoryClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MapleStoryClient::from_env()?;

    let profile = client.get_character_profile_by_name("온앤온").await?;

    println!("캐릭터: {} (Lv.{})", profile.basic.character_name.as_deref().unwrap_or("?"), profile.basic.character_level.unwrap_or(0));
    println!("전투력: {:?}", profile.stat.combat_power());
    println!("보이는 모자: {:?}", profile.coordination.hat().map(|h| &h.name));
    if let Some(dojang) = &profile.dojang {
        println!("무릉도장 최고 기록: {}층", dojang.dojang_best_floor.unwrap_or(0));
    }

    Ok(())
}
```

---

### 5. 🏆 상용 정보 사이트 수준의 올인원 대시보드 (`get_character_dashboard_by_name`)

메이플스카우터, 츄츄지지, 메애기, 메이플로드, 메짱 등 OpenAPI 기반 캐릭터 정보/스펙 분석 사이트에서 보여주는 **모든 필수 데이터(기본 스펙, 전투력, 스타포스 합, 심볼 포스, 유니온/아티팩트, 길드/노블레스 스킬, 어빌리티, 6차 헥사 코어/스탯, 5차 V매트릭스, 장비 세트효과, 펫, 안드로이드, 성향, 코디/아바타 렌더러)**를 단 한 번의 호출로 취합합니다:

```rust
use maplestory_openapi::{AvatarAction, AvatarEmotion, MapleStoryClient, WeaponMotion};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MapleStoryClient::from_env()?;

    // 닉네임 1줄 호출로 20개 이상의 엔드포인트 병렬 취합 + 길드 자동 연계 조회
    let dash = client.get_character_dashboard_by_name("온앤온").await?;

    println!("=== {} (Lv.{} {}) ===", dash.character_name(), dash.character_level(), dash.character_class());
    println!("전투력: {}", dash.combat_power());
    println!("스타포스: {}성 | 아케인포스: {} | 어센틱포스: {}", 
        dash.total_starforce(), dash.arcane_force(), dash.authentic_force());
    println!("유니온: Lv.{} ({:?}) | 아티팩트: Lv.{:?}", 
        dash.union_level().unwrap_or(0), dash.union_grade(), dash.union_artifact_level());
    
    // 길드 및 노블레스 스킬 (소속 길드 자동 연계 조회)
    if let Some(guild) = &dash.guild {
        println!("길드: {} (Lv.{}) | 마스터: {:?} | 노블스킬: {}p",
            guild.guild_name.as_deref().unwrap_or(""),
            guild.guild_level.unwrap_or(0),
            guild.guild_master_name,
            dash.noblesse_skill_points()
        );
    }

    // 어빌리티 & 무릉도장
    println!("어빌리티: {:?} | 무릉: {:?}층", dash.ability_grade(), dash.dojang_best_floor());

    // 코디 프리즘 아이템 & 커스텀 아바타 렌더러 URL
    println!("프리즘 아이템: {}개", dash.coordination.prism_items().count());
    if let Some(img_url) = dash.coordination.custom_image(
        AvatarAction::Jump,
        AvatarEmotion::Wink,
        WeaponMotion::Default,
        96, 96,
    ) {
        println!("커스텀 점프+윙크 아바타 URL: {}", img_url);
    }

    Ok(())
}
```

---

## 🛠 정밀한 에러 처리 (Error Handling)

`MapleStoryError`는 넥슨 API의 구체적인 에러 코드와 메시지를 포함하며, 편리한 판별 헬퍼를 제공합니다:

```rust
use maplestory_openapi::{MapleStoryClient, MapleStoryError};

#[tokio::main]
async fn main() {
    let client = MapleStoryClient::from_env().unwrap();

    match client.get_character_by_name("존재하지않는캐릭터12345").await {
        Ok(info) => println!("조회 성공: {:?}", info.character_name),
        Err(err) if err.is_not_found() => {
            println!("해당 캐릭터를 찾을 수 없습니다.");
        }
        Err(err) if err.is_rate_limited() => {
            println!("요청 한도 초과! {:?} 후에 재시도하세요.", err.retry_after());
        }
        Err(err) if err.is_auth_error() => {
            println!("API 키가 올바르지 않습니다: {:?}", err.api_message());
        }
        // 패턴 매칭으로 상세 정보 추출
        Err(MapleStoryError::ApiError { code, message, status }) => {
            println!("API 오류 [{} (HTTP {})]: {}", code, status, message);
        }
        Err(e) => println!("기타 오류: {}", e),
    }
}
```

---

## 💡 넥슨 OpenAPI 실무 주의사항 및 팁 (Gotchas)

1. **데이터 갱신 주기 (평균 15분)**:
   - 인게임 캐릭터 변경 사항(레벨업, 아이템 변경, 코디 변경 등)은 실시간이 아니며 **약 15분 후** API에 반영됩니다.
2. **조회 가능 기준일 (2023-12-21)**:
   - 넥슨 OpenAPI의 과거 데이터는 **2023-12-21** 이후의 데이터만 조회 가능합니다.
3. **전일 데이터 갱신 시각 (오전 02:00 KST)**:
   - 전일(어제) 데이터는 다음 날 오전 2시(KST) 이후부터 정상 확인이 가능합니다.
4. **OCID 변경 가능성 & 캐시 활용**:
   - 캐릭터 닉네임 변경이나 게임사 업데이트로 인해 드물게 OCID가 변경될 수 있습니다. 본 라이브러리의 `features = ["cache"]`는 기본 24시간 TTL을 적용하여 레이트 리밋을 보호하면서도 안전하게 최신성을 유지합니다.

---

## 📋 지원 API 전체 목록

| 분류 | 메서드 | 공식 엔드포인트 | 설명 |
|------|--------|----------------|------|
| **편의 기능** | `get_character_dashboard_by_name(name)` | — | **🏆 닉네임 한방 올인원 종합 스펙 대시보드 (길드·유니온·스펙·코디 병렬 취합)** |
| **편의 기능** | `get_character_by_name(name)` | — | 닉네임 한방 기본 정보 조회 |
| **편의 기능** | `get_character_stat_by_name(name)` | — | 닉네임 한방 종합 능력치 조회 |
| **편의 기능** | `get_character_item_equipment_by_name(name)` | — | 닉네임 한방 장착 장비 조회 |
| **편의 기능** | `get_character_coordination_by_name(name)` | — | **닉네임 한방 최종 코디/외형 합성 조회** |
| **편의 기능** | `get_character_profile_by_name(name)` | — | **닉네임 한방 전체 종합 프로필 병렬 조회** |
| **User** | `get_character_list()` | `GET /maplestory/v1/character/list` | 계정 캐릭터 목록 |
| **User** | `get_achievement(date)` | `GET /maplestory/v1/user/achievement` | 업적 정보 |
| **Union** | `get_user_union(ocid, date)` | `GET /maplestory/v1/user/union` | 유니온 레벨 및 등급, 아티팩트 레벨 |
| **Union** | `get_user_union_raider(ocid, date)` | `GET /maplestory/v1/user/union-raider` | 유니온 공격대원 배치 및 점령 스탯 |
| **Union** | `get_user_union_artifact(ocid, date)` | `GET /maplestory/v1/user/union-artifact` | 유니온 아티팩트 효과 및 크리스탈 |
| **Guild** | `get_guild_id(name, world)` | `GET /maplestory/v1/guild/id` | 길드 식별자(oguild_id) 조회 |
| **Guild** | `get_guild_basic(oguild_id, date)` | `GET /maplestory/v1/guild/basic` | 길드 기본 정보 및 노블레스 스킬 |
| **Guild** | `get_guild_by_name(name, world, date)`| — | 길드명+월드로 기본 정보 즉시 조회 |
| **Character**| `get_character_id(name)` | `GET /maplestory/v1/id` | OCID 조회 |
| **Character**| `get_character_basic(ocid, date)` | `GET /maplestory/v1/character/basic` | 기본 정보 |
| **Character**| `get_character_popularity(ocid, date)`| `GET /maplestory/v1/character/popularity` | 인기도 |
| **Character**| `get_character_stat(ocid, date)` | `GET /maplestory/v1/character/stat` | 종합 능력치 |
| **Character**| `get_character_hyper_stat(ocid, date)`| `GET /maplestory/v1/character/hyper-stat` | 하이퍼스탯 |
| **Character**| `get_character_propensity(ocid, date)`| `GET /maplestory/v1/character/propensity` | 성향 |
| **Character**| `get_character_ability(ocid, date)` | `GET /maplestory/v1/character/ability` | 어빌리티 |
| **Character**| `get_character_item_equipment(ocid, date)`| `GET /maplestory/v1/character/item-equipment` | 장착 장비 |
| **Character**| `get_character_cashitem_equipment(ocid, date)`| `GET /maplestory/v1/character/cashitem-equipment` | 캐시 장비 |
| **Character**| `get_character_symbol_equipment(ocid, date)`| `GET /maplestory/v1/character/symbol-equipment` | 심볼 |
| **Character**| `get_character_set_effect(ocid, date)`| `GET /maplestory/v1/character/set-effect` | 세트 효과 |
| **Character**| `get_character_beauty_equipment(ocid, date)`| `GET /maplestory/v1/character/beauty-equipment` | 헤어/성형/피부 |
| **Character**| `get_character_android_equipment(ocid, date)`| `GET /maplestory/v1/character/android-equipment` | 안드로이드 |
| **Character**| `get_character_pet_equipment(ocid, date)`| `GET /maplestory/v1/character/pet-equipment` | 펫 장비 |
| **Character**| `get_character_skill(ocid, grade, date)`| `GET /maplestory/v1/character/skill` | 스킬 정보 |
| **Character**| `get_character_link_skill(ocid, date)`| `GET /maplestory/v1/character/link-skill` | 링크 스킬 |
| **Character**| `get_character_vmatrix(ocid, date)` | `GET /maplestory/v1/character/vmatrix` | V매트릭스 |
| **Character**| `get_character_hexamatrix(ocid, date)`| `GET /maplestory/v1/character/hexamatrix` | HEXA 코어 |
| **Character**| `get_character_hexamatrix_stat(ocid, date)`| `GET /maplestory/v1/character/hexamatrix-stat` | HEXA 스탯 |
| **Character**| `get_character_dojang(ocid, date)` | `GET /maplestory/v1/character/dojang` | 무릉도장 기록 |
| **Character**| `get_character_other_stat(ocid, date)`| `GET /maplestory/v1/character/other-stat` | 기타 능력치 |
| **Character**| `get_character_ring_exchange_skill_equipment(ocid, date)`| `GET /maplestory/v1/character/ring-exchange-skill-equipment` | 링 익스체인지 |
| **Character**| `get_character_ring_reserve_skill_equipment(ocid, date)`| `GET /maplestory/v1/character/ring-reserve-skill-equipment` | 예비 특수 반지 |

---

## 🏃 예제 실행

`.env` 파일에 `NEXON_API_KEY=...`를 입력한 후 즉시 실행할 수 있습니다:

```sh
# 1. 🏆 올인원 종합 스펙 대시보드 (전투력, 스타포스, 심볼, 유니온, 길드, 노블스킬, 코디)
cargo run --example character_dashboard

# 2. 닉네임 한방 조회 (전투력, 스공, 대표 코디 포함)
cargo run --example quick_lookup -- 온앤온

# 3. 최종 코디/외형 정보 상세 조회 (캐시 우선 + 모루 외형 + 프리즘)
cargo run --example coordination_lookup -- 온앤온

# 4. 아바타 1024자리 Base16 디코딩 및 동적 렌더러
cargo run --example avatar_decoding_and_look -- 온앤온

# 5. 🎬 아바타 애니메이션 GIF 생성 (걷기, 윙크, 숨쉬기, 점프 등)
cargo run --example avatar_animation_gif --features animation -- 온앤온

# 6. 기본 사용법
cargo run --example basic_usage

# 7. 전체 정보 병렬 조회
cargo run --example character_full_info -- 온앤온
```

---

### 🎬 아바타 애니메이션 GIF 생성 (`animation` 기능)

`animation` 기능을 켜면(`features = ["animation"]`), 단 몇 줄의 코드로 메이플스토리 아바타의 모션(걷기, 윙크, 대기 숨쉬기, 점프, 비행 등)을 **무손실 투명 배경의 애니메이션 GIF 파일**로 바로 생성할 수 있습니다:

```rust
use maplestory_openapi::{AnimationPreset, MapleStoryClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MapleStoryClient::from_env()?;

    // 온앤온 캐릭터의 걷기(Walk) 애니메이션 GIF 생성 (128x128 해상도)
    let gif_bytes = client
        .render_character_gif_by_name("온앤온", &AnimationPreset::Walk, Some(128), Some(128))
        .await?;

    std::fs::write("walk.gif", gif_bytes)?;
    println!("걷기 애니메이션 GIF 생성 완료!");

    Ok(())
}
```

* 제공 프리셋: `AnimationPreset::Walk`, `AnimationPreset::Wink`, `AnimationPreset::Stand`, `AnimationPreset::Jump`, `AnimationPreset::Fly`, `AnimationPreset::Sit`, `AnimationPreset::Prone`, `AnimationPreset::Climb`, `AnimationPreset::Custom(Vec<AnimationFrame>)`

---

## 💻 언어별 빠른 시작 (Multi-Language Quick Starts)

<details>
<summary><b>🐍 Python</b> (자세히 보기)</summary>

```python
import asyncio
from maplestory_openapi import MapleStoryClient

async def main():
    client = MapleStoryClient.from_env()
    dash = await client.get_character_dashboard("온앤온")
    print(f"닉네임: {dash['basic']['character_name']}, 레벨: Lv.{dash['basic']['character_level']}")

    # 걷기 애니메이션 GIF 생성
    gif_bytes = await client.render_character_gif("온앤온", preset="walk", width=128, height=128)
    with open("walk.gif", "wb") as f:
        f.write(gif_bytes)

asyncio.run(main())
```
자세한 빌드 및 사용법: [`bindings/python/README.md`](./bindings/python)
</details>

<details>
<summary><b>🟩 Node.js / TypeScript</b> (자세히 보기)</summary>

```typescript
import { MapleStoryClient } from 'maplestory-openapi';
import * as fs from 'fs';

const client = MapleStoryClient.fromEnv();
const dash = await client.getCharacterDashboard("온앤온");
console.log(`닉네임: ${dash.basic.character_name}, 레벨: Lv.${dash.basic.character_level}`);

// 걷기 애니메이션 GIF 생성 (Buffer 반환)
const gifBuffer = await client.renderCharacterGif("온앤온", "walk", 128, 128);
fs.writeFileSync("walk.gif", gifBuffer);
```
자세한 빌드 및 사용법: [`bindings/nodejs/README.md`](./bindings/nodejs)
</details>

<details>
<summary><b>🔷 C# (.NET 8+)</b> (자세히 보기)</summary>

```csharp
using MapleStory.OpenApi;

using var client = MapleStoryClient.FromEnv();
string dashJson = await client.GetCharacterDashboardAsync("온앤온");
Console.WriteLine(dashJson);

// 걷기 애니메이션 GIF 생성
byte[] gifBytes = await client.RenderCharacterGifAsync("온앤온", "walk", 128, 128);
File.WriteAllBytes("walk.gif", gifBytes);
```
자세한 빌드 및 사용법: [`bindings/dotnet/README.md`](./bindings/dotnet)
</details>

<details>
<summary><b>🐹 Go</b> (자세히 보기)</summary>

```go
package main

import (
    "fmt"
    "os"
    "maplestory-openapi/bindings/go"
)

func main() {
    client, _ := maplestory.FromEnv()
    defer client.Close()

    dashJson, _ := client.GetCharacterDashboard("온앤온")
    fmt.Println(dashJson)

    gifBytes, _ := client.RenderCharacterGif("온앤온", "walk", 128, 128)
    os.WriteFile("walk.gif", gifBytes, 0644)
}
```
자세한 빌드 및 사용법: [`bindings/go/README.md`](./bindings/go)
</details>

<details>
<summary><b>🌐 WebAssembly / Browser</b> (자세히 보기)</summary>

```javascript
import init, { decodeAvatarLook, buildAvatarUrl } from './pkg/maplestory_openapi_wasm.js';

await init();
const parsed = decodeAvatarLook(avatarUrl);
const walkingUrl = buildAvatarUrl(parsed.code, "walk", 1, "smile", 1, "default");
```
자세한 빌드 및 사용법: [`bindings/wasm/README.md`](./bindings/wasm)
</details>

<details>
<summary><b>📱 Mobile (iOS Swift / Android Kotlin)</b> (자세히 보기)</summary>

- **iOS Swift**: C-ABI Native FFI를 활용한 고성능 클라이언트 [`bindings/mobile/ios/MapleStoryClient.swift`](./bindings/mobile/ios)
- **Android Kotlin**: JNI 네이티브 브릿지 클라이언트 [`bindings/mobile/android/MapleStoryClient.kt`](./bindings/mobile/android)
</details>

---

## 🤝 기여 가이드 (Contributing)

버그 리포트, 새로운 기능 제안, 문서 보강 등 모든 기여를 환영합니다!  
로컬 개발 환경 설정, 커밋 컨벤션, PR 제출 방법은 [CONTRIBUTING.md](CONTRIBUTING.md)를 참고해 주세요.

---

## 📜 라이선스 (License)

이 프로젝트는 [MIT License](LICENSE-MIT) 또는 [Apache License 2.0](LICENSE-APACHE) 듀얼 라이선스로 배포됩니다.
공식 문서 및 가이드는 [docs/README.md](docs/README.md)에서 확인하실 수 있습니다.


