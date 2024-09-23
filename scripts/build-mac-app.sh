APP_NAME="Rusty Lander"
RUST_CRATE_NAME="rusty-lander"
mkdir -p "${APP_NAME}.app/Contents/MacOS"
mkdir -p "${APP_NAME}.app/Contents/Resources"
cp Info.plist "${APP_NAME}.app/Contents/Info.plist"
cp AppIcon.icns "${APP_NAME}.app/Contents/Resources/AppIcon.icns"
cp -a assets "${APP_NAME}.app/Contents/MacOS/"
cargo build --release --target x86_64-apple-darwin # build for Intel
cargo build --release --target aarch64-apple-darwin # build for Apple Silicon
lipo "target/x86_64-apple-darwin/release/${RUST_CRATE_NAME}" "target/aarch64-apple-darwin/release/${RUST_CRATE_NAME}" -create -output "${APP_NAME}.app/Contents/MacOS/${APP_NAME}"
