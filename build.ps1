$ErrorActionPreference = "Stop"

if (-not (Get-Command wasm-pack -ErrorAction SilentlyContinue)) {
    Write-Host "wasm-pack fehlt. Installation:" -ForegroundColor Yellow
    Write-Host "  cargo install wasm-pack"
    exit 1
}

Write-Host "== viz nach wasm ==" -ForegroundColor Cyan
wasm-pack build viz --release --target web --out-dir ../dist/wasm --out-name viz
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Remove-Item -Force -ErrorAction SilentlyContinue `
    dist\wasm\.gitignore, dist\wasm\package.json, dist\wasm\README.md

Write-Host "== seiten ==" -ForegroundColor Cyan
cargo run --release -p site -- build
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host ""
Write-Host "fertig. vorschau: cargo run --release -p site -- serve" -ForegroundColor Green
