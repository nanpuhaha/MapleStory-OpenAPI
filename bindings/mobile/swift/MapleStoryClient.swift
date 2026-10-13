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

    public init?(apiKey: String, baseUrl: String) {
        guard let h = apiKey.withCString({ cKey in
            baseUrl.withCString({ cUrl in
                maplestory_client_with_base_url(cKey, cUrl)
            })
        }) else {
            return nil
        }
        self.handle = h
    }

    public static func fromEnv() -> MapleStoryClient? {
        guard let h = maplestory_client_from_env() else {
            return nil
        }
        return MapleStoryClient(rawHandle: h)
    }

    private init(rawHandle: OpaquePointer) {
        self.handle = rawHandle
    }

    deinit {
        if let h = handle {
            maplestory_client_free(h)
        }
    }

    private static func takeLastError() -> String? {
        guard let errPtr = maplestory_last_error_message() else {
            return nil
        }
        defer { maplestory_free_string(errPtr) }
        return String(cString: errPtr)
    }

    private static func unwrapJsonResult(code: Int32, outJson: UnsafeMutablePointer<CChar>?) throws -> String {
        guard code == 0, let jsonPtr = outJson else {
            throw MapleStoryError.apiError(code: code, message: takeLastError())
        }
        defer { maplestory_free_string(jsonPtr) }
        return String(cString: jsonPtr)
    }

    private static func withOptionalCString<R>(_ value: String?, _ body: (UnsafePointer<CChar>?) -> R) -> R {
        guard let value = value, !value.isEmpty else {
            return body(nil)
        }
        return value.withCString { body($0) }
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
            return try Self.unwrapJsonResult(code: code, outJson: outJson)
        }.value
    }

    /// Asynchronously retrieves guild profile (guild basic + member profiles) JSON string.
    public func getGuildProfile(guildName: String, worldName: String, date: String? = nil) async throws -> String {
        guard let h = handle else {
            throw MapleStoryError.clientClosed
        }

        return try await Task.detached {
            var outJson: UnsafeMutablePointer<CChar>?
            let code = guildName.withCString { cGuild in
                worldName.withCString { cWorld in
                    Self.withOptionalCString(date) { cDate in
                        maplestory_get_guild_profile(h, cGuild, cWorld, cDate, &outJson)
                    }
                }
            }
            return try Self.unwrapJsonResult(code: code, outJson: outJson)
        }.value
    }

    /// Asynchronously retrieves user union level/grade information by character name as a JSON string.
    public func getUserUnion(characterName: String, date: String? = nil) async throws -> String {
        guard let h = handle else {
            throw MapleStoryError.clientClosed
        }

        return try await Task.detached {
            var outJson: UnsafeMutablePointer<CChar>?
            let code = characterName.withCString { cName in
                Self.withOptionalCString(date) { cDate in
                    maplestory_get_user_union(h, cName, cDate, &outJson)
                }
            }
            return try Self.unwrapJsonResult(code: code, outJson: outJson)
        }.value
    }

    /// Asynchronously retrieves overall ranking information as a JSON string.
    public func getOverallRanking(
        date: String,
        worldName: String? = nil,
        worldType: Int32 = -1,
        characterClass: String? = nil,
        ocid: String? = nil,
        page: Int32 = 0
    ) async throws -> String {
        guard let h = handle else {
            throw MapleStoryError.clientClosed
        }

        return try await Task.detached {
            var outJson: UnsafeMutablePointer<CChar>?
            let code = date.withCString { cDate in
                Self.withOptionalCString(worldName) { cWorld in
                    Self.withOptionalCString(characterClass) { cClass in
                        Self.withOptionalCString(ocid) { cOcid in
                            maplestory_get_overall_ranking(h, cDate, cWorld, worldType, cClass, cOcid, page, &outJson)
                        }
                    }
                }
            }
            return try Self.unwrapJsonResult(code: code, outJson: outJson)
        }.value
    }

    /// Asynchronously retrieves the latest official notices list as a JSON string.
    public func getNoticeList() async throws -> String {
        guard let h = handle else {
            throw MapleStoryError.clientClosed
        }

        return try await Task.detached {
            var outJson: UnsafeMutablePointer<CChar>?
            let code = maplestory_get_notice_list(h, &outJson)
            return try Self.unwrapJsonResult(code: code, outJson: outJson)
        }.value
    }

    /// Asynchronously retrieves the account summary (union + artifact + champion + achievements) as a JSON string.
    public func getAccountSummary(characterName: String, date: String? = nil) async throws -> String {
        guard let h = handle else {
            throw MapleStoryError.clientClosed
        }

        return try await Task.detached {
            var outJson: UnsafeMutablePointer<CChar>?
            let code = characterName.withCString { cName in
                Self.withOptionalCString(date) { cDate in
                    maplestory_get_account_summary(h, cName, cDate, &outJson)
                }
            }
            return try Self.unwrapJsonResult(code: code, outJson: outJson)
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
                throw MapleStoryError.apiError(code: code, message: Self.takeLastError())
            }

            defer { maplestory_free_bytes(dataPtr, outLen) }
            return Data(bytes: dataPtr, count: outLen)
        }.value
    }
}

public enum MapleStoryError: Error {
    case clientClosed
    case apiError(code: Int32, message: String?)
}

