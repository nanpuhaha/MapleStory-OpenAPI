using System;
using System.IO;
using System.Text.Json;
using System.Threading.Tasks;

namespace MapleStory.OpenApi
{
    class Program
    {
        static async Task Main(string[] args)
        {
            Console.OutputEncoding = System.Text.Encoding.UTF8;
            Console.WriteLine("========================================");
            Console.WriteLine(" C# .NET 바인딩 실서버 연동 테스트");
            Console.WriteLine("========================================");

            // .env 파일 로드
            string envPath = Path.Combine(AppContext.BaseDirectory, "..", "..", "..", "..", "..", ".env");
            if (File.Exists(envPath))
            {
                foreach (var line in File.ReadAllLines(envPath))
                {
                    if (!string.IsNullOrWhiteSpace(line) && !line.StartsWith("#") && line.Contains("="))
                    {
                        var parts = line.Split(new[] { '=' }, 2);
                        Environment.SetEnvironmentVariable(parts[0].Trim(), parts[1].Trim());
                    }
                }
            }

            using var client = MapleStoryClient.FromEnv();
            Console.WriteLine("1. MapleStoryClient 초기화 성공!");

            string target = "온앤온";
            Console.WriteLine($"\n2. '{target}' 올인원 대시보드 조회 중...");
            string json = await client.GetCharacterDashboardAsync(target);

            using var doc = JsonDocument.Parse(json);
            var root = doc.RootElement;
            var basic = root.GetProperty("basic");
            var name = basic.GetProperty("character_name").GetString();
            var job = basic.GetProperty("character_class").GetString();
            var level = basic.GetProperty("character_level").GetInt64();

            Console.WriteLine($"   [기본 정보] 이름: {name}, 직업: {job}, 레벨: Lv.{level}");

            if (root.TryGetProperty("stat", out var stat) && stat.TryGetProperty("final_stat", out var finalStats))
            {
                foreach (var s in finalStats.EnumerateArray())
                {
                    if (s.GetProperty("stat_name").GetString() == "전투력")
                    {
                        long cp = long.Parse(s.GetProperty("stat_value").GetString() ?? "0");
                        Console.WriteLine($"   [스펙 정보] 전투력: {cp:N0}");
                        break;
                    }
                }
            }

            if (root.TryGetProperty("guild", out var guild) && guild.ValueKind == JsonValueKind.Object)
            {
                string? guildName = guild.GetProperty("guild_name").GetString();
                int noblePts = 0;
                if (guild.TryGetProperty("guild_noblesse_skill", out var skills))
                {
                    foreach (var sk in skills.EnumerateArray())
                    {
                        noblePts += sk.GetProperty("skill_level").GetInt32();
                    }
                }
                Console.WriteLine($"   [길드 정보] 길드명: {guildName}, 노블레스 스킬 총합: {noblePts} 포인트");
            }

            Console.WriteLine("\n3. 걷기 애니메이션 GIF 생성 테스트 중...");
            byte[] gifBytes = await client.RenderCharacterGifAsync(target, "walk", 96, 96);
            Console.WriteLine($"   [GIF 생성] 성공! (크기: {gifBytes.Length} bytes)");

            Console.WriteLine("\n✅ C# .NET 바인딩이 100% 정상 작동합니다!");
        }
    }
}
