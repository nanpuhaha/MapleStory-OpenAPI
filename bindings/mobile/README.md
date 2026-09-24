# maplestory-openapi (Mobile: iOS & Android)

Native bindings and bridge implementation for Mobile platforms (iOS Swift & Android Kotlin), powered by Rust C-ABI FFI.

## 📱 iOS (Swift)
- Compatible with SwiftUI, UIKit, macOS, and iOS 13+
- Uses Swift modern Concurrency (`async/await`)
- Returns standard `Foundation.Data` for GIF rendering directly into `WKWebView` or `SDWebImage`

### Building for iOS
```bash
# Cross-compile for iOS devices and Simulator
cargo build --target aarch64-apple-ios --release
cargo build --target aarch64-apple-ios-sim --release

# Package as XCFramework
xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/libmaplestory_openapi_c.a -headers ../c/include \
  -library target/aarch64-apple-ios-sim/release/libmaplestory_openapi_c.a -headers ../c/include \
  -output MapleStoryOpenApi.xcframework
```

---

## 🤖 Android (Kotlin)
- Compatible with Jetpack Compose, Kotlin Coroutines (`suspend fun`), and Android API 21+
- Uses JNI bridge over `libmaplestory_openapi_c.so`

### Building for Android (NDK)
```bash
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -o app/src/main/jniLibs build --release
```
