@setlocal && pushd "%~dp0.."
cargo run --manifest-path=crates/xtask/Cargo.toml --bin gen
@if ERRORLEVEL 1 goto :end
cargo doc --all-features --open
@if ERRORLEVEL 1 goto :end
:end
@endlocal && popd && exit /b %ERRORLEVEL%
