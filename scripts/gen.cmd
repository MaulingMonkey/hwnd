@setlocal && pushd "%~dp0.."
cargo run --manifest-path=crates/xtask/Cargo.toml --bin gen
@endlocal && popd && exit /b %ERRORLEVEL%
