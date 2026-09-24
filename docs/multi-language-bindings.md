# Rust 기반 다국어 네이티브 바인딩 아키텍처 (Multi-Language Bindings)

이 문서는 Rust로 작성된 `maplestory-openapi` 코어 엔진을 기반으로 **Python, Node.js/TypeScript, WebAssembly, C# .NET, Go, 모바일(iOS Swift / Android Kotlin)** 공식 네이티브 바인딩을 구축한 아키텍처 원리와 사용 방법을 설명합니다.

---

## 1. 다국어 바인딩 원리 및 아키텍처

Rust는 가비지 컬렉터(GC)가 없고 런타임 오버헤드가 없으며, 표준 C-ABI(Application Binary Interface) 규격을 완벽하게 지원합니다.  
따라서 코어 로직(네트워크 통신, 병렬 취합, JSON 역직렬화, 코디 합성, GIF 인코딩)을 단 한 번 Rust로 견고하게 작성하면, 모든 프로그래밍 언어에서 제로-오버헤드로 임베딩하여 실행할 수 있습니다.

```mermaid
flowchart TD
    Core["🦀 Rust Core Engine (maplestory-openapi)"]
    
    subgraph DirectBindings["직접 바인딩 (Zero-Copy / Native ABI)"]
        Py["🐍 Python (PyO3 + maturin)"]
        Node["🟩 Node.js / TS (napi-rs)"]
        Wasm["🌐 WebAssembly (wasm-bindgen)"]
    end
    
    subgraph CFiber["C-ABI FFI 코어 계층 (bindings/c)"]
        C_DLL["maplestory_openapi_c (.dll / .so / .dylib)"]
        C_H["maplestory_openapi.h (C Header)"]
    end
    
    subgraph FFIBindings["C-ABI FFI 기반 바인딩"]
        DotNet["🔷 C# / .NET (P/Invoke)"]
        GoLang["🐹 Go (cgo)"]
        iOS["🍏 iOS (Swift Native FFI)"]
        Android["🤖 Android (Kotlin / JNI)"]
    end
    
    Core --> DirectBindings
    Core --> CFiber
    CFiber --> FFIBindings
```

---

## 2. 지원 언어별 구현 내역 및 빠른 시작

### 🐍 1. Python (`bindings/python`)
- **기술 스택**: `PyO3 0.22`, `maturin`, `pyo3-async-runtimes`, `pythonize`
- **특징**: Rust 모델을 파이썬 네이티브 `dict`로 제로-카피 직렬화하여 반환. 비동기 `async/await` 완벽 지원.
- **빌드 및 로컬 설치**:
  ```bash
  cd bindings/python
  maturin develop --release
  ```
- **사용 예시**:
  ```python
  import asyncio
  from maplestory_openapi import MapleStoryClient

  async def main():
      client = MapleStoryClient.from_env()
      dash = await client.get_character_dashboard_by_name("온앤온")
      print(f"{dash['name']} 전투력: {dash['combat_power']:,}")
      
      # 걷기 애니메이션 GIF 바이트 생성
      gif_bytes = await client.render_character_gif("온앤온", "walk", 128, 128)
      with open("walk.gif", "wb") as f:
          f.write(gif_bytes)

  asyncio.run(main())
  ```

---

### 🟩 2. Node.js / TypeScript (`bindings/nodejs`)
- **기술 스택**: `napi-rs 2.16`
- **특징**: Node.js C++ 애드온 대비 가볍고 안전함. 컴파일 시 `index.d.ts` 타입 정의 자동 생성. Node `Buffer`로 GIF 반환.
- **빌드 및 로컬 테스트**:
  ```bash
  cd bindings/nodejs
  npm install
  npm run build
  node test_live.js
  ```
- **사용 예시**:
  ```typescript
  import { MapleStoryClient } from 'maplestory-openapi';
  import * as fs from 'fs';

  const client = MapleStoryClient.fromEnv();
  const dash = await client.getCharacterDashboardByName("온앤온");
  console.log(`${dash.name} 전투력: ${dash.combat_power.toLocaleString()}`);

  const gifBuffer = await client.renderCharacterGif("온앤온", "walk", 128, 128);
  fs.writeFileSync("walk.gif", gifBuffer);
  ```

---

### 🌐 3. WebAssembly / 브라우저 (`bindings/wasm`)
- **기술 스택**: `wasm-bindgen`, `serde-wasm-bindgen`
- **특징**: 서버를 거치지 않고 웹 브라우저/클라이언트에서 1024자리 Base16 아바타 코드를 디코딩하고, 실시간 포즈 URL 및 프레임 시퀀스를 0ms로 계산.
- **빌드**:
  ```bash
  cd bindings/wasm
  wasm-pack build --target web
  ```
- **브라우저 사용 예시**:
  ```javascript
  import init, { decodeAvatarLook, buildAvatarUrl } from './pkg/maplestory_openapi_wasm.js';

  await init();
  const parsed = decodeAvatarLook(avatarUrl);
  // 점프+웃는 표정 실시간 변환 URL 획득
  const jumpUrl = buildAvatarUrl(parsed.code, "jump", 1, "smile", 1, "default");
  ```

---

### 🔷 4. C# / .NET (`bindings/dotnet`)
- **기술 스택**: C# P/Invoke (.NET 8.0, .NET 9.0, .NET Standard 2.0 멀티 타겟)
- **특징**: `IDisposable` 및 비동기 `Task<string>`, `Task<byte[]>` 지원. Unity 게임 클라이언트 및 ASP.NET Core 백엔드에 즉시 적용 가능.
- **실행 및 테스트**:
  ```bash
  dotnet run --project bindings/dotnet/MapleStory.OpenApi.csproj --framework net8.0
  ```
- **사용 예시**:
  ```csharp
  using MapleStory.OpenApi;

  using var client = MapleStoryClient.FromEnv();
  var dash = client.GetCharacterDashboardByName("온앤온");
  Console.WriteLine($"닉네임: {dash.Name}, 전투력: {dash.CombatPower:N0}");

  byte[] gif = client.RenderCharacterGif("온앤온", "walk", 128, 128);
  File.WriteAllBytes("walk.gif", gif);
  ```

---

### 🐹 5. Go (`bindings/go`)
- **기술 스택**: `cgo` 기반 패키지 (`go 1.20+`)
- **특징**: Gin, Fiber 등 Go 기반 고성능 마이크로서비스 백엔드에 적합.
- **사용 예시**:
  ```go
  package main

  import (
      "fmt"
      "os"
      "maplestory-openapi/bindings/go"
  )

  func main() {
      client, _ := maplestory.FromEnv()
      defer client.Close()

      dash, _ := client.GetCharacterDashboardByName("온앤온")
      fmt.Printf("닉네임: %s, 전투력: %d\n", dash.Name, dash.CombatPower)

      gif, _ := client.RenderCharacterGif("온앤온", "walk", 128, 128)
      os.WriteFile("walk.gif", gif, 0644)
  }
  ```

---

### 📱 6. 모바일 (iOS Swift & Android Kotlin) (`bindings/mobile`)
- **iOS Swift**: C-ABI Native FFI를 활용한 Swift 클라이언트 (`bindings/mobile/ios/MapleStoryClient.swift`). `async/await` 및 `Foundation.Data`를 지원하여 SwiftUI에서 바로 사용 가능.
- **Android Kotlin**: JNI 네이티브 브릿지 클라이언트 (`bindings/mobile/android/MapleStoryClient.kt`). Kotlin Coroutines `suspend fun` 및 `ByteArray`를 지원하여 Jetpack Compose에서 즉시 사용 가능.

---

## 3. 워크스페이스 통합 빌드 및 검증

모든 네이티브 FFI 크레이트는 루트 `Cargo.toml` 워크스페이스에 통합 관리되므로, 다음 단 1줄의 커맨드로 전체 언어 바인딩을 일괄 컴파일 및 검증할 수 있습니다:

```bash
cargo check --workspace
cargo test --workspace
```
