# 24: 장비 교체 및 스펙 비교 시뮬레이터 (Gear Swap & Stat Diff Engine)

**What to build:**
기존 `CharacterItemEquipment`와 `CharacterStat`을 바탕으로, 특정 슬롯의 장비 아이템을 다른 아이템으로 교체하거나 잠재옵션/스타포스를 변경했을 때의 세트 효과 변화, 주스탯 변동치 및 예상 전투력 증감(`StatDiff`, `CombatPowerDiff`)을 네트워크 호출 없이 순수 함수로 역산/시뮬레이션하는 도메인 엔진을 제공합니다.

**Blocked by:** None (can start immediately)

**Status:** done

- [x] `EquipmentDiff` 및 `StatDiff` 도메인 모델 정의 (슬롯별 아이템 비교, 스탯 증감치, 공격력/마력 증감치, 보스데미지/방무 증감치)
- [x] 장비 교체 시 세트 효과(`ItemSetEffect`) 재계산 및 세트 옵션 반영 로직 구현
- [x] 잠재옵션 및 스타포스 수치 변경에 따른 최종 스탯 및 전투력 변동 역산 헬퍼 구현 (`CharacterCombatAnalytics::simulate_equipment_swap`)
- [x] 네트워크 의존성 없는 순수 단위 테스트(가상 아이템 교체, 세트 효과 깨짐/추가, 전투력 증감 검증) 작성 및 통과
