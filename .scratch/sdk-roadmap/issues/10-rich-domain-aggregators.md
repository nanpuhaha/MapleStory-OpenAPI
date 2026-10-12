# 10: Rich Domain Aggregators (Guild Noblesse & User Account Summary)

**What to build:** Introduce high-level domain aggregation and analytics helpers for Guilds (comprehensive guild profile with noblesse skill breakdown and combat power statistics of active members) and Accounts (account-level union/character overview).

**Blocked by:** None

**Status:** done

- [x] Add `GuildProfile` model and `client.guild().get_guild_profile(&self, guild_name: &str, world_name: &str)` that aggregates guild basic info, noblesse stats, and optionally concurrent basic info of guild members.
- [x] Add `AccountSummary` model combining `client.user().get_character_list()`, `client.user().get_achievement()`, and main character union levels.
- [x] Provide test mocks and verify error tolerance on partial member lookup failures.
