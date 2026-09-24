# 기여 가이드라인 (Contributing Guide)

`MapleStory-OpenAPI-Rust` 프로젝트에 관심을 가져주셔서 감사합니다!  
버그 제보, 기능 제안, 문서 보강, 코드 기여 등 모든 형태의 기여를 환영합니다.

---

## 🛠️ 개발 환경 설정 (Development Setup)

### 1. 필수 도구
- **Rust Toolchain**: 최신 Stable 버전 (`rustup update stable`)
- **Git**
- (선택 사항) 다국어 바인딩 빌드 시:
  - Python: `python 3.9+`, `pip install maturin`
  - Node.js: `node 18+`, `npm`
  - .NET: `.NET 8.0 SDK`
  - Go: `go 1.20+`

### 2. 저장소 복제 및 빌드
```bash
git clone https://github.com/yourusername/maplestory-openapi-rust.git
cd maplestory-openapi-rust

# 전체 워크스페이스 빌드 확인
cargo check --workspace
```

### 3. API 키 설정 (테스트용)
테스트 및 예제 실행을 위해 프로젝트 루트에 `.env` 파일을 생성하고 본인의 넥슨 OpenAPI 키를 설정합니다:
```env
NEXON_API_KEY=test_xxxxxxxxxxxxxxxxxxxxxxxxxxxx
```
*(주의: `.env` 파일은 `.gitignore`에 등록되어 있어 원격에 커밋되지 않습니다)*

---

## 🧪 테스트 및 품질 검증

PR을 제출하기 전에 반드시 아래 세 가지 검증을 통과해야 합니다:

### 1. 코드 포맷 검사
```bash
cargo fmt --all -- --check
```
*불일치 발생 시 `cargo fmt --all` 명령어로 자동 정렬할 수 있습니다.*

### 2. 린트(Clippy) 검사 (경고 0개 유지)
```bash
cargo clippy --workspace --all-targets
```

### 3. 전체 테스트 실행
```bash
cargo test --workspace
```

---

## 📝 커밋 메시지 컨벤션 (Conventional Commits)

일관된 릴리즈 노트 작성을 위해 [Conventional Commits](https://www.conventionalcommits.org/) 규격을 따릅니다:

- `feat`: 새로운 기능 추가 (예: `feat(client): add new union ranking endpoint`)
- `fix`: 버그 수정 (예: `fix(avatar): correct overall layer concealment rule`)
- `docs`: 문서 수정 및 보강 (예: `docs: update discord bot recipe`)
- `refactor`: 기능 변경 없는 코드 리팩토링
- `test`: 테스트 코드 추가 및 수정
- `chore`: 빌드 설정, 의존성 업데이트 등

---

## 🔀 Pull Request (PR) 절차

1. 저장소를 Fork하고 기능 브랜치를 생성합니다 (`git checkout -b feat/my-new-feature`).
2. 변경 사항을 구현하고 관련 단위 테스트를 추가합니다.
3. `cargo fmt`, `cargo clippy`, `cargo test`를 실행하여 문제가 없는지 확인합니다.
4. 커밋을 작성하고 본인의 Fork 저장소로 Push합니다.
5. `master` 브랜치를 대상으로 Pull Request를 오픈합니다.
6. PR 설명에 변경 이유와 테스트 결과를 상세히 작성해 주시면 빠른 검토에 큰 도움이 됩니다.

---

## 💬 질문 및 버그 제보

궁금한 점이나 제안 사항이 있다면 언제든지 [GitHub Issues](https://github.com/yourusername/maplestory-openapi-rust/issues)에 글을 남겨주세요!
