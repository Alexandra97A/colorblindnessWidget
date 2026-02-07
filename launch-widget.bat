@echo off
REM Color Blindness Widget Launcher
REM This script launches the Color Blindness Checker application

cd /d "%~dp0"

if exist "target\release\colorblind-widget.exe" (
    start "" "target\release\colorblind-widget.exe"
) else (
    echo Error: Application not found!
    echo Please build the project first using: cargo build --release
    pause
)
