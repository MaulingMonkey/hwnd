@setlocal && pushd "%~dp0.."
cargo clean
:end
@endlocal && popd && exit /b %ERRORLEVEL%
