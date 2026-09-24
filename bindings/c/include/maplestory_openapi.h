#ifndef MAPLESTORY_OPENAPI_H
#define MAPLESTORY_OPENAPI_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ClientHandle ClientHandle;

/**
 * Create a new MapleStory client handle with an API key.
 * Returns NULL on failure.
 */
ClientHandle* maplestory_client_new(const char* api_key);

/**
 * Create a new MapleStory client handle from NEXON_API_KEY environment variable.
 */
ClientHandle* maplestory_client_from_env(void);

/**
 * Free a MapleStory client handle.
 */
void maplestory_client_free(ClientHandle* handle);

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
