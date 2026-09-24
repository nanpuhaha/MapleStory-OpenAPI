# MapleStory.OpenApi (.NET / C#)

Official high-performance .NET bindings for NEXON MapleStory OpenAPI, powered by Rust C-ABI FFI.

## 🚀 Features
- Multi-target support: `.NET Standard 2.0`, `.NET 8.0`, `.NET 9.0`
- Fully asynchronous (`Task<string>`, `Task<byte[]>`)
- Complete memory-safe lifetime management with `IDisposable`
- Direct Animated GIF byte generation for Windows Forms, WPF, Avalonia, MAUI, ASP.NET Core, and Unity

## 📖 Usage

```csharp
using MapleStory.OpenApi;

using var client = MapleStoryClient.FromEnv();

// 1. 올인원 대시보드 조회
string dashboardJson = await client.GetCharacterDashboardAsync("온앤온");

// 2. 걷기 애니메이션 GIF 바이트 생성
byte[] gifBytes = await client.RenderCharacterGifAsync("온앤온", "walk", 128, 128);
File.WriteAllBytes("walk.gif", gifBytes);
```
