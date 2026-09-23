# MapleStory OpenAPI — Rust Client

[![Crates.io](https://img.shields.io/crates/v/maplestory-openapi.svg)](https://crates.io/crates/maplestory-openapi)
[![Docs.rs](https://docs.rs/maplestory-openapi/badge.svg)](https://docs.rs/maplestory-openapi)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

NEXON MapleStory OpenAPI의 Rust 클라이언트 라이브러리입니다.  
공식 Spec(`openapi.nexon.com`)을 Source of Truth로 구현했습니다.

> **API Key 발급**: [NEXON Developers](https://openapi.nexon.com/)

---

## 특징

- ✅ **25개 API 엔드포인트** 전체 지원 (User 2개 + Character 23개)
- ⚡ **닉네임 한방 조회** — `get_character_by_name()` 한 줄로 끝
- 🚀 **완전 비동기** (tokio 기반)
- 🔒 **타입 안전** — 공식 YAML spec 기반 Rust 모델
- 🛠 **에러 처리** — 상세한 에러 타입과 메시지

---

## 설치

`Cargo.toml`에 추가:

```toml
[dependencies]
maplestory-openapi = "0.1"
tokio = { version = "1", features = ["full"] }
```

---

## 빠른 시작

### 닉네임 한방 조회 (추천)

```rust
use maplestory_openapi::MapleStoryClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MapleStoryClient::new("YOUR_API_KEY")?;

    // OCID 조회 → 기본 정보 조회를 한 번에
    let info = client.get_character_by_name("캐릭터명").await?;

    println!("이름: {}", info.character_name.as_deref().unwrap_or("?"));
    println!("레벨: {}", info.character_level.unwrap_or(0));
    println!("직업: {}", info.character_class.as_deref().unwrap_or("?"));
    println!("월드: {}", info.world_name.as_deref().unwrap_or("?"));

    Ok(())
}
```

### 단계별 조회

```rust
use maplestory_openapi::{MapleStoryClient, SkillGrade};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MapleStoryClient::new("YOUR_API_KEY")?;

    // 1단계: OCID 조회
    let ocid = client.get_ocid("캐릭터명").await?;

    // 2단계: 여러 정보 병렬 조회
    let (basic, stat, dojang) = tokio::join!(
        client.get_character_basic(&ocid, None),
        client.get_character_stat(&ocid, None),
        client.get_character_dojang(&ocid, None),
    );

    let basic = basic?;
    println!("Lv.{} {}", basic.character_level.unwrap_or(0),
             basic.character_class.as_deref().unwrap_or("?"));

    Ok(())
}
```

---

## 지원 API 목록

### User

| 메서드 | 엔드포인트 | 설명 |
|--------|-----------|------|
| `get_character_list()` | `GET /maplestory/v1/character/list` | 계정 캐릭터 목록 |
| `get_achievement(date)` | `GET /maplestory/v1/user/achievement` | 업적 정보 |

### Character

| 메서드 | 엔드포인트 | 설명 |
|--------|-----------|------|
| `get_character_id(name)` | `GET /maplestory/v1/id` | OCID 조회 |
| `get_ocid(name)` | `GET /maplestory/v1/id` | OCID 문자열 조회 |
| `get_character_by_name(name)` | — | **닉네임 한방 조회** (OCID+basic) |
| `get_character_basic(ocid, date)` | `GET /maplestory/v1/character/basic` | 기본 정보 |
| `get_character_popularity(ocid, date)` | `GET /maplestory/v1/character/popularity` | 인기도 |
| `get_character_stat(ocid, date)` | `GET /maplestory/v1/character/stat` | 종합 능력치 |
| `get_character_hyper_stat(ocid, date)` | `GET /maplestory/v1/character/hyper-stat` | 하이퍼스탯 |
| `get_character_propensity(ocid, date)` | `GET /maplestory/v1/character/propensity` | 성향 |
| `get_character_ability(ocid, date)` | `GET /maplestory/v1/character/ability` | 어빌리티 |
| `get_character_item_equipment(ocid, date)` | `GET /maplestory/v1/character/item-equipment` | 장착 장비 |
| `get_character_cashitem_equipment(ocid, date)` | `GET /maplestory/v1/character/cashitem-equipment` | 캐시 장비 |
| `get_character_symbol_equipment(ocid, date)` | `GET /maplestory/v1/character/symbol-equipment` | 심볼 |
| `get_character_set_effect(ocid, date)` | `GET /maplestory/v1/character/set-effect` | 세트 효과 |
| `get_character_beauty_equipment(ocid, date)` | `GET /maplestory/v1/character/beauty-equipment` | 헤어/성형/피부 |
| `get_character_android_equipment(ocid, date)` | `GET /maplestory/v1/character/android-equipment` | 안드로이드 |
| `get_character_pet_equipment(ocid, date)` | `GET /maplestory/v1/character/pet-equipment` | 펫 장비 |
| `get_character_skill(ocid, grade, date)` | `GET /maplestory/v1/character/skill` | 스킬 정보 |
| `get_character_link_skill(ocid, date)` | `GET /maplestory/v1/character/link-skill` | 링크 스킬 |
| `get_character_vmatrix(ocid, date)` | `GET /maplestory/v1/character/vmatrix` | V매트릭스 |
| `get_character_hexamatrix(ocid, date)` | `GET /maplestory/v1/character/hexamatrix` | HEXA 코어 |
| `get_character_hexamatrix_stat(ocid, date)` | `GET /maplestory/v1/character/hexamatrix-stat` | HEXA 스탯 |
| `get_character_dojang(ocid, date)` | `GET /maplestory/v1/character/dojang` | 무릉도장 기록 |
| `get_character_other_stat(ocid, date)` | `GET /maplestory/v1/character/other-stat` | 기타 능력치 |
| `get_character_ring_exchange_skill_equipment(ocid, date)` | `GET /maplestory/v1/character/ring-exchange-skill-equipment` | 링 익스체인지 |
| `get_character_ring_reserve_skill_equipment(ocid, date)` | `GET /maplestory/v1/character/ring-reserve-skill-equipment` | 예비 특수 반지 |

---

## 날짜 파라미터

- 모든 날짜는 `"YYYY-MM-DD"` 형식 (KST 기준)
- `None`을 전달하면 가장 최신 데이터 조회
- 최소 조회 가능 날짜: **2023-12-21**
- 전날 데이터는 당일 **오전 2시 (KST)** 이후 조회 가능

```rust
// 특정 날짜 조회
let basic = client.get_character_basic(&ocid, Some("2024-01-15")).await?;

// 최신 데이터 조회
let basic = client.get_character_basic(&ocid, None).await?;
```

---

## 스킬 등급 (SkillGrade)

`get_character_skill` 사용 시 `SkillGrade` 지정 필요:

```rust
use maplestory_openapi::SkillGrade;

// 4차 스킬
client.get_character_skill(&ocid, SkillGrade::Fourth, None).await?;

// 5차 스킬
client.get_character_skill(&ocid, SkillGrade::Fifth, None).await?;

// 하이퍼 패시브
client.get_character_skill(&ocid, SkillGrade::HyperPassive, None).await?;
```

사용 가능한 값: `Zero`, `First`, `FirstHalf`, `Second`, `SecondHalf`, `Third`, `Fourth`, `HyperPassive`, `HyperActive`, `Fifth`, `Sixth`

---

## 에러 처리

```rust
use maplestory_openapi::{MapleStoryClient, MapleStoryError};

match client.get_character_by_name("없는캐릭터").await {
    Ok(info) => println!("{:?}", info),
    Err(MapleStoryError::CharacterNotFound { name }) => {
        println!("캐릭터 '{}' 를 찾을 수 없습니다", name);
    }
    Err(MapleStoryError::AuthenticationFailed) => {
        println!("API 키가 유효하지 않습니다");
    }
    Err(MapleStoryError::RateLimited { retry_after }) => {
        println!("요청 한도 초과. {}초 후 재시도하세요", retry_after);
    }
    Err(e) => println!("오류: {}", e),
}
```

---

## 예제 실행

```sh
# 환경 변수 설정
export NEXON_API_KEY=your_api_key_here

# 닉네임 한방 조회
cargo run --example quick_lookup -- 캐릭터명

# 기본 사용법
cargo run --example basic_usage

# 전체 정보 병렬 조회
cargo run --example character_full_info -- 캐릭터명
```

---

## 라이선스

MIT — 자세한 내용은 [LICENSE](LICENSE) 파일을 참조하세요.

---

## 참고

- **공식 OpenAPI 문서**: https://openapi.nexon.com/ko/game/maplestory/
- **Source of Truth**: `14_ko_script20260917040003.yaml`
- **참고 구현체**: [SpiralMoon/maplestory.openapi](https://github.com/SpiralMoon/maplestory.openapi)
