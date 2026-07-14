$ErrorActionPreference = "Stop"
Set-Location "$PSScriptRoot/rust"
cargo build
Write-Host "`nBuild concluído. Abra a pasta godot/ no Godot 4.6+ e execute o projeto."
