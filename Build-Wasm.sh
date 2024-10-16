# runs in WSL2, copies the resulting .a file to Windows
EMCC_CFLAGS="-g -s ERROR_ON_UNDEFINED_SYMBOLS=0 --no-entry -s FULL_ES3=1"
cargo +nightly build --release --target wasm32-unknown-emscripten
cp target/wasm32-unknown-emscripten/release/libprimrose_rust.a /mnt/c/Primrose/PrimroseEngine/Libraries