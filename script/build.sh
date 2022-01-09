#!/usr/bin/env bash

platforms="$(cat << EOL
  [
    { "target": "x86_64-apple-darwin", "name": "darwin-amd64" },
    { "target": "aarch64-apple-darwin", "name": "darwin-arm64" },
  ]
EOL
)"
platforms_end_index="$(echo "${platforms}" | jq '. | length - 1')"

for i in $(seq 0 "${platforms_end_index}"); do
    target=$(echo "${platforms}" | jq -r ".[${i}].target")
    name=$(echo "${platforms}" | jq -r ".[${i}].name")

    cargo build --release --target $target
    mv "target/${target}/release/gh-prj" "./dist/${name}"
done
