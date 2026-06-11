@echo off
set "ProjectDir=%~dp0"
cd /d "%ProjectDir%"

if not exist "oled-wallpaper-automator.exe" (
    echo oled-wallpaper-automator.exe not found!
    echo Downloading precompiled binary from GitHub...
    powershell -ExecutionPolicy Bypass -Command "[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; Invoke-WebRequest -Uri 'https://github.com/scuba3198/oled-wallpaper-automator/releases/latest/download/oled-wallpaper-automator.exe' -OutFile 'oled-wallpaper-automator.exe'"
)

if not exist "oled-wallpaper-automator.exe" (
    echo.
    echo [ERROR] Failed to download oled-wallpaper-automator.exe.
    echo Please make sure you are connected to the internet and try again.
    echo Or download it manually from:
    echo https://github.com/scuba3198/oled-wallpaper-automator/releases
    echo.
    pause
    exit /b 1
)

echo Selecting a fresh wallpaper...
oled-wallpaper-automator.exe
pause

