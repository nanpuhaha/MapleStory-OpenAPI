import asyncio
import os
import sys

# Windows 콘솔 인코딩 대응
if sys.platform == "win32":
    sys.stdout.reconfigure(encoding="utf-8")

# .env 파일에서 환경변수 로드
env_file = os.path.join(os.path.dirname(__file__), "..", "..", ".env")
if os.path.exists(env_file):
    with open(env_file, "r", encoding="utf-8") as f:
        for line in f:
            if line.strip() and not line.startswith("#") and "=" in line:
                k, v = line.strip().split("=", 1)
                os.environ[k.strip()] = v.strip()

from maplestory_api import MapleStoryClient

async def main():
    print("========================================")
    print(" Python 바인딩 실서버 연동 테스트")
    print("========================================")
    
    client = MapleStoryClient.from_env()
    print("1. MapleStoryClient 초기화 성공!")

    target = "온앤온"
    print(f"\n2. '{target}' 올인원 대시보드 조회 중...")
    dash = await client.get_character_dashboard(target)

    print(f"   [기본 정보] 이름: {dash['basic']['character_name']}, 직업: {dash['basic']['character_class']}, 레벨: Lv.{dash['basic']['character_level']}")
    
    # final_stat 배열에서 전투력 찾기
    for stat_entry in dash.get('stat', {}).get('final_stat', []):
        if stat_entry.get('stat_name') == '전투력':
            print(f"   [스펙 정보] 전투력: {int(stat_entry.get('stat_value')):,}")
            break
    
    guild = dash.get("guild")
    if guild:
        skills = guild.get("guild_noblesse_skill", [])
        noble_pts = sum(s.get("skill_level", 0) for s in skills)
        print(f"   [길드 정보] 길드명: {guild['guild_name']}, 노블레스 스킬 총합: {noble_pts} 포인트")
    
    print("\n3. 걷기 애니메이션 GIF 생성 테스트 중...")
    gif_bytes = await client.render_character_gif(target, preset="walk", width=96, height=96)
    print(f"   [GIF 생성] 성공! (크기: {len(gif_bytes)} bytes)")
    
    print("\n✅ Python 바인딩이 100% 정상 작동합니다!")

if __name__ == "__main__":
    asyncio.run(main())
