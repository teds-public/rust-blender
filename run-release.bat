@echo off
setlocal

REM Add Rust to PATH
set PATH=%USERPROFILE%\.cargo\bin;%PATH%

REM Check if cargo is available
where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Cargo not found in PATH
    echo Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

echo Running Blender File Finder (release mode)...
echo.

REM Run the project in release mode
cargo run --release

if %errorlevel% neq 0 (
    echo.
    echo ========================================
    echo Run failed! Please check the errors above.
    echo ========================================
    pause
    exit /b 1
)
