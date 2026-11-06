@echo off
setlocal DisableDelayedExpansion
title SunSet Tools
cd /d "%~dp0"
if exist "%~dp0scripts\tools\aurora-tools.ps1" goto repo
if exist "%~dp0SunSet-Viewer\scripts\tools\aurora-tools.ps1" goto project
powershell -NoProfile -ExecutionPolicy Bypass -Command "$f = Join-Path $env:TEMP 'sunset-setup.ps1'; irm https://raw.githubusercontent.com/Adriano-Severino/SunSet-Viewer/main/scripts/setup.ps1 -OutFile $f; & $f"
set "SunsetToolsExitCode=%errorlevel%"
pause
exit /b %SunsetToolsExitCode%
:repo
set "SunsetToolsScript=%~dp0scripts\tools\aurora-tools.ps1"
goto launch
:project
set "SunsetToolsScript=%~dp0SunSet-Viewer\scripts\tools\aurora-tools.ps1"
:launch
powershell -NoProfile -ExecutionPolicy Bypass -File "%SunsetToolsScript%" %*
exit /b %errorlevel%
