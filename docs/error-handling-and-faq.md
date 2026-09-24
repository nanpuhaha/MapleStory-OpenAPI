# 에러 핸들링 및 FAQ / 트러블슈팅 가이드

이 문서는 `maplestory-openapi` 라이브러리를 사용할 때 발생할 수 있는 에러 유형, 넥슨 OpenAPI의 정책 및 제약사항, 그리고 자주 묻는 질문(FAQ)과 해결 방법을 안내합니다.

---

## 1. 에러 유형 및 처리 전략

`maplestory-openapi`는 모든 네트워크 및 API 오류를 `MapleStoryError` 열거형으로 엄격하게 분류하여 제공합니다.

### 1) 주요 에러 열거형 (`MapleStoryError`)

| 에러 변형 (Variant) | 원인 | 해결 방안 |
|---|---|---|
| `MapleStoryError::InvalidApiKey` | API 키가 비어있거나 올바르지 않은 경우 | 발급받은 API 키가 정확한지 확인하고 공백이나 개행 문자를 점검합니다. |
| `MapleStoryError::CharacterNotFound(name)` | 해당 닉네임의 캐릭터가 존재하지 않거나 삭제/탈퇴된 경우 | 닉네임 오탈자를 점검하거나 최근 닉네임 변경 이력을 확인합니다. |
| `MapleStoryError::Api { error_name, message, status }` | 넥슨 서버에서 표준 에러 응답을 반환한 경우 | 아래의 HTTP 상태 코드별 대처표를 참고합니다. |
| `MapleStoryError::RateLimitExceeded` | 넥슨 API 호출 제한(초당/분당)을 초과한 경우 (HTTP 429) | 지연 재시도(Exponential Backoff)를 수행하거나 `cache` 기능을 활성화합니다. |
| `MapleStoryError::Http(err)` | 네트워크 단절, DNS 해석 실패, 타임아웃 등 | 네트워크 연결 상태를 확인하고 재시도합니다. |
| `MapleStoryError::Serialization(err)` | 넥슨 응답 JSON 구조가 변경되었거나 파싱 실패 시 | 라이브러리 최신 버전으로 업데이트하거나 이슈를 제보해 주세요. |

---

## 2. 넥슨 OpenAPI HTTP 상태 코드 및 대응

넥슨 OpenAPI는 실패 시 다음과 같은 JSON 에러 본문을 반환합니다:
```json
{
  "error": {
    "name": "OPENAPI00003",
    "message": "유효하지 않은 식별자입니다."
  }
}
```

### 1) 자주 발생하는 오류 코드

#### 🔴 HTTP 400 Bad Request (`OPENAPI00003`, `OPENAPI00004`)
- **원인**: 전달된 `ocid`나 파라미터가 유효하지 않거나, 과거 데이터 조회 날짜가 너무 오래된 경우.
- **규칙**: 메이플스토리 OpenAPI는 **2023년 12월 21일 데이터부터 조회 가능**합니다. 그 이전 날짜를 요청하면 400 에러가 반환됩니다.
- **대응**: 날짜 파라미터가 `YYYY-MM-DD` 형식인지, 2023-12-21 이후인지 확인합니다.

#### 🔴 HTTP 401 Unauthorized / HTTP 403 Forbidden (`OPENAPI00002`, `OPENAPI00007`)
- **원인**: API 키가 잘못되었거나, 서비스가 비활성화된 경우.
- **팁**: 넥슨 개발자 센터에는 **'테스트용 API 키(개발용)'**와 **'서비스용 API 키(운영용)'**가 있습니다.
  - 테스트용 API 키: 발급자의 넥슨 계정에 연결된 캐릭터만 조회 가능할 수 있습니다.
  - 종합 검색 사이트나 봇을 운영하려면 반드시 **'서비스 신청'**을 통해 정식 서비스 API 키를 발급받으셔야 타인의 캐릭터도 자유롭게 조회할 수 있습니다.

#### 🔴 HTTP 429 Too Many Requests (`OPENAPI00006`)
- **원인**: 초당 또는 일일 호출 제한(Rate Limit)을 초과했습니다.
- **대응**:
  1. 라이브러리의 `cache` 기능을 켜서 `GET /id` 호출을 메모리에 캐싱합니다 (캐시 히트 시 네트워크 0ms).
  2. 대규모 배치 작업 시 요청 간 최소 50~100ms의 `sleep`을 부여합니다.

