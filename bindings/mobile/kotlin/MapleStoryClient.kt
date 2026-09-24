package com.maplestory.openapi

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.io.Closeable

/**
 * High-performance Kotlin client for NEXON MapleStory OpenAPI, powered by Rust via JNI/C-FFI.
 */
class MapleStoryClient private constructor(private var handle: Long) : Closeable {

    companion object {
        init {
            System.loadLibrary("maplestory_openapi_c")
        }

        @JvmStatic
        external fun nativeClientNew(apiKey: String): Long

        @JvmStatic
        external fun nativeClientFromEnv(): Long

        @JvmStatic
        external fun nativeClientFree(handle: Long)

        @JvmStatic
        external fun nativeGetCharacterDashboard(handle: Long, name: String): String?

        @JvmStatic
        external fun nativeRenderCharacterGif(
            handle: Long,
            name: String,
            preset: String,
            width: Int,
            height: Int
        ): ByteArray?

        fun create(apiKey: String): MapleStoryClient {
            val h = nativeClientNew(apiKey)
            if (h == 0L) throw IllegalStateException("Failed to initialize MapleStoryClient")
            return MapleStoryClient(h)
        }

        fun fromEnv(): MapleStoryClient {
            val h = nativeClientFromEnv()
            if (h == 0L) throw IllegalStateException("NEXON_API_KEY environment variable not set")
            return MapleStoryClient(h)
        }
    }

    suspend fun getCharacterDashboard(characterName: String): String = withContext(Dispatchers.IO) {
        checkHandle()
        nativeGetCharacterDashboard(handle, characterName)
            ?: throw RuntimeException("Failed to get character dashboard")
    }

    suspend fun renderCharacterGif(
        characterName: String,
        preset: String = "walk",
        width: Int = 128,
        height: Int = 128
    ): ByteArray = withContext(Dispatchers.IO) {
        checkHandle()
        nativeRenderCharacterGif(handle, characterName, preset, width, height)
            ?: throw RuntimeException("Failed to render character GIF")
    }

    private fun checkHandle() {
        if (handle == 0L) throw IllegalStateException("Client is closed")
    }

    override fun close() {
        if (handle != 0L) {
            nativeClientFree(handle)
            handle = 0L
        }
    }
}
