# Domain Glossary (GLOSSARY.md)

This document defines the canonical domain vocabulary for `maplestory-openapi`. All modules, interfaces, and architecture documentation use these terms consistently.

---

## 1. Character Spec & Identity (캐릭터 식별 및 스펙)

### OCID (Open Character ID, 오픈 캐릭터 식별자)
- **Definition**: NEXON MapleStory OpenAPI에서 캐릭터를 고유하게 식별하는 불변 키.
- **Rules**: 대부분의 상세 조회 API는 닉네임 대신 OCID를 요구하므로, `GET /id` 엔드포인트를 통해 닉네임을 OCID로 변환한 후 사용합니다. 닉네임 변경 전까지 불변이므로 인메모리 캐시(`IdentityCache`)의 주요 대상입니다.

### OGuildID (Open Guild ID, 오픈 길드 식별자)
- **Definition**: NEXON MapleStory OpenAPI에서 길드를 고유하게 식별하는 키.
- **Rules**: 길드 이름과 월드 이름의 조합으로 유일하게 식별되며, `GET /guild/id`를 통해 조회됩니다.

### CharacterDashboard (캐릭터 종합 대시보드)
- **Definition**: 캐릭터의 기본 정보, 스탯, 일반/캐시 장비, 심볼, 어빌리티, 5차/6차 매트릭스, 유니온, 길드 등 최대 20개 이상의 엔드포인트를 통합 집계한 최상위 도메인 모델.
- **Rules**: 복잡한 병렬 호출을 단일 인터페이스 뒤로 캡슐화하며, 부분 실패가 발생하더라도 필수 스펙을 유지하는 복원력을 가집니다.

### DashboardSection (대시보드 집계 단위)
- **Definition**: `CharacterDashboard`를 구성하는 독립된 하위 데이터 도메인(스탯, 장비, 심볼, 어빌리티, V매트릭스, HEXA매트릭스, 유니온, 펫, 안드로이드, 길드 등).
- **Rules**: 각 섹션은 독립적으로 선택(마스킹)되거나 실패할 수 있으며, 호출자는 불필요한 섹션 조회를 건너뛸 수 있습니다.

### DashboardOptions (대시보드 조회 제어 모듈)
- **Definition**: `CharacterDashboard` 조회 시 포함하거나 제외할 섹션, 동시성 정책 등을 지정하는 옵션 빌더.
- **Rules**: 기본값(`DashboardOptions::all()`)은 전체 섹션을 조회하며, 호출자가 필요한 섹션만 지정하여 API 호출량과 응답 지연을 최적화할 수 있습니다.

### Partial Degradation (부분 실패 복원)
- **Definition**: 대시보드 집계 중 비필수 엔드포인트(예: 치장 장비 일시 장애, 6차 전직 미해금으로 인한 404, 일시적 429 등)에서 오류가 발생하더라도, 전체 집계를 파기하지 않고 성공한 섹션을 보존하면서 진단 정보를 `errors` 맵에 격리하는 복원 전략.

---

## 2. Appearance & Coordination (코디 및 외형)

### CharacterCoordination (캐릭터 코디/외형 합성)
- **Definition**: 일반 장비, 캐시 장비, 뷰티(헤어/성형/피부), 신비의 모루 외형을 게임 내 우선순위에 따라 하나의 최종 가시 외형(Visible Equipment)으로 합성한 도메인 모델.
- **Rules**: 캐시 장비가 일반 장비를 덮어쓰며, 한벌옷 착용 시 하의가 가려지고, 모루가 적용된 일반 장비는 모루 외형을 표기합니다.

### AvatarCode (아바타 코드)
- **Definition**: 캐릭터 렌더 CDN URL에 포함된 1024자 Hex(Base16: A-P) 인코딩 문자열.
- **Rules**: 캐릭터의 외형 부품과 모션, 각도, 표정 등을 조합하여 애니메이션 프레임 또는 GIF를 생성하는 데 사용됩니다.

---

## 3. Combat & Guild (전투 및 연계)

### CombatPower (전투력)
- **Definition**: 캐릭터의 종합 전투 지표 수치(`final_stat` 항목의 `전투력`).

### NoblesseSkillPoints (노블레스 스킬 포인트)
- **Definition**: 길드 스킬 중 보스 몬스터 공격 데미지, 방어율 무시, 크리티컬 데미지, 데미지 증가 등 노블레스 길드 스킬 레벨의 총합.
