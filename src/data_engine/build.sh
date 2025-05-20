#!/bin/bash

TARGET_HOST=$(rustc -vV | grep 'host' |  cut -c 7-)

RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none" cargo +nightly build \
  -Z build-std=std,panic_abort \
  -Z build-std-features="optimize_for_size" \
  --target $TARGET_HOST --release

cp ~/CyberCraft/src/data_engine/target/$TARGET_HOST/release/data_engine  ~/CyberCraft/src/data_engine/target/release/data_engine
upx --best --lzma ~/CyberCraft/src/data_engine/target/$TARGET_HOST/release/data_engine
cp ~/CyberCraft/src/data_engine/target/$TARGET_HOST/release/data_engine ~/CyberCraft/src/bin/data_engine_smal

chmod +x ~/CyberCraft/src/bin/data_engine_smal

