@echo off

svdtools patch svd_patch.yaml HK32F0301MxxC_patched.svd

if %ERRORLEVEL% neq 0 goto EXIT

echo SVD Patched...

svd2rust -c svd2rust.toml -i ./HK32F0301MxxC_patched.svd

rmdir /S /Q src
del /Q HK32F0301MxxC_patched.svd

form -i lib.rs -o src/ && rm lib.rs

cargo fmt

echo PAC Regenerated...

:EXIT
echo.
pause