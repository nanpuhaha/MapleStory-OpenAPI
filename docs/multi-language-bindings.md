# Rust 기반 다국어 바인딩 가이드 (Multi-Language Bindings)

이 문서는 Rust로 작성된 `maplestory-openapi` 코어 로직을 기반으로 **Python, JavaScript/TypeScript, Go, C#, Swift, Kotlin 등 다양한 언어용 라이브러리(패키지)**를 제작하는 원리와 실무 제작 방법을 설명합니다.

---

## 1. 왜 Rust 코드로 다른 언어 라이브러리를 만들 수 있는가?

### 💡 핵심 원리
1. **가비지 컬렉터(GC) 없는 네이티브 바이너리**:
   - Go나 Java, C#은 자체 런타임과 GC가 있어서 다른 언어 내부로 들어가기 매우 까다롭습니다.
   - 반면 Rust는 C/C++과 동일하게 **런타임 오버헤드가 없고 기계어로 직접 컴파일**되므로 어떤 언어의 프로세스 안에도 부드럽게 임베딩될 수 있습니다.
2. **C-ABI(Application Binary Interface) 호환성**:
   - 현대의 거의 모든 프로그래밍 언어(Python, JS, Go, Java, C#, Swift 등)는 C 언어 규격(C-ABI)으로 노출된 함수를 직접 호출(FFI; Foreign Function Interface)할 수 있는 기능을 내장하고 있습니다.
3. **메모리 안전성과 제로-코스트 추상화**:
   - C/C++로 바인딩을 짜면 메모리 누수나 세그멘테이션 폴트(Crash)가 발생하기 쉽지만, Rust는 컴파일 타임에 메모리 안전성을 보장하므로 타 언어에서 가져다 써도 프로그램이 죽지 않는 고성능 네이티브 모듈을 만들 수 있습니다.

---

## 2. 지원 가능한 언어 및 공식 생태계 도구

| 대상 언어 / 플랫폼 | 공식 추천 도구 (Toolchain) | 빌드 및 배포 산출물 | 주요 활용처 |
|---|---|---|---|
| **Python** | **PyO3 + maturin** | CPython Native Extension (`.whl`, `pip install`) | 데이터 분석, 주피터 노트북, Django/FastAPI 백엔드, 디스코드 봇 |
| **Node.js (JS/TS)** | **napi-rs** | Node-API Native Addon (`.node`, `npm install`) | 고성능 백엔드, Electron 데스크톱 앱, CLI 도구 |
| **Web / Browser** | **wasm-bindgen + wasm-pack** | WebAssembly (`.wasm` + `.d.ts`) | React/Vue 프론트엔드, 브라우저 내 직접 연산/렌더링 |
| **Kotlin (Android)** | **Mozilla UniFFI** | Android AAR / JNI 라이브러리 | 안드로이드 앱 개발 |
| **Swift (iOS/macOS)** | **Mozilla UniFFI** | Swift Package (XCFramework) | 아이폰/아이패드/맥 앱 개발 |
| **C# (.NET)** | **cbindgen / ClrMD** | DLL + P/Invoke C# 래퍼 (NuGet) | Unity 게임 클라이언트, .NET 백엔드 |
| **Go** | **cgo + cbindgen** | C 헤더 + 정적 라이브러리 (`.a`) | Go 마이크로서비스 백엔드 |

---

## 3. 추천 프로젝트 아키텍처 (Cargo Workspace 구조)

코어 Rust 라이브러리를 오염시키지 않고 깔끔하게 관리하려면 Cargo Workspace를 구축하는 것이 표준입니다:

```text
MapleStory-OpenAPI-Rust/
├── Cargo.toml                     # 워크스페이스 루트
├── maplestory-openapi/            # [Core] 순수 Rust 라이브러리 (crates.io 배포용)
│   ├── Cargo.toml
│   └── src/
├── bindings/
│   ├── python/                    # [Python] PyO3 바인딩 (PyPI: .whl)
│   │   ├── Cargo.toml
│   │   ├── pyproject.toml
│   │   └── src/lib.rs
│   ├── nodejs/                    # [Node.js] napi-rs 바인딩 (npm: .node + index.d.ts)
│   │   ├── Cargo.toml
│   │   ├── package.json
│   │   └── src/lib.rs
│   ├── wasm/                      # [WebAssembly] 브라우저 바인딩
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── c/                         # [C-ABI FFI] C, C++, 모바일 공통 네이티브 코어
│   │   ├── Cargo.toml
│   │   ├── include/maplestory_openapi.h
│   │   └── src/lib.rs
│   ├── dotnet/                    # [C# / .NET] NuGet 패키지 (net8.0/net9.0/netstandard2.0)
│   │   ├── MapleStory.OpenApi.csproj
│   │   └── MapleStoryClient.cs
│   ├── go/                        # [Go] cgo 패키지
│   │   ├── go.mod
│   │   └── maplestory.go
│   └── mobile/                    # [모바일] iOS (Swift) & Android (Kotlin)
│       ├── swift/MapleStoryClient.swift
│       └── kotlin/MapleStoryClient.kt
└── docs/
```

---

## 4. 구체적인 언어별 제작 방법

### 🐍 1) Python 라이브러리 제작 (`PyO3` + `maturin`)

가장 많이 사용되는 방식입니다. Rust 코드를 작성하고 명령어 한 줄이면 `pip install` 가능한 휠(`whl`) 파일이 나옵니다.

#### 1단계: `bindings/python/Cargo.toml` 설정
```toml
[package]
name = "maplestory-openapi-python"
version = "0.1.0"
edition = "2021"

[lib]
name = "maplestory_openapi"
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.22", features = ["extension-module", "abi3-py38"] }
pyo3-async-runtimes = { version = "0.22", features = ["tokio-runtime"] }
maplestory-openapi = { path = "../../maplestory-openapi" }
tokio = { version = "1", features = ["full"] }
```

#### 2단계: Rust에서 파이썬 클래스 및 비동기 함수 노출 (`src/lib.rs`)
```rust
use pyo3::prelude::*;
use maplestory_openapi::MapleStoryClient as CoreClient;

#[pyclass]
struct MapleStoryClient {
    inner: CoreClient,
}

#[pymethods]
impl MapleStoryClient {
    #[new]
    fn new(api_key: &str) -> PyResult<Self> {
        let client = CoreClient::new(api_key)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(Self { inner: client })
    }

    /// 파이썬의 async/await 함수로 직접 노출
    fn get_character_dashboard<'py>(&self, py: Python<'py>, character_name: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let dash = client.get_character_dashboard_by_name(&character_name).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            
            // 파이썬 딕셔너리나 직렬화된 JSON 문자열 반환
            Ok(serde_json::to_string(&dash).unwrap())
        })
    }
}

#[pymodule]
fn maplestory_openapi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MapleStoryClient>()?;
    Ok(())
}
```

#### 3단계: 빌드 및 실행
```bash
# maturin 도구 설치
pip install maturin

# 개발 환경에 즉시 로컬 빌드 및 설치
maturin develop

# 배포용 Wheel(.whl) 패키지 빌드
maturin build --release
```

#### 4단계: 파이썬에서 실제 사용
```python
import asyncio
from maplestory_openapi import MapleStoryClient

async def main():
    client = MapleStoryClient("YOUR_API_KEY")
    dash_json = await client.get_character_dashboard("온앤온")
    print(dash_json)

asyncio.run(main())
```

---

### 🌐 2) JavaScript / Node.js 라이브러리 제작 (`napi-rs`)

Node.js 환경에서는 `napi-rs`가 전 세계 표준으로 쓰입니다 (SWC, Prisma, Tailwind 등이 모두 이 도구로 Rust 기반 고성능 npm 패키지를 제작함).

#### 1단계: napi-rs 프로젝트 생성
```bash
npm install -g @napi-rs/cli
napi new bindings/nodejs
```

#### 2단계: Rust에서 JS 함수 및 클래스 노출
```rust
use napi_derive::napi;
use maplestory_openapi::{MapleStoryClient as CoreClient, AnimationPreset};

#[napi]
pub struct MapleStoryClient {
    inner: CoreClient,
}

#[napi]
impl MapleStoryClient {
    #[napi(constructor)]
    pub fn new(api_key: String) -> napi::Result<Self> {
        let inner = CoreClient::new(&api_key)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(Self { inner })
    }

    /// TypeScript의 Promise<Buffer>로 노출 (애니메이션 GIF 생성)
    #[napi]
    pub async fn render_character_walk_gif(&self, character_name: String) -> napi::Result<napi::bindgen_prelude::Buffer> {
        let gif_bytes = self.inner
            .render_character_gif_by_name(&character_name, &AnimationPreset::Walk, Some(128), Some(128))
            .await
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        
        Ok(gif_bytes.into())
    }
}
```

#### 3단계: 빌드 및 TypeScript 자동 생성
```bash
npm run build
```
- 컴파일이 완료되면 `index.d.ts` 타입 정의 파일과 바이너리가 자동으로 생성되어 완전한 TypeScript 지원을 제공합니다.

#### 4단계: Node.js에서 사용
```javascript
const { MapleStoryClient } = require('maplestory-openapi-node');
const fs = require('fs');

async function main() {
  const client = new MapleStoryClient(process.env.NEXON_API_KEY);
  const gifBuffer = await client.renderCharacterWalkGif('온앤온');
  fs.writeFileSync('walk.gif', gifBuffer);
}
main();
```

---

### 📱 3) 모바일/멀티플랫폼 단일 생성 (`Mozilla UniFFI`)

Mozilla가 Firefox 모바일 앱을 위해 개발한 오픈소스 도구로, **Rust 코드 한 벌로 Swift(iOS), Kotlin(Android), Python 바인딩 코드를 동시에 자동 생성**해 줍니다.
- `.udl` (Interface Definition Language) 파일이나 Rust 매크로를 작성해 두면, 컴파일러가 각 언어별 클래스와 브릿지 코드를 자동으로 만들어줍니다.
- 모바일 크로스 플랫폼 앱을 고려할 때 가장 이상적인 솔루션입니다.

---

## 5. 결론 및 로드맵 제안

1. **현재 상태**: 순수 Rust 크레이트(`maplestory-openapi`)의 코어 엔진이 완벽하게 갖추어져 있습니다.
2. **확장 방향**:
   - 데이터 분석/머신러닝/디스코드 봇 유저를 위해 **Python 바인딩 (`PyO3 + maturin`)**을 1순위로 추가하는 것을 권장합니다.
   - 웹 생태계/프론트엔드 유저를 위해 **Node.js (`napi-rs`)** 또는 **Wasm (`wasm-bindgen`)** 바인딩을 2순위로 추가할 수 있습니다.
