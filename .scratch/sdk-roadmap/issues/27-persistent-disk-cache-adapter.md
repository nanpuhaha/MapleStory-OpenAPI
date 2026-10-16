# 27: 영속 디스크/파일 기반 식별자 캐시 어댑터 (Persistent Disk Cache Adapter)

**What to build:**
기존 `IdentityCache` 트레이트를 확장/구현하여, 애플리케이션이나 프로세스가 재시작되어도 닉네임-OCID 매핑 및 길드명-길드ID 매핑을 로컬 파일 시스템에 영속 보관하여 불필요한 API 호출을 원천 차단하는 디스크 캐시 구현체를 제공합니다.

**Blocked by:** None (can start immediately)

**Status:** ready-for-agent

- [ ] 파일 기반 영속 캐시 구조체(`DiskIdentityCache`) 구현 및 `IdentityCache` 트레이트 구현
- [ ] 캐시 엔트리 만료(TTL) 및 직렬화/역직렬화(JSON 또는 바이너리 포맷) 안전성 보장
- [ ] 동시 프로세스/스레드 접근 시 파일 락 또는 안전한 아토믹 쓰기 보장
- [ ] 프로세스 재시작 시뮬레이션 및 캐시 히트/미스 동작 단위 테스트 작성 및 검증
