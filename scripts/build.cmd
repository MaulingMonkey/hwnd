@setlocal && pushd "%~dp0.."
cargo run --manifest-path=crates/xtask/Cargo.toml --bin gen
@if ERRORLEVEL 1 goto :end
cargo test --all-features
@if ERRORLEVEL 1 goto :end
cargo doc --all-features
@if ERRORLEVEL 1 goto :end
cargo build
@if ERRORLEVEL 1 goto :end
cargo test --all-features --target=i686-pc-windows-msvc
@if ERRORLEVEL 1 goto :end
:end
@endlocal && popd && exit /b %ERRORLEVEL%
