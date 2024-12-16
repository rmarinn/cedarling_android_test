## Pre-requisite  for set-up:
1. Refer: https://forgen.tech/en/blog/post/building-an-android-app-with-rust-using-uniffi#step-3-building-libraries-for-android
2. Copy the project in `jans-cedarling/bindings/`
## How to Run  
1. cargo build
2. cargo ndk -o bindings/cedarling_android/rustandroid/app/src/main/jniLibs --manifest-path ./Cargo.toml -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 build --release
3. cargo run --bin uniffi-bindgen generate --library ./target/debug/libmobile.dylib --language kotlin --out-dir bindings/cerdarling_android/rustandroid/app/src/main/java/com/example/rust_android
4. Make changes in com.example.rust_android.MainActivity to import and use cedarling.