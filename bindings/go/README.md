# maplestory-openapi (Go)

Official high-performance Go client bindings for NEXON MapleStory OpenAPI, powered by Rust C-ABI FFI via `cgo`.

## 🚀 Prerequisites
- Go 1.20+
- C compiler (GCC or Clang)
- Built native Rust library (`maplestory_openapi_c`)

## 📖 Usage

```go
package main

import (
    "fmt"
    "os"
    "github.com/yourusername/maplestory-openapi-rust/bindings/go"
)

func main() {
    client, err := maplestory.FromEnv()
    if err != nil {
        panic(err)
    }
    defer client.Close()

    // 1. 올인원 대시보드 조회
    dashJson, err := client.GetCharacterDashboard("온앤온")
    if err != nil {
        panic(err)
    }
    fmt.Println(dashJson)

    // 2. 걷기 애니메이션 GIF 생성
    gifBytes, err := client.RenderCharacterGif("온앤온", "walk", 128, 128)
    if err != nil {
        panic(err)
    }
    os.WriteFile("walk.gif", gifBytes, 0644)
}
```
