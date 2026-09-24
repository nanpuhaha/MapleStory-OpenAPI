const fs = require('fs');
const path = require('path');

// .env 파일 로드
const envPath = path.join(__dirname, '..', '..', '.env');
if (fs.existsSync(envPath)) {
  const envContent = fs.readFileSync(envPath, 'utf8');
  for (const line of envContent.split('\n')) {
    if (line.trim() && !line.startsWith('#') && line.includes('=')) {
      const [k, v] = line.trim().split('=', 2);
      process.env[k.trim()] = v.trim();
    }
  }
}

const { MapleStoryClient, AvatarCode } = require('./index.js');

async function main() {
  console.log('========================================');
  console.log(' Node.js 바인딩 실서버 연동 테스트');
  console.log('========================================');

  const apiKey = process.env.NEXON_API_KEY;
  const client = new MapleStoryClient(apiKey);
  console.log('1. MapleStoryClient 초기화 성공!');

  const target = '온앤온';
  console.log(`\n2. '${target}' 올인원 대시보드 조회 중...`);
  const dash = await client.getCharacterDashboard(target);

  console.log(`   [기본 정보] 이름: ${dash.basic.character_name}, 직업: ${dash.basic.character_class}, 레벨: Lv.${dash.basic.character_level}`);
  
  const combatPower = dash.stat.final_stat?.find(s => s.stat_name === '전투력')?.stat_value;
  console.log(`   [스펙 정보] 전투력: ${Number(combatPower).toLocaleString()}`);

  if (dash.guild) {
    const noblePts = dash.guild.guild_noblesse_skill?.reduce((sum, s) => sum + (s.skill_level || 0), 0);
    console.log(`   [길드 정보] 길드명: ${dash.guild.guild_name}, 노블레스 스킬: ${noblePts} 포인트`);
  }

  console.log('\n3. 걷기 애니메이션 GIF 생성 테스트 중...');
  const gifBuffer = await client.renderCharacterGif(target, 'walk', 96, 96);
  console.log(`   [GIF 생성] 성공! (Buffer 크기: ${gifBuffer.length} bytes)`);

  console.log('\n✅ Node.js 바인딩이 100% 정상 작동합니다!');
}

main().catch(err => {
  console.error('오류 발생:', err);
  process.exit(1);
});
