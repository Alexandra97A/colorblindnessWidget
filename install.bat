@echo off
REM ============================================
REM Color Blindness Widget Installer
REM ============================================

echo.
echo ========================================
echo  Color Blindness Widget Installer
echo ========================================
echo.

REM Get the directory where the script is located
set "SCRIPT_DIR=%~dp0"
set "EXE_PATH=%SCRIPT_DIR%colorblind-widget.exe"
set "APP_NAME=Color Blindness Checker"

REM Check if executable exists
if not exist "%EXE_PATH%" (
    echo ERROR: colorblind-widget.exe not found!
    echo Please ensure the .exe file is in the same folder as this installer.
    echo.
    pause
    exit /b 1
)

echo Found: %EXE_PATH%
echo.

REM Create Desktop Shortcut
echo [1/4] Creating desktop shortcut...
powershell -Command "$WS = New-Object -ComObject WScript.Shell; $SC = $WS.CreateShortcut('%USERPROFILE%\Desktop\%APP_NAME%.lnk'); $SC.TargetPath = '%EXE_PATH%'; $SC.WorkingDirectory = '%SCRIPT_DIR%'; $SC.Description = 'Color blindness accessibility checker for designers'; $SC.Save()"
if %errorlevel% equ 0 (
    echo    SUCCESS: Desktop shortcut created!
) else (
    echo    WARNING: Could not create desktop shortcut
)
echo.

REM Create Start Menu entry
echo [2/4] Creating Start Menu entry...
if not exist "%APPDATA%\Microsoft\Windows\Start Menu\Programs\Accessibility" mkdir "%APPDATA%\Microsoft\Windows\Start Menu\Programs\Accessibility"
powershell -Command "$WS = New-Object -ComObject WScript.Shell; $SC = $WS.CreateShortcut('%APPDATA%\Microsoft\Windows\Start Menu\Programs\Accessibility\%APP_NAME%.lnk'); $SC.TargetPath = '%EXE_PATH%'; $SC.WorkingDirectory = '%SCRIPT_DIR%'; $SC.Description = 'Color blindness accessibility checker for designers'; $SC.Save()"
if %errorlevel% equ 0 (
    echo    SUCCESS: Start Menu entry created!
) else (
    echo    WARNING: Could not create Start Menu entry
)
echo.

REM Ask about taskbar pinning
echo [3/4] Pin to Taskbar?
echo.
echo Would you like to pin the widget to your taskbar for quick access?
echo (You can unpin it later by right-clicking the taskbar icon)
echo.
choice /C YN /M "Pin to taskbar"
if %errorlevel% equ 1 (
    echo.
    echo    Please pin manually:
    echo    1. Right-click the desktop shortcut
    echo    2. Select "Pin to taskbar"
    echo    (Or drag the desktop shortcut to your taskbar)
    echo.
    timeout /t 5
)
echo.

REM Ask about startup
echo [4/4] Start with Windows?
echo.
echo Would you like the widget to start automatically when Windows starts?
echo (Useful if you check colors frequently)
echo.
choice /C YN /M "Add to Startup"
if %errorlevel% equ 1 (
    echo    Adding to Startup...
    if not exist "%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup" mkdir "%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup"
    powershell -Command "$WS = New-Object -ComObject WScript.Shell; $SC = $WS.CreateShortcut('%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\%APP_NAME%.lnk'); $SC.TargetPath = '%EXE_PATH%'; $SC.WorkingDirectory = '%SCRIPT_DIR%'; $SC.Save()"
    if %errorlevel% equ 0 (
        echo    SUCCESS: Added to Startup folder!
    ) else (
        echo    WARNING: Could not add to Startup
    )
) else (
    echo    Skipped Startup installation
)
echo.

REM Success message
echo.
echo ========================================
echo  Installation Complete!
echo ========================================
echo.
echo You can now launch the widget from:
echo   - Desktop shortcut: "%APP_NAME%"
echo   - Start Menu: Accessibility ^> %APP_NAME%
echo   - Taskbar (if pinned)
echo.
echo To uninstall: Run uninstall.bat
echo.
echo Launch the widget now?
choice /C YN /M "Start application"
if %errorlevel% equ 1 (
    start "" "%EXE_PATH%"
)

echo.
echo Thank you for using Color Blindness Widget!
echo Help make the digital world more accessible! ^<3
echo.
pause
