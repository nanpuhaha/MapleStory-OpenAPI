# maplestory-openapi-wasm

WebAssembly bindings for NEXON MapleStory Avatar & Look rendering in modern browsers (React, Vue, Svelte).

## 🚀 Build

```bash
wasm-pack build --target web
```

## 📖 Usage in Browser

```javascript
import init, { WasmAvatarCode } from './pkg/maplestory_openapi_wasm.js';

await init();

// 1. 아바타 코드로 객체 초기화
const avatar = new WasmAvatarCode("JFKCLCKKAMIBFPCLOJLPFLPENKBMBCDC...");

// 2. 동적 포즈 URL 즉시 생성
const jumpUrl = avatar.toLookUrl("A06", "E00", "W00", 128, 128);

// 3. 걷기 애니메이션 프레임 및 지연시간(ms) 획득
const frames = avatar.getAnimationFrames("walk", 128, 128);
console.log(frames);
// [{ url: "...?action=A02...", delay_ms: 250 }, { url: "...?action=A03...", delay_ms: 250 }]
```
