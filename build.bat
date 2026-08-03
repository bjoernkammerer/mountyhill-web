@echo off
setlocal

where wasm-pack >nul 2>nul
if errorlevel 1 (
  echo wasm-pack fehlt. Installation: cargo install wasm-pack
  exit /b 1
)

echo == viz nach wasm ==
wasm-pack build viz --release --target web --out-dir ../dist/wasm --out-name viz
if errorlevel 1 exit /b 1

del /q dist\wasm\.gitignore dist\wasm\package.json dist\wasm\README.md 2>nul

echo == seiten ==
cargo run --release -p site -- build
if errorlevel 1 exit /b 1

echo.
echo fertig. vorschau: cargo run --release -p site -- serve
