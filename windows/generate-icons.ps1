# generate-icons.ps1
# Generates MSIX-compatible PNG icons from the app SVG.
# Requires ImageMagick (magick command) installed.

param(
    [string]$SvgPath = "data\com.example.qr_studio.svg",
    [string]$OutputDir = "msix-layout\Assets"
)

$ErrorActionPreference = "Stop"

# Create output directory
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

# Define required icon sizes for MSIX
$icons = @{
    "Square44x44Logo.png"    = 44
    "Square150x150Logo.png"  = 150
    "Wide310x150Logo.png"    = "310x150"
    "Square310x310Logo.png"  = 310
    "StoreLogo.png"          = 50
    "SplashScreen.png"       = "620x300"
}

foreach ($entry in $icons.GetEnumerator()) {
    $name = $entry.Key
    $size = $entry.Value
    $output = Join-Path $OutputDir $name

    if ($size -is [string]) {
        # Size is WxH format
        magick convert -background none $SvgPath -resize "${size}!" -gravity center -extent $size $output
    } else {
        # Square icon
        magick convert -background none $SvgPath -resize "${size}x${size}" -gravity center -extent "${size}x${size}" $output
    }

    if (Test-Path $output) {
        Write-Host "  Created: $name ($size)"
    } else {
        Write-Error "  FAILED: $name"
    }
}

Write-Host "`nAll icons generated in $OutputDir"
