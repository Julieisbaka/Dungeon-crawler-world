@echo off
:: Find Visual Studio installation path
set "VSWHERE=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe"
for /f "usebackq delims=" %%i in (`"%VSWHERE%" -latest -property installationPath`) do (
  set "VS_PATH=%%i"
)

if not defined VS_PATH (
  echo Error: Visual Studio not found
  exit /b 1
)

:: Setup VS environment
call "%VS_PATH%\Common7\Tools\VsDevCmd.bat"

:: Add CMake to PATH if it exists in common locations
if exist "C:\Program Files\CMake\bin" set "PATH=C:\Program Files\CMake\bin;%PATH%"
if exist "C:\Program Files (x86)\CMake\bin" set "PATH=C:\Program Files (x86)\CMake\bin;%PATH%"
