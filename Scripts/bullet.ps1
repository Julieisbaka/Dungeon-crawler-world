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

cmake -S "$bulletDir\$bulletSrcDir" -B $buildDir -G "MinGW Makefiles"

mingw32-make -C $buildDir

Write-Host "Bullet Physics built successfully. Libraries are in $buildDir\lib\Release"
