# 28: 길드원/계정 캐릭터 대량 병렬 수집기 (Batch Character & Guild Roster Crawler)

**What to build:**
길드원 전체 목록이나 계정 내 다수 캐릭터 목록을 전달받아, 내장된 `ConcurrencyLimiter` 속도에 맞춰 동시 요청을 제어하며 전체 캐릭터의 프로필(`CharacterProfile`) 및 전투력/레벨 요약을 안전하게 일괄 수집하는 병렬 배치 API를 제공합니다.

**Blocked by:** 27: 영속 디스크/파일 기반 식별자 캐시 어댑터 (Persistent Disk Cache Adapter)

**Status:** done

- [x] `GuildClient` 및 `CharacterClient`에 배치 수집 메서드 구현 (`fetch_roster_profiles`, `fetch_character_batch`)
- [x] `futures::stream::buffer_unordered` 또는 청크 기반 병렬 처리 및 Rate Limiter와의 원활한 통합
- [x] 개별 캐릭터 조회 실패(탈퇴, 비공개, 오류 등) 시 부분 성공(`BatchResult<T>`) 처리 및 상세 에러 집계
- [x] 모의 클라이언트를 활용한 동시성 제어 및 대량 수집 시나리오 단위 테스트 작성
