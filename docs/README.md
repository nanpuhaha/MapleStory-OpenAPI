# MapleStory OpenAPI Rust Documentation

이 디렉토리는 **MapleStory OpenAPI Rust 클라이언트**의 아키텍처, 핵심 기능 설계서, 상세 가이드 및 확장 가이드를 담고 있습니다.

## 📚 문서 목차

1. [시스템 아키텍처 및 설계 원칙 (architecture.md)](architecture.md)
   - 런타임-애그노스틱 비동기 설계 (`reqwest`, `futures-util`)
   - 모듈화 구조 및 계층별 책임 분리
   - `moka` 기반 인메모리 OCID 캐싱 전략
   - 에러 핸들링 및 타입 안전성 모델링

2. [코디 합성 및 아바타 모션/애니메이션 엔진 (coordination-and-avatar.md)](coordination-and-avatar.md)
   - 메이플스토리 인게임 장비 렌더링 우선순위 (캐시 장비 vs 일반 장비 vs 신비의 모루)
   - 한벌옷 착용 시 하의 가림 처리 및 투명 장비/비노출 보조무기 감지
   - 1024자리 Base16 아바타 코드 디코딩 및 512바이트 바이너리 패킷 구조
   - 동적 룩(Look) 파라미터 규격 (`action`, `emotion`, `wmotion`)
   - 무손실 투명 배경 애니메이션 GIF 인코딩 파이프라인

3. [올인원 전적 대시보드 가이드 (dashboard-guide.md)](dashboard-guide.md)
   - 상용 전적 서비스(메이플스카우터, 츄츄지지, 메애기, 메이플로드 등) 벤치마킹 설계
   - `CharacterDashboard`의 20여 개 엔드포인트 병렬 취합 구조
   - 소속 길드 자동 연계 조회 및 노블레스 스킬 포인트 총합 계산
   - 유니온 등급/공격대원 배치/아티팩트 및 6차 전직 헥사 매트릭스 도메인 헬퍼

4. [다국어 바인딩 가이드 (multi-language-bindings.md)](multi-language-bindings.md)
   - Rust 코드를 Python, JavaScript(Node.js/WebAssembly), Go, C# 등 다른 언어 라이브러리로 변환하는 원리와 실무 도구 가이드
   - PyO3 / maturin을 이용한 파이썬 패키지(whl) 제작
   - napi-rs / wasm-bindgen을 이용한 자바스크립트 패키지(npm) 제작
   - UniFFI 및 C-ABI(cbindgen)를 이용한 크로스 플랫폼 SDK 빌드
