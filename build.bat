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

echo Building Blender File Finder...
echo.

REM Build the project in release mode
cargo build --release

if %errorlevel% equ 0 (
    echo.
    echo ========================================
    echo Build successful!
    echo.
    echo Executable location:
    echo %CD%\target\release\blender_file_finder.exe
    echo ========================================
    echo.
    echo Press any key to run the application...
    pause >nul

    REM Run the application
    start "" "%CD%\target\release\blender_file_finder.exe"
) else (
    echo.
    echo ========================================
    echo Build failed! Please check the errors above.
    echo ========================================
    pause
    exit /b 1
)
