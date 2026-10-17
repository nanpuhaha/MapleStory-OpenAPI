# 30: 메이플 OpenAPI 독립형 터미널 CLI 도구 (`maple-cli`)

**What to build:**
터미널에서 즉시 메이플 정보를 조회할 수 있는 Rust 네이티브 CLI 바이너리 크레이트로, 캐릭터 대시보드 출력(`maple char <name>`), 아바타 이미지 저장(`maple avatar <name>`), 두 캐릭터 스펙 비교(`maple compare <name1> <name2>`), 스타포스 기댓값 계산(`maple calc starforce`)을 명령줄 인터페이스로 제공합니다.

**Blocked by:** 24: 장비 교체 및 스펙 비교 시뮬레이터 (Gear Swap & Stat Diff Engine), 27: 영속 디스크/파일 기반 식별자 캐시 어댑터 (Persistent Disk Cache Adapter)

**Status:** done

- [x] 워크스페이스 내 독립 바이너리 크레이트 `maple-cli` 추가 (clap 기반 CLI 인터페이스)
- [x] 환경변수(`NEXON_API_KEY`) 및 로컬 디스크 캐시 자동 연동 구성
- [x] 서브커맨드 구현:
  - `maple char <name>`: 컬러 ANSI 터미널 대시보드 출력
  - `maple avatar <name> [--action <action>] [--out <file>]`: 아바타 이미지/GIF/WebP 렌더링 및 저장
  - `maple compare <name1> <name2>`: 두 캐릭터 스펙 및 장비 차이 시각적 비교
  - `maple calc starforce --level <lvl> --from <from> --to <to>`: 스타포스 기댓값 및 비용 분석
- [x] CLI 인수 파싱 및 기본 실행 플로우 통합 테스트 작성
