# 26: 6차 전직 헥사 매트릭스 강화 잔여 비용 계산기 (HEXA Matrix Progression Calculator)

**What to build:**
`CharacterHexaMatrix` 및 `CharacterHexaMatrixStat` 데이터를 바탕으로 현재 활성화된 스킬/스탯 코어의 레벨에서 목표 레벨(최대 30레벨)까지 도달하는 데 필요한 솔 에르다 및 솔 에르다 조각 잔여 수량을 직업군/코어 타입별로 즉시 계산해주는 도메인 헬퍼를 제공합니다.

**Blocked by:** None (can start immediately)

**Status:** ready-for-agent

- [ ] 헥사 매트릭스 코어 타입별(스킬 코어, 마스터리 코어, 강화 코어, 공용 코어, 헥사스탯) 1~30레벨 소요 재료 테이블 정의
- [ ] `CharacterHexaMatrix` 모델에 `remaining_materials(target_level)` 헬퍼 구현 (코어별 및 전체 합산 솔 에르다 / 솔 에르다 조각 잔여량)
- [ ] `CharacterHexaMatrixStat` 모델에 헥사스탯 레벨업 소요 조각 계산 헬퍼 구현
- [ ] 다양한 코어 레벨 조합에 대한 단위 테스트 및 정확성 검증 작성
