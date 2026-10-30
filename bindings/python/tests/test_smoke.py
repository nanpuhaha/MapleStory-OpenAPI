import asyncio
import os
import unittest
from maplestory_api import MapleStoryClient, AvatarCode


class TestMapleStoryBindingsSmoke(unittest.TestCase):
    def test_client_methods_exposed(self):
        client = MapleStoryClient("test_api_key")
        expected_methods = [
            "get_character_dashboard",
            "get_character_basic",
            "get_character_coordination",
            "render_character_gif",
            "get_guild_basic",
            "get_guild_profile",
            "get_user_union",
            "get_user_union_artifact",
            "get_user_union_raider",
            "get_overall_ranking",
            "get_union_ranking",
            "get_guild_ranking",
            "get_dojang_ranking",
            "get_starforce_history",
            "get_potential_history",
            "get_cube_history",
            "get_soul_potential_history",
            "get_notice_list",
            "get_update_notice_list",
            "get_event_notice_list",
            "get_cashshop_notice_list",
            "get_account_summary",
            "get_scheduler_character_state",
        ]
        for method in expected_methods:
            self.assertTrue(hasattr(client, method), f"Missing method: {method}")

    def test_avatar_code_roundtrip(self):
        hex_code = "a1" * 512
        avatar = AvatarCode(hex_code)
        self.assertEqual(avatar.code(), hex_code)
        self.assertEqual(len(avatar.to_bytes()), 512)
        url = avatar.to_look_url(action="A00", emotion="E00", width=96, height=96)
        self.assertIn(hex_code, url)


if __name__ == "__main__":
    unittest.main()
