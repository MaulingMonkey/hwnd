@setlocal && pushd "%~dp0.."
cargo run --manifest-path=crates/xtask/Cargo.toml --bin gen
@if ERRORLEVEL 1 goto :end
cargo test
@if ERRORLEVEL 1 goto :end
cargo doc
@if ERRORLEVEL 1 goto :end
:end
@endlocal && popd && exit /b %ERRORLEVEL%
