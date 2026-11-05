@echo off
setlocal DisableDelayedExpansion
title SunSet Tools
cd /d "%~dp0"
if exist "%~dp0scripts\tools\aurora-tools.ps1" goto repo
if exist "%~dp0aurora-viewer\scripts\tools\aurora-tools.ps1" goto project
powershell -NoProfile -ExecutionPolicy Bypass -Command "$f = Join-Path $env:TEMP 'sunset-setup.ps1'; irm https://raw.githubusercontent.com/Adriano-Severino/second-life-projeto/main/SunSet-Viewer/scripts/setup.ps1 -OutFile $f; & $f"
set "AuroraToolsExitCode=%errorlevel%"
pause
exit /b %AuroraToolsExitCode%
:repo
set "AuroraToolsScript=%~dp0scripts\tools\aurora-tools.ps1"
goto launch
:project
set "AuroraToolsScript=%~dp0aurora-viewer\scripts\tools\aurora-tools.ps1"
:launch
powershell -NoProfile -ExecutionPolicy Bypass -File "%AuroraToolsScript%" %*
exit /b %errorlevel%
