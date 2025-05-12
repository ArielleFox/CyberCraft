#!/bin/bash

RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build --release
