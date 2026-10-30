# maplestory-api (Node.js)

Official high-performance Node.js bindings for NEXON MapleStory OpenAPI, powered by Rust and `napi-rs`.

## 🚀 Installation

```bash
npm install maplestory-api
```

## 📖 Quick Start

```typescript
import { MapleStoryClient } from 'maplestory-api';
import * as fs from 'fs';

async function main() {
  const client = new MapleStoryClient(process.env.NEXON_API_KEY!);

  // 1. 올인원 종합 대시보드 조회
  const dash = await client.getCharacterDashboard('온앤온');
  console.log(`이름: ${dash.basic.character_name}`);
  console.log(`전투력: ${dash.stat.combat_power}`);

  // 2. 걷기 애니메이션 GIF 바로 생성 (Buffer 반환)
  const gifBuffer = await client.renderCharacterGif('온앤온', 'walk', 128, 128);
  fs.writeFileSync('walk.gif', gifBuffer);
  console.log('walk.gif 생성 완료!');
}

main();
```
