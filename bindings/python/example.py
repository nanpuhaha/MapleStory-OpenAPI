import asyncio
import os
from maplestory_openapi import MapleStoryClient

async def run():
    print("=== [Python Binding Test] ===")
    api_key = os.getenv("NEXON_API_KEY", "test")
    client = MapleStoryClient(api_key)
    print("MapleStoryClient 초기화 성공!")

if __name__ == "__main__":
    asyncio.run(run())
