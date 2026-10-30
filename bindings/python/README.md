# maplestory-openapi (Python)

Official high-performance Python bindings for NEXON MapleStory OpenAPI, powered by Rust.

## 🚀 Installation

```bash
pip install maturin
maturin develop
```

## 📖 Quick Start

```python
import asyncio
import os
from maplestory_api import MapleStoryClient

async def main():
    client = MapleStoryClient.from_env()

    # 1. 닉네임 한방 올인원 대시보드 (전투력, 유니온, 길드, 노블스킬, 코디)
    dash = await client.get_character_dashboard("온앤온")
    print(f"이름: {dash['basic']['character_name']}")
    print(f"전투력: {dash['stat']['combat_power']}")
    print(f"길드: {dash.get('guild', {}).get('guild_name')} (노블 {dash.get('guild', {}).get('noblesse_points')}p)")

    # 2. 걷기 애니메이션 GIF 바로 생성
    gif_bytes = await client.render_character_gif("온앤온", preset="walk", width=128, height=128)
    with open("walk.gif", "wb") as f:
        f.write(gif_bytes)
    print("walk.gif 생성 완료!")

if __name__ == "__main__":
    asyncio.run(main())
```
