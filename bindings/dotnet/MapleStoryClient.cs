using System;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace MapleStory.OpenAPI
{
    /// <summary>
    /// High-performance C# client for NEXON MapleStory OpenAPI, powered by Rust.
    /// </summary>
    public sealed class MapleStoryClient : IDisposable
    {
        private const string LibName = "maplestory_openapi_c";

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr maplestory_last_error_message();

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr maplestory_client_new([MarshalAs(UnmanagedType.LPUTF8Str)] string apiKey);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr maplestory_client_with_base_url(
            [MarshalAs(UnmanagedType.LPUTF8Str)] string apiKey,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string baseUrl
        );

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr maplestory_client_from_env();

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        private static extern void maplestory_client_free(IntPtr handle);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        private static extern int maplestory_get_character_dashboard(
            IntPtr handle,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string characterName,
            out IntPtr outJson
        );

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        private static extern int maplestory_get_guild_profile(
            IntPtr handle,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string guildName,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string worldName,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string? date,
            out IntPtr outJson
        );

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        private static extern int maplestory_get_user_union(
            IntPtr handle,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string characterName,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string? date,
            out IntPtr outJson
        );

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        private static extern int maplestory_get_overall_ranking(
            IntPtr handle,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string date,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string? worldName,
            int worldType,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string? characterClass,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string? ocid,
            int page,
            out IntPtr outJson
        );

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        private static extern int maplestory_get_notice_list(
            IntPtr handle,
            out IntPtr outJson
        );

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        private static extern int maplestory_get_account_summary(
            IntPtr handle,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string characterName,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string? date,
            out IntPtr outJson
        );

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        private static extern int maplestory_render_character_gif(
            IntPtr handle,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string characterName,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string preset,
            uint width,
            uint height,
            out IntPtr outData,
            out UIntPtr outLen
        );

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        private static extern void maplestory_free_string(IntPtr ptr);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        private static extern void maplestory_free_bytes(IntPtr ptr, UIntPtr len);

        private IntPtr _handle;
        private bool _disposed;

        public MapleStoryClient(string apiKey)
        {
            if (string.IsNullOrWhiteSpace(apiKey))
                throw new ArgumentException("API key cannot be null or empty", nameof(apiKey));

            _handle = maplestory_client_new(apiKey);
            if (_handle == IntPtr.Zero)
                throw new InvalidOperationException(GetLastError("Failed to initialize MapleStoryClient handle"));
        }

        public MapleStoryClient(string apiKey, string baseUrl)
        {
            if (string.IsNullOrWhiteSpace(apiKey))
                throw new ArgumentException("API key cannot be null or empty", nameof(apiKey));
            if (string.IsNullOrWhiteSpace(baseUrl))
                throw new ArgumentException("Base URL cannot be null or empty", nameof(baseUrl));

            _handle = maplestory_client_with_base_url(apiKey, baseUrl);
            if (_handle == IntPtr.Zero)
                throw new InvalidOperationException(GetLastError("Failed to initialize MapleStoryClient handle"));
        }

        private MapleStoryClient(IntPtr handle)
        {
            _handle = handle;
        }

        public static MapleStoryClient FromEnv()
        {
            var handle = maplestory_client_from_env();
            if (handle == IntPtr.Zero)
                throw new InvalidOperationException(GetLastError("NEXON_API_KEY environment variable not set or failed to initialize"));
            return new MapleStoryClient(handle);
        }

        /// <summary>
        /// Asynchronously retrieves the comprehensive all-in-one character dashboard JSON string.
        /// </summary>
        public Task<string> GetCharacterDashboardAsync(string characterName)
        {
            CheckDisposed();
            if (string.IsNullOrWhiteSpace(characterName))
                throw new ArgumentException("Character name cannot be null or empty", nameof(characterName));

            return InvokeJsonAsync((out IntPtr outJson) =>
                maplestory_get_character_dashboard(_handle, characterName, out outJson),
                "Failed to get character dashboard"
            );
        }

        /// <summary>
        /// Asynchronously retrieves the guild profile (guild basic + member profiles) JSON string.
        /// </summary>
        public Task<string> GetGuildProfileAsync(string guildName, string worldName, string? date = null)
        {
            CheckDisposed();
            if (string.IsNullOrWhiteSpace(guildName))
                throw new ArgumentException("Guild name cannot be null or empty", nameof(guildName));
            if (string.IsNullOrWhiteSpace(worldName))
                throw new ArgumentException("World name cannot be null or empty", nameof(worldName));

            return InvokeJsonAsync((out IntPtr outJson) =>
                maplestory_get_guild_profile(_handle, guildName, worldName, date, out outJson),
                "Failed to get guild profile"
            );
        }

        /// <summary>
        /// Asynchronously retrieves user union level/grade information by character name as a JSON string.
        /// </summary>
        public Task<string> GetUserUnionAsync(string characterName, string? date = null)
        {
            CheckDisposed();
            if (string.IsNullOrWhiteSpace(characterName))
                throw new ArgumentException("Character name cannot be null or empty", nameof(characterName));

            return InvokeJsonAsync((out IntPtr outJson) =>
                maplestory_get_user_union(_handle, characterName, date, out outJson),
                "Failed to get user union"
            );
        }

        /// <summary>
        /// Asynchronously retrieves overall ranking information as a JSON string.
        /// </summary>
        public Task<string> GetOverallRankingAsync(
            string date,
            string? worldName = null,
            int worldType = -1,
            string? characterClass = null,
            string? ocid = null,
            int page = 0
        )
        {
            CheckDisposed();
            if (string.IsNullOrWhiteSpace(date))
                throw new ArgumentException("Date cannot be null or empty", nameof(date));

            return InvokeJsonAsync((out IntPtr outJson) =>
                maplestory_get_overall_ranking(_handle, date, worldName, worldType, characterClass, ocid, page, out outJson),
                "Failed to get overall ranking"
            );
        }

        /// <summary>
        /// Asynchronously retrieves the latest official notices list as a JSON string.
        /// </summary>
        public Task<string> GetNoticeListAsync()
        {
            CheckDisposed();
            return InvokeJsonAsync((out IntPtr outJson) =>
                maplestory_get_notice_list(_handle, out outJson),
                "Failed to get notice list"
            );
        }

        /// <summary>
        /// Asynchronously retrieves the account summary (union + artifact + champion + achievements) as a JSON string.
        /// </summary>
        public Task<string> GetAccountSummaryAsync(string characterName, string? date = null)
        {
            CheckDisposed();
            if (string.IsNullOrWhiteSpace(characterName))
                throw new ArgumentException("Character name cannot be null or empty", nameof(characterName));

            return InvokeJsonAsync((out IntPtr outJson) =>
                maplestory_get_account_summary(_handle, characterName, date, out outJson),
                "Failed to get account summary"
            );
        }

        /// <summary>
        /// Asynchronously renders an animated GIF for the specified character motion.
        /// Presets: "walk", "wink", "stand", "jump", "sit", "prone", "fly", "blink"
        /// </summary>
        public Task<byte[]> RenderCharacterGifAsync(
            string characterName,
            string preset = "walk",
            uint width = 128,
            uint height = 128
        )
        {
            CheckDisposed();
            if (string.IsNullOrWhiteSpace(characterName))
                throw new ArgumentException("Character name cannot be null or empty", nameof(characterName));

            return Task.Run(() =>
            {
                int code = maplestory_render_character_gif(
                    _handle,
                    characterName,
                    preset,
                    width,
                    height,
                    out IntPtr outData,
                    out UIntPtr outLen
                );

                if (code != 0 || outData == IntPtr.Zero)
                    throw new InvalidOperationException(GetLastError($"Failed to render character GIF (Error code: {code})"));

                try
                {
                    int length = (int)outLen.ToUInt32();
                    byte[] buffer = new byte[length];
                    Marshal.Copy(outData, buffer, 0, length);
                    return buffer;
                }
                finally
                {
                    maplestory_free_bytes(outData, outLen);
                }
            });
        }

        private delegate int NativeJsonCall(out IntPtr outJson);

        private static Task<string> InvokeJsonAsync(NativeJsonCall call, string fallbackError)
        {
            return Task.Run(() =>
            {
                int code = call(out IntPtr outJson);
                if (code != 0 || outJson == IntPtr.Zero)
                    throw new InvalidOperationException(GetLastError($"{fallbackError} (Error code: {code})"));

                try
                {
                    return Utf8StringFromNative(outJson);
                }
                finally
                {
                    maplestory_free_string(outJson);
                }
            });
        }

        private static string GetLastError(string fallback)
        {
            IntPtr errPtr = maplestory_last_error_message();
            if (errPtr == IntPtr.Zero)
                return fallback;

            try
            {
                string detail = Utf8StringFromNative(errPtr);
                return string.IsNullOrWhiteSpace(detail) ? fallback : $"{fallback}: {detail}";
            }
            finally
            {
                maplestory_free_string(errPtr);
            }
        }

        private static string Utf8StringFromNative(IntPtr ptr)
        {
            int len = 0;
            while (Marshal.ReadByte(ptr, len) != 0) len++;
            byte[] bytes = new byte[len];
            Marshal.Copy(ptr, bytes, 0, len);
            return Encoding.UTF8.GetString(bytes);
        }

        private void CheckDisposed()
        {
            if (_disposed || _handle == IntPtr.Zero)
                throw new ObjectDisposedException(nameof(MapleStoryClient));
        }

        public void Dispose()
        {
            if (!_disposed)
            {
                if (_handle != IntPtr.Zero)
                {
                    maplestory_client_free(_handle);
                    _handle = IntPtr.Zero;
                }
                _disposed = true;
                GC.SuppressFinalize(this);
            }
        }

        ~MapleStoryClient()
        {
            Dispose();
        }
    }
}

