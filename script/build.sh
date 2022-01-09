#!/usr/bin/env bash

cargo build --release --target $TARGET
mv "target/${TARGET}/release/gh-prj" "./dist/${FILENAME}"
