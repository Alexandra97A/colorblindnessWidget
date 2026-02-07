@echo off
REM ============================================
REM Color Blindness Widget Uninstaller
REM ============================================

echo.
echo ========================================
echo  Color Blindness Widget Uninstaller
echo ========================================
echo.

set "APP_NAME=Color Blindness Checker"

echo This will remove shortcuts and start menu entries.
echo The executable file will NOT be deleted.
echo.
echo Continue with uninstallation?
choice /C YN /M "Uninstall shortcuts"
if %errorlevel% neq 1 (
    echo.
    echo Uninstallation cancelled.
    pause
    exit /b 0
)

echo.
echo Removing shortcuts...
echo.

REM Remove Desktop shortcut
echo [1/4] Removing desktop shortcut...
if exist "%USERPROFILE%\Desktop\%APP_NAME%.lnk" (
    del "%USERPROFILE%\Desktop\%APP_NAME%.lnk"
    echo    SUCCESS: Desktop shortcut removed
) else (
    echo    INFO: Desktop shortcut not found
)
echo.

REM Remove Start Menu entry
echo [2/4] Removing Start Menu entry...
if exist "%APPDATA%\Microsoft\Windows\Start Menu\Programs\Accessibility\%APP_NAME%.lnk" (
    del "%APPDATA%\Microsoft\Windows\Start Menu\Programs\Accessibility\%APP_NAME%.lnk"
    echo    SUCCESS: Start Menu entry removed
) else (
    echo    INFO: Start Menu entry not found
)
echo.

REM Remove Startup entry
echo [3/4] Removing from Startup...
if exist "%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\%APP_NAME%.lnk" (
    del "%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\%APP_NAME%.lnk"
    echo    SUCCESS: Removed from Startup
) else (
    echo    INFO: Startup entry not found
)
echo.

REM Taskbar note
echo [4/4] Taskbar icon...
echo    NOTE: If pinned, please unpin manually:
echo    Right-click the taskbar icon ^> Unpin from taskbar
echo.

echo ========================================
echo  Uninstallation Complete!
echo ========================================
echo.
echo All shortcuts have been removed.
echo.
echo Note: The executable file was not deleted.
echo You can manually delete the entire folder if desired.
echo.
echo Would you like to delete the executable file now?
choice /C YN /M "Delete colorblind-widget.exe"
if %errorlevel% equ 1 (
    set "SCRIPT_DIR=%~dp0"
    if exist "%SCRIPT_DIR%colorblind-widget.exe" (
        del "%SCRIPT_DIR%colorblind-widget.exe"
        echo    Executable deleted.
    )
    echo.
    echo You can now delete this entire folder.
)

echo.
echo Thank you for using Color Blindness Widget!
echo.
pause
