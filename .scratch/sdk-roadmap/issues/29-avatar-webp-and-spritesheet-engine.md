# 29: 아바타 WebP 애니메이션 및 스프라이트 시트 패커 (WebP & Spritesheet Avatar Engine)

**What to build:**
기존 GIF 렌더러에 더해, 더 작은 용량과 알파 투명도를 완벽 지원하는 WebP 애니메이션 인코딩과 웹/게임 엔진에서 즉시 활용할 수 있도록 아바타 프레임을 그리드 형태의 PNG 스프라이트 시트 및 프레임 아틀라스 JSON으로 패킹하는 미디어 익스포터를 제공합니다.

**Blocked by:** None (can start immediately)

**Status:** done

- [x] `image` 크레이트 또는 WebP 인코더를 활용한 애니메이션 WebP 생성 함수(`encode_animated_webp`) 구현
- [x] 다중 아바타 프레임(RGBA 이미지 버퍼)을 2D 바둑판 그리드로 결합하는 스프라이트 시트 패커 구현
- [x] 프레임별 바운딩 박스, 오프셋, 지속 시간(딜레이 ms)을 포함하는 스프라이트 아틀라스 메타데이터 JSON 생성 구조체 구현
- [x] 프레임 입력에 따른 WebP 바이너리 및 스프라이트시트 이미지/JSON 생성 검증 단위 테스트 작성
