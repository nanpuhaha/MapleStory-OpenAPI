import Foundation

/// High-performance Swift client for NEXON MapleStory OpenAPI, powered by Rust C-ABI FFI.
public final class MapleStoryClient {
    private var handle: OpaquePointer?

    public init?(apiKey: String) {
        guard let h = apiKey.withCString({ maplestory_client_new($0) }) else {
            return nil
        }
        self.handle = h
    }

    public static func fromEnv() -> MapleStoryClient? {
        guard let h = maplestory_client_from_env() else {
            return nil
        }
        let client = MapleStoryClient(rawHandle: h)
        return client
    }

    private init(rawHandle: OpaquePointer) {
        self.handle = rawHandle
    }

    deinit {
        if let h = handle {
            maplestory_client_free(h)
        }
    }

    /// Asynchronously retrieves character dashboard JSON string.
    public func getCharacterDashboard(characterName: String) async throws -> String {
        guard let h = handle else {
            throw MapleStoryError.clientClosed
        }

        return try await Task.detached {
            var outJson: UnsafeMutablePointer<CChar>?
            let code = characterName.withCString { cName in
                maplestory_get_character_dashboard(h, cName, &outJson)
            }

            guard code == 0, let jsonPtr = outJson else {
                throw MapleStoryError.apiError(code: code)
            }

            defer { maplestory_free_string(jsonPtr) }
            return String(cString: jsonPtr)
        }.value
    }

    /// Asynchronously renders animated GIF for character motion.
    /// Presets: "walk", "wink", "stand", "jump", "sit", "prone", "fly", "blink"
    public func renderCharacterGif(
        characterName: String,
        preset: String = "walk",
        width: UInt32 = 128,
        height: UInt32 = 128
    ) async throws -> Data {
        guard let h = handle else {
            throw MapleStoryError.clientClosed
        }

        return try await Task.detached {
            var outData: UnsafeMutablePointer<UInt8>?
            var outLen: Int = 0

            let code = characterName.withCString { cName in
                preset.withCString { cPreset in
                    maplestory_render_character_gif(h, cName, cPreset, width, height, &outData, &outLen)
                }
            }

            guard code == 0, let dataPtr = outData else {
                throw MapleStoryError.apiError(code: code)
            }

            defer { maplestory_free_bytes(dataPtr, outLen) }
            return Data(bytes: dataPtr, count: outLen)
        }.value
    }
}

public enum MapleStoryError: Error {
    case clientClosed
    case apiError(code: Int32)
}