#### 🔴 HTTP 500 Internal Server Error (`OPENAPI00001`)
- **원인**: 넥슨 서버 점검 중이거나 일시적인 장애입니다.
- **대응**: 정기 점검 시간(매주 목요일 오전)인지 확인하고 잠시 후 재시도합니다.

---

## 3. 실무 모범 사례 (Best Practices)

### 1) `cache` 기능 활성화로 레이트 리밋 방어
캐릭터의 닉네임과 고유 식별자(`ocid`)는 닉네임을 변경하지 않는 한 변하지 않습니다.
`Cargo.toml`에서 `cache` feature를 켜면, 인메모리 고성능 LRU+TTL 캐시(`moka`)가 활성화되어 동일 닉네임에 대한 `get_character_id` 호출을 0ms로 즉시 반환합니다.

```toml
[dependencies]
maplestory-openapi = { version = "0.1", features = ["cache"] }
```

### 2) 날짜(`date`) 파라미터 규칙 이해하기
- `date`를 생략하거나 `None`으로 전달하면 **어제(KST 기준 전일 00:00:00)의 데이터**가 기본값으로 조회됩니다.
- 당일 실시간 갱신 데이터(예: 방금 올린 레벨이나 착용한 장비)는 넥슨 API 정책상 **매일 오전 01:00 KST를 기점으로 전일 데이터가 최종 집계**되므로, 실시간 반영까지 약간의 시차가 존재할 수 있습니다.

---

## 4. 자주 묻는 질문 (FAQ)

### Q1. "투명 장갑"이나 "투명 망토"를 꼈는데 왜 코디에 안 보이나요?
넥슨 캐시 장비 엔드포인트는 투명 장비도 하나의 캐시 아이템으로 반환합니다.
본 라이브러리의 `get_character_coordination_by_name()`은 투명 아이템을 자동 감지하여 `is_transparent: true` 플래그를 제공하므로, UI에서 빈 슬롯으로 표시할지 또는 투명 아이템 아이콘을 띄울지 손쉽게 제어할 수 있습니다.

### Q2. 신비의 모루가 적용된 아이템은 어떻게 처리되나요?
일반 장비의 `item_shape_name`과 `item_shape_icon`을 자동 감지합니다.
`coordination.visible_items`에는 실제 화면에 보이는 **모루 외형**이 우선 노출되며, `is_anvil: true` 플래그와 함께 `original_name` 필드에 실제 장비 이름이 함께 보존됩니다.

### Q3. 한벌옷을 입으면 하의가 왜 목록에 안 나오나요?
메이플스토리 인게임 규칙상 상의 슬롯에 한벌옷(Overall)을 입으면 하의가 완전히 가려집니다.
`CharacterCoordination`은 이러한 게임 엔진 레이어 규칙을 완벽하게 재현하여, 하의 장비를 최종 외형 목록에서 자동으로 제외합니다.

### Q4. 컬러링 프리즘 염색 정보는 어떻게 확인하나요?
`item.coloring_prism` 필드에 색상환(Hue), 채도(Saturation), 명도(Value) 변경 수치가 포함되며, `item.icon`에는 넥슨 CDN에서 실시간 렌더링된 공식 프리즘 적용 아이콘 URL이 자동으로 연결됩니다.

### Q5. 아바타 1024자리 Base16 코드는 어디서 얻나요?
1. `client.get_character_dashboard_by_name("온앤온")` 호출 후 `dash.avatar_code()`로 바로 얻을 수 있습니다.
2. 넥슨 캐시 장비나 아바타 이미지 URL(`https://open.api.nexon.com/.../look/ABC...`)을 `AvatarCode::from_url_or_code()`에 전달하면 손쉽게 인스턴스를 생성할 수 있습니다.

### Q6. 애니메이션 GIF 생성 시 배경을 투명하게 유지할 수 있나요?
네! 라이브러리의 `animation` 기능(`encode_animated_gif`)은 RGBA 알파 채널을 완벽하게 보존하므로, 웹사이트나 디스코드 어두운 테마 어디에 올려도 캐릭터 외곽선이 깔끔한 무손실 투명 배경으로 렌더링됩니다.

### Q7. 타 언어(Python, Node.js, C#, Go)에서도 동일한 기능을 쓸 수 있나요?
네! `bindings/` 디렉토리에 공식 네이티브 바인딩이 구현되어 있어, 동일한 Rust 코어 엔진의 속도와 기능을 각 언어의 고유한 문법(`async/await`, `Promise`, `Task`)으로 그대로 사용할 수 있습니다.
