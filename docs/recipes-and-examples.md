# 실전 활용 레시피 (Recipes & Use Cases)

이 문서는 `maplestory-openapi`를 활용하여 실무에서 가장 많이 제작하는 **디스코드 봇(Discord Bot)**, **웹 API 백엔드**, **아바타 GIF 생성기**의 완성형 코드 예제를 제공합니다.

---

## 🤖 레시피 1: 캐릭터 스펙 조회 디스코드 봇 (Discord Bot)

유저가 `/스펙 온앤온`을 입력하면 캐릭터의 레벨, 직업, 전투력, 유니온, 길드, 코디 룩을 화려한 디스코드 임베드로 전송합니다.

### 🐍 Python (`discord.py`)
```python
import os
import discord
from discord.ext import commands
from maplestory_openapi import MapleStoryClient

intents = discord.Intents.default()
bot = commands.Bot(command_prefix="!", intents=intents)
maple = MapleStoryClient(os.environ["NEXON_API_KEY"])

@bot.tree.command(name="스펙", description="메이플스토리 캐릭터 종합 스펙 정보를 조회합니다.")
async def lookup(interaction: discord.Interaction, 닉네임: str):
    await interaction.response.defer()
    
    try:
        dash = await maple.get_character_dashboard_by_name(닉네임)
        
        embed = discord.Embed(
            title=f"Lv.{dash['level']} {dash['name']} ({dash['world']})",
            description=f"**직업**: {dash['class']} | **인기도**: {dash['popularity']:,}",
            color=0xF57C00
        )
        embed.set_thumbnail(url=dash['avatar_url'])
        
        # 핵심 스펙
        embed.add_field(name="⚔️ 전투력", value=f"{dash['combat_power']:,}", inline=True)
        embed.add_field(name="⭐ 스타포스", value=f"{dash['total_starforce']}성", inline=True)
        embed.add_field(name="🔮 아케인 / 어센틱", value=f"{dash['arcane_force']} / {dash['authentic_force']}", inline=True)
        
        # 길드 및 유니온
        guild_info = f"{dash['guild_name']} (Lv.{dash['guild_level']}, 노블 {dash['noblesse_skill_points']}p)" if dash['guild_name'] else "없음"
        embed.add_field(name="🏰 길드", value=guild_info, inline=True)
        embed.add_field(name="🛡️ 유니온", value=f"Lv.{dash['union_level']} ({dash['union_grade']})", inline=True)
        embed.add_field(name="🥋 무릉도장", value=f"{dash['dojang_best_floor']}층", inline=True)
        
        await interaction.followup.send(embed=embed)
    except Exception as e:
        await interaction.followup.send(f"❌ 조회 실패: {e}")

bot.run(os.environ["DISCORD_BOT_TOKEN"])
```

### 🟩 Node.js / TypeScript (`discord.js`)
```typescript
import { Client, GatewayIntentBits, EmbedBuilder } from 'discord.js';
import { MapleStoryClient } from 'maplestory-openapi';

const client = new Client({ intents: [GatewayIntentBits.Guilds] });
const maple = MapleStoryClient.fromEnv();

client.on('interactionCreate', async (interaction) => {
  if (!interaction.isChatInputCommand() || interaction.commandName !== '스펙') return;

  const characterName = interaction.options.getString('닉네임', true);
  await interaction.deferReply();

  try {
    const dash = await maple.getCharacterDashboardByName(characterName);

    const embed = new EmbedBuilder()
      .setTitle(`Lv.${dash.level} ${dash.name} (${dash.world})`)
      .setDescription(`**직업**: ${dash.character_class} | **인기도**: ${dash.popularity.toLocaleString()}`)
      .setThumbnail(dash.avatar_url)
      .setColor(0x00A8FF)
      .addFields(
        { name: '⚔️ 전투력', value: dash.combat_power.toLocaleString(), inline: true },
        { name: '⭐ 스타포스', value: `${dash.total_starforce}성`, inline: true },
        { name: '🔮 아케인/어센틱', value: `${dash.arcane_force} / ${dash.authentic_force}`, inline: true },
        { name: '🏰 소속 길드', value: dash.guild_name ? `${dash.guild_name} (노블 ${dash.noblesse_skill_points}p)` : '없음', inline: true },
        { name: '🛡️ 유니온', value: `Lv.${dash.union_level} (${dash.union_grade})`, inline: true }
      );

    await interaction.editReply({ embeds: [embed] });
  } catch (err: any) {
    await interaction.editReply(`❌ 조회 실패: ${err.message}`);
  }
});

client.login(process.env.DISCORD_BOT_TOKEN);
```

---

## 🌐 레시피 2: 초고속 캐릭터 스펙 검색 REST API 서버

### 🦀 Rust (`Axum`)
```rust
use axum::{extract::{Path, State}, routing::get, Json, Router};
use maplestory_openapi::{MapleStoryClient, CharacterDashboard};
use std::sync::Arc;

struct AppState {
    client: MapleStoryClient,
}

async fn get_character(
    Path(name): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<CharacterDashboard>, (axum::http::StatusCode, String)> {
    state.client.get_character_dashboard_by_name(&name).await
        .map(Json)
        .map_err(|e| (axum::http::StatusCode::NOT_FOUND, e.to_string()))
}

#[tokio::main]
async fn main() {
    let client = MapleStoryClient::from_env().expect("API Key required");
    let state = Arc::new(AppState { client });

    let app = Router::new()
        .route("/api/character/:name", get(get_character))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("API 서버 가동 중: http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
```

---

## 🎬 레시피 3: 아바타 애니메이션 GIF 생성 및 저장

캐릭터의 인게임 룩을 추출하여 **걷기(Walk)** 모션과 **윙크(Wink)** 모션이 결합된 투명 배경 애니메이션 GIF를 파일로 저장합니다.

```rust
use maplestory_openapi::{MapleStoryClient, AnimationPreset};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MapleStoryClient::from_env()?;

    println!("'온앤온' 캐릭터의 걷기 애니메이션 GIF 생성 중...");
    
    // 128x128 캔버스 크기로 걷기 모션 GIF 렌더링
    let gif_data = client
        .render_character_gif_by_name("온앤온", &AnimationPreset::Walk, Some(128), Some(128))
        .await?;

    fs::write("walk.gif", gif_data)?;
    println!("✅ walk.gif 파일 생성 완료!");

    Ok(())
}
```
