# MapleStory OpenAPI Rust 공식 기술 문서 (Documentation)

`MapleStory-OpenAPI-Rust` 라이브러리의 전체 아키텍처, 도메인 엔진 설계서, 다국어 바인딩 원리, 실전 활용 레시피, 그리고 에러 핸들링 및 FAQ를 안내합니다.

---

## 📚 문서 전체 목차

| 문서 | 설명 | 주요 내용 |
|---|---|---|
| 📐 [**시스템 아키텍처 (architecture.md)**](architecture.md) | 라이브러리 설계 원칙 | • 런타임-애그노스틱 비동기 설계 (`reqwest`, `futures-util`)<br>• 계층별 책임 분리 (Low-level vs High-level)<br>• `moka` 인메모리 OCID 캐싱 아키텍처<br>• 타입 안전성 및 유연한 폴백 모델링 |
| 👗 [**코디 합성 & 아바타 모션 엔진 (coordination-and-avatar.md)**](coordination-and-avatar.md) | 코디/외형 렌더링 규격 | • 캐시 장비 vs 일반 장비 vs 신비의 모루 노출 우선순위<br>• 한벌옷 하의 가림 및 투명 장비 감지 규칙<br>• 1024자리 Base16 아바타 코드 512B 바이너리 구조<br>• 동적 모션 파라미터(`action`, `emotion`) 규격<br>• 무손실 투명 배경 애니메이션 GIF 인코딩 파이프라인 |
| 🏆 [**올인원 종합 스펙 대시보드 가이드 (dashboard-guide.md)**](dashboard-guide.md) | 상용 정보 사이트급 대시보드 | • 메이플스카우터·츄츄지지·메애기 등 벤치마킹 설계<br>• 20여 개 엔드포인트 병렬 취합 (`CharacterDashboard`)<br>• 소속 길드 자동 연계 및 노블레스 스킬 총합 계산<br>• 6차 헥사 매트릭스 / 유니온 아티팩트 도메인 헬퍼 |
| 🌐 [**다국어 네이티브 바인딩 아키텍처 (multi-language-bindings.md)**](multi-language-bindings.md) | 크로스 플랫폼 지원 | • Rust 코어 엔진을 기반으로 한 다국어 바인딩 원리<br>• Python (`PyO3`), Node.js (`napi-rs`), WebAssembly (`wasm-bindgen`)<br>• C# .NET (P/Invoke), Go (`cgo`), 모바일 (iOS Swift / Android Kotlin) |
| 🤖 [**실전 활용 레시피 (recipes-and-examples.md)**](recipes-and-examples.md) | 실무 완성형 코드 스니펫 | • 디스코드 봇 레시피 (Python `discord.py`, TS `discord.js`)<br>• 초고속 캐릭터 정보 검색 REST API 서버 (Rust `Axum`)<br>• 아바타 걷기/윙크 투명 GIF 생성기 |
| ❓ [**에러 핸들링 & FAQ / 트러블슈팅 (error-handling-and-faq.md)**](error-handling-and-faq.md) | 장애 대응 및 자주 묻는 질문 | • 넥슨 HTTP 상태 코드별 대처표 (400, 401, 403, 429, 500)<br>• API 키 권한(테스트용 vs 서비스용) 및 점검 시간 안내<br>• 레이트 리밋 방어 전략 (`cache` feature)<br>• 코디/아바타 관련 자주 묻는 질문 7선 |

---

## 🚀 빠른 시작 링크
- 코어 Rust 사용법: [프로젝트 루트 README.md](../README.md)
- 오픈소스 기여 가이드: [CONTRIBUTING.md](../CONTRIBUTING.md)
