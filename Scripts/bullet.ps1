# Download, extract, and build Bullet Physics in the current directory

$ErrorActionPreference = "Stop"

# Set variables
$bulletVersion = "3.25"
$bulletZipUrl = "https://github.com/bulletphysics/bullet3/archive/refs/tags/$bulletVersion.zip"
$bulletZip = "bullet3-$bulletVersion.zip"
$bulletDir = "bullet"
$bulletSrcDir = "bullet3-$bulletVersion"
$buildDir = "$bulletDir/build"

# Download Bullet Physics
Invoke-WebRequest -Uri $bulletZipUrl -OutFile $bulletZip

# Extract
Expand-Archive -Path $bulletZip -DestinationPath $bulletDir

# Create build directory
New-Item -ItemType Directory -Path $buildDir -Force | Out-Null

# Run CMake to generate Visual Studio solution
cmake -S "$bulletDir\$bulletSrcDir" -B $buildDir -G "Visual Studio 16 2019" -A x64

# Build using MSBuild
msbuild "$buildDir\ALL_BUILD.vcxproj" /p:Configuration=Release

Write-Host "Bullet Physics built successfully. Libraries are in $buildDir\lib\Release"