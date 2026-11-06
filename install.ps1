# pixelox installer shim (Windows PowerShell).
#
#   powershell -ExecutionPolicy ByPass -c "irm https://raw.githubusercontent.com/djynnius/pixelox/main/install.ps1 | iex"
#
# Resolves the latest (or $env:PIXELOX_VERSION) GitHub release and runs the installer that the
# release workflow generated and attached to it. That installer verifies the archive's SHA-256,
# installs pixelox.exe to %USERPROFILE%\.pixelox\bin (override with $env:PIXELOX_INSTALL_DIR),
# and adds it to the user PATH; no administrator rights are needed. pixelox's local models use
# ONNX Runtime, which needs the Visual C++ Redistributable (present on most systems).
$ErrorActionPreference = 'Stop'

$repo = 'djynnius/pixelox'
if ($env:PIXELOX_VERSION) {
    $version = $env:PIXELOX_VERSION.TrimStart('v')
    $url = "https://github.com/$repo/releases/download/v$version/pixelox-installer.ps1"
} else {
    $url = "https://github.com/$repo/releases/latest/download/pixelox-installer.ps1"
}

try {
    $installer = Invoke-RestMethod -Uri $url -UseBasicParsing
} catch {
    Write-Error "pixelox: could not download $url ($($_.Exception.Message))"
    exit 1
}
Invoke-Expression $installer
