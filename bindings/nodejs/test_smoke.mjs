import assert from "node:assert/strict";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);

// Smoke test verifying JsMapleStoryClient and JsAvatarCode domain surface
export function verifyBindingsSurface(mod) {
  const { MapleStoryClient, AvatarCode } = mod;
  const client = new MapleStoryClient("test_api_key");

  const expectedMethods = [
    "getCharacterDashboard",
    "getCharacterBasic",
    "getCharacterCoordination",
    "renderCharacterGif",
    "getGuildBasic",
    "getGuildProfile",
    "getUserUnion",
    "getUserUnionArtifact",
    "getUserUnionRaider",
    "getOverallRanking",
    "getUnionRanking",
    "getGuildRanking",
    "getDojangRanking",
    "getStarforceHistory",
    "getPotentialHistory",
    "getCubeHistory",
    "getSoulPotentialHistory",
    "getNoticeList",
    "getUpdateNoticeList",
    "getEventNoticeList",
    "getCashshopNoticeList",
    "getAccountSummary",
    "getSchedulerCharacterState",
  ];

  for (const method of expectedMethods) {
    assert.equal(
      typeof client[method],
      "function",
      `Expected MapleStoryClient.${method} to be a function`
    );
  }

  const hexCode = "a1".repeat(512);
  const avatar = new AvatarCode(hexCode);
  assert.equal(avatar.code(), hexCode);
  assert.equal(avatar.toBytes().length, 512);
  const lookUrl = avatar.toLookUrl("A00", "E00", null, 96, 96);
  assert.ok(lookUrl.includes(hexCode));
}
