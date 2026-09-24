using System;
using System.IO;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace MapleStory.OpenApi
{
    /// <summary>
    /// High-performance C# client for NEXON MapleStory OpenAPI, powered by Rust.
    /// </summary>
    public sealed class MapleStoryClient : IDisposable
    {
        private const string LibName = "maplestory_openapi_c";

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr maplestory_client_new([MarshalAs(UnmanagedType.LPUTF8Str)] string apiKey);

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
                throw new InvalidOperationException("Failed to initialize MapleStoryClient handle");
        }

        private MapleStoryClient(IntPtr handle)
        {
            _handle = handle;
        }

        public static MapleStoryClient FromEnv()
        {
            var handle = maplestory_client_from_env();
            if (handle == IntPtr.Zero)
                throw new InvalidOperationException("NEXON_API_KEY environment variable not set or failed to initialize");
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

            return Task.Run(() =>
            {
                int code = maplestory_get_character_dashboard(_handle, characterName, out IntPtr outJson);
                if (code != 0 || outJson == IntPtr.Zero)
                    throw new InvalidOperationException($"Failed to get character dashboard (Error code: {code})");

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
                    throw new InvalidOperationException($"Failed to render character GIF (Error code: {code})");

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
