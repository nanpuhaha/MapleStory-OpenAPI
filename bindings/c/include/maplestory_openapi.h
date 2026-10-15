#ifndef MAPLESTORY_OPENAPI_H
#define MAPLESTORY_OPENAPI_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ClientHandle ClientHandle;

/**
 * High-leverage typed combat analytics summary.
 * Eliminates the need for C/C++ callers to parse JSON for combat calculations.
 */
typedef struct {
    int64_t combat_power;
    double boss_damage;
    double ignore_monster_armor;
    double damage;
    double critical_rate;
    double critical_damage;
    int32_t total_starforce;
    int32_t arcane_force;
    int32_t authentic_force;
    int32_t dojang_best_floor;
    int32_t estimated_dojang_floor;
    double effective_damage_multiplier_300;
    double effective_damage_multiplier_380;
} MapleStoryCombatSummary;

/**
 * Retrieve the last error message recorded on the current thread.
 * Returns a newly allocated UTF-8 C string (free with maplestory_free_string),
 * or NULL if no error has occurred.
 */
char* maplestory_last_error_message(void);

/**
 * Create a new MapleStory client handle with an API key.
 * Returns NULL on failure.
 */
ClientHandle* maplestory_client_new(const char* api_key);

/**
 * Create a new MapleStory client handle with an API key and a custom base URL.
 * Returns NULL on failure.
 */
ClientHandle* maplestory_client_with_base_url(const char* api_key, const char* base_url);

/**
 * Create a new MapleStory client handle from NEXON_API_KEY environment variable.
 */
ClientHandle* maplestory_client_from_env(void);

/**
 * Free a MapleStory client handle.
 */
void maplestory_client_free(ClientHandle* handle);

/**
 * Fetch character combat analytics summary by OCID directly into a typed C struct.
 * Returns 0 on success.
 */
int32_t maplestory_character_get_combat_summary(
    ClientHandle* handle,
    const char* ocid,
    MapleStoryCombatSummary* out_summary
);

/**
 * Fetch character combat analytics summary directly by character name into a typed C struct.
 * Returns 0 on success.
 */
int32_t maplestory_get_character_combat_summary(
    ClientHandle* handle,
    const char* character_name,
    MapleStoryCombatSummary* out_summary
);

/**
 * Fetch synthesized character coordination (visible equipment and appearance) by OCID as a JSON string.
 * Returns 0 on success. The caller must free *out_json using maplestory_free_string.
 */
int32_t maplestory_character_get_coordination(
    ClientHandle* handle,
    const char* ocid,
    char** out_json
);

/**
 * Fetch synthesized character coordination (visible equipment and appearance) directly by character name as a JSON string.
 * Returns 0 on success. The caller must free *out_json using maplestory_free_string.
 */
int32_t maplestory_get_character_coordination(
    ClientHandle* handle,
    const char* character_name,
    char** out_json
);

/**
 * Fetch character dashboard JSON string.
 * Returns 0 on success. The caller must free *out_json using maplestory_free_string.
 */
int32_t maplestory_get_character_dashboard(
    ClientHandle* handle,
    const char* character_name,
    char** out_json
);

/**
 * Fetch guild profile (guild basic + member profiles) JSON string.
 * `date` may be NULL or empty for the latest available data.
 * Returns 0 on success. The caller must free *out_json using maplestory_free_string.
 */
int32_t maplestory_get_guild_profile(
    ClientHandle* handle,
    const char* guild_name,
    const char* world_name,
    const char* date,
    char** out_json
);

/**
 * Fetch user union level/grade information by character name as a JSON string.
 * `date` may be NULL or empty for the latest available data.
 * Returns 0 on success. The caller must free *out_json using maplestory_free_string.
 */
int32_t maplestory_get_user_union(
    ClientHandle* handle,
    const char* character_name,
    const char* date,
    char** out_json
);

/**
 * Fetch overall ranking information as a JSON string.
 * Optional string parameters (`world_name`, `character_class`, `ocid`) may be NULL or empty.
 * Pass `world_type < 0` to omit world_type filter, and `page <= 0` to omit page filter.
 * Returns 0 on success. The caller must free *out_json using maplestory_free_string.
 */
int32_t maplestory_get_overall_ranking(
    ClientHandle* handle,
    const char* date,
    const char* world_name,
    int32_t world_type,
    const char* character_class,
    const char* ocid,
    int32_t page,
    char** out_json
);

/**
 * Fetch the latest official in-game notices list as a JSON string.
 * Returns 0 on success. The caller must free *out_json using maplestory_free_string.
 */
int32_t maplestory_get_notice_list(
    ClientHandle* handle,
    char** out_json
);

/**
 * Fetch a complete account summary (union + artifact + champion + achievements) as a JSON string.
 * `date` may be NULL or empty for the latest available data.
 * Returns 0 on success. The caller must free *out_json using maplestory_free_string.
 */
int32_t maplestory_get_account_summary(
    ClientHandle* handle,
    const char* character_name,
    const char* date,
    char** out_json
);

/**
 * Render an animated GIF directly by character name.
 * Returns 0 on success. The caller must free *out_data using maplestory_free_bytes.
 */
int32_t maplestory_render_character_gif(
    ClientHandle* handle,
    const char* character_name,
    const char* preset,
    uint32_t width,
    uint32_t height,
    uint8_t** out_data,
    size_t* out_len
);

/**
 * Convert 1024-character AvatarCode to a dynamic Look URL.
 * Returns 0 on success. The caller must free *out_url using maplestory_free_string.
 */
int32_t maplestory_avatar_to_look_url(
    const char* code_or_url,
    const char* action,
    const char* emotion,
    const char* wmotion,
    uint32_t width,
    uint32_t height,
    char** out_url
);

/**
 * Free a string allocated by this library.
 */
void maplestory_free_string(char* ptr);

/**
 * Free a byte buffer allocated by this library.
 */
void maplestory_free_bytes(uint8_t* ptr, size_t len);

#ifdef __cplusplus
}
#endif

#endif /* MAPLESTORY_OPENAPI_H */

