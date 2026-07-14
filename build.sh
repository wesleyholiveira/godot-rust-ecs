#!/usr/bin/env sh
set -eu
cd "$(dirname "$0")/rust"
cargo build
printf '\nBuild concluído. Abra a pasta godot/ no Godot 4.6+ e execute o projeto.\n'
