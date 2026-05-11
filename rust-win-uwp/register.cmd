@echo off
setlocal

cargo build || exit /b 1

set TARGET=target\debug

if not exist "%TARGET%\foo_bar.exe" (
    echo ERROR: %TARGET%\foo_bar.exe is missing.
    exit /b 1
)

copy /Y appx\* "%TARGET%" >nul || exit /b 1

REM Detect processor architecture and patch the manifest so MSIX matches the binary.
set MSIX_ARCH=neutral
if /I "%PROCESSOR_ARCHITECTURE%"=="AMD64"  set MSIX_ARCH=x64
if /I "%PROCESSOR_ARCHITECTURE%"=="ARM64"  set MSIX_ARCH=arm64
if /I "%PROCESSOR_ARCHITECTURE%"=="x86"    set MSIX_ARCH=x86
powershell -NoProfile -ExecutionPolicy Bypass -command ^
  "(Get-Content '%TARGET%\AppxManifest.xml') -replace 'Version=\"1.0.0.0\"', 'Version=\"1.0.0.0\" ProcessorArchitecture=\"%MSIX_ARCH%\"' | Set-Content '%TARGET%\AppxManifest.xml'" || exit /b 1

REM Optional: capture crash dumps for foo_bar.exe under %LOCALAPPDATA%\CrashDumps\.
REM Requires Administrator. Skipped silently when not elevated.
net session >nul 2>&1
if not errorlevel 1 (
    reg add "HKLM\SOFTWARE\Microsoft\Windows\Windows Error Reporting\LocalDumps\foo_bar.exe" /v DumpType   /t REG_DWORD     /d 2 /f >nul
    reg add "HKLM\SOFTWARE\Microsoft\Windows\Windows Error Reporting\LocalDumps\foo_bar.exe" /v DumpCount  /t REG_DWORD     /d 5 /f >nul
    reg add "HKLM\SOFTWARE\Microsoft\Windows\Windows Error Reporting\LocalDumps\foo_bar.exe" /v DumpFolder /t REG_EXPAND_SZ /d "%%LOCALAPPDATA%%\CrashDumps" /f >nul
)

pushd "%TARGET%"
powershell -NoProfile -ExecutionPolicy Bypass -command "Get-AppxPackage *a3f8c2e1-9b4d-4e7a-b1c2-7d6e5f4a3b21* | Remove-AppxPackage" >nul 2>&1
powershell -NoProfile -ExecutionPolicy Bypass -command "Add-AppxPackage -Register AppxManifest.xml" || (popd & exit /b 1)
popd

echo.
echo Registered. Launch "Foo Bar Desktop" from the Start menu.
