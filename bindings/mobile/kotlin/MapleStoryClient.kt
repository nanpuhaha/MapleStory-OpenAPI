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
        external fun nativeLastErrorMessage(): String?

        @JvmStatic
        external fun nativeClientNew(apiKey: String): Long

        @JvmStatic
        external fun nativeClientWithBaseUrl(apiKey: String, baseUrl: String): Long

        @JvmStatic
        external fun nativeClientFromEnv(): Long

        @JvmStatic
        external fun nativeClientFree(handle: Long)

        @JvmStatic
        external fun nativeGetCharacterDashboard(handle: Long, name: String): String?

        @JvmStatic
        external fun nativeGetGuildProfile(handle: Long, guildName: String, worldName: String, date: String?): String?

        @JvmStatic
        external fun nativeGetUserUnion(handle: Long, characterName: String, date: String?): String?

        @JvmStatic
        external fun nativeGetOverallRanking(
            handle: Long,
            date: String,
            worldName: String?,
            worldType: Int,
            characterClass: String?,
            ocid: String?,
            page: Int
        ): String?

        @JvmStatic
        external fun nativeGetNoticeList(handle: Long): String?

        @JvmStatic
        external fun nativeGetAccountSummary(handle: Long, characterName: String, date: String?): String?

        @JvmStatic
        external fun nativeRenderCharacterGif(
            handle: Long,
            name: String,
            preset: String,
            width: Int,
            height: Int
        ): ByteArray?

        private fun formatError(fallback: String): String {
            val detail = nativeLastErrorMessage()
            return if (detail.isNullOrBlank()) fallback else "$fallback: $detail"
        }

        fun create(apiKey: String): MapleStoryClient {
            val h = nativeClientNew(apiKey)
            if (h == 0L) throw IllegalStateException(formatError("Failed to initialize MapleStoryClient"))
            return MapleStoryClient(h)
        }

        fun createWithBaseUrl(apiKey: String, baseUrl: String): MapleStoryClient {
            val h = nativeClientWithBaseUrl(apiKey, baseUrl)
            if (h == 0L) throw IllegalStateException(formatError("Failed to initialize MapleStoryClient"))
            return MapleStoryClient(h)
        }

        fun fromEnv(): MapleStoryClient {
            val h = nativeClientFromEnv()
            if (h == 0L) throw IllegalStateException(formatError("NEXON_API_KEY environment variable not set"))
            return MapleStoryClient(h)
        }
    }

    suspend fun getCharacterDashboard(characterName: String): String = withContext(Dispatchers.IO) {
        checkHandle()
        nativeGetCharacterDashboard(handle, characterName)
            ?: throw RuntimeException(formatError("Failed to get character dashboard"))
    }

    suspend fun getGuildProfile(guildName: String, worldName: String, date: String? = null): String =
        withContext(Dispatchers.IO) {
            checkHandle()
            nativeGetGuildProfile(handle, guildName, worldName, date)
                ?: throw RuntimeException(formatError("Failed to get guild profile"))
        }

    suspend fun getUserUnion(characterName: String, date: String? = null): String =
        withContext(Dispatchers.IO) {
            checkHandle()
            nativeGetUserUnion(handle, characterName, date)
                ?: throw RuntimeException(formatError("Failed to get user union"))
        }

    suspend fun getOverallRanking(
        date: String,
        worldName: String? = null,
        worldType: Int = -1,
        characterClass: String? = null,
        ocid: String? = null,
        page: Int = 0
    ): String = withContext(Dispatchers.IO) {
        checkHandle()
        nativeGetOverallRanking(handle, date, worldName, worldType, characterClass, ocid, page)
            ?: throw RuntimeException(formatError("Failed to get overall ranking"))
    }

    suspend fun getNoticeList(): String = withContext(Dispatchers.IO) {
        checkHandle()
        nativeGetNoticeList(handle)
            ?: throw RuntimeException(formatError("Failed to get notice list"))
    }

    suspend fun getAccountSummary(characterName: String, date: String? = null): String =
        withContext(Dispatchers.IO) {
            checkHandle()
            nativeGetAccountSummary(handle, characterName, date)
                ?: throw RuntimeException(formatError("Failed to get account summary"))
        }

    suspend fun renderCharacterGif(
        characterName: String,
        preset: String = "walk",
        width: Int = 128,
        height: Int = 128
    ): ByteArray = withContext(Dispatchers.IO) {
        checkHandle()
        nativeRenderCharacterGif(handle, characterName, preset, width, height)
            ?: throw RuntimeException(formatError("Failed to render character GIF"))
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

