# Generates PNG icons required by the browser extension manifest.
# Run once from the extension directory:  .\make_icons.ps1

Add-Type -AssemblyName System.Drawing

$outputDir = Join-Path $PSScriptRoot "icons"
if (-not (Test-Path $outputDir)) { New-Item -ItemType Directory $outputDir | Out-Null }

foreach ($size in @(16, 48, 128)) {
    $bmp = New-Object System.Drawing.Bitmap($size, $size)
    $g   = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias

    # Background: indigo gradient (approximate with solid fill)
    $brush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 99, 102, 241))
    $g.FillRectangle($brush, 0, 0, $size, $size)
    $brush.Dispose()

    # Draw a simple crosshair / focus ring
    $pen = New-Object System.Drawing.Pen([System.Drawing.Color]::White, [float]($size / 10))
    $pen.StartCap = [System.Drawing.Drawing2D.LineCap]::Round
    $pen.EndCap   = [System.Drawing.Drawing2D.LineCap]::Round

    $cx = $size / 2.0
    $cy = $size / 2.0
    $r  = $size * 0.28
    $dot = $size * 0.08

    # Outer ring
    $g.DrawEllipse($pen, [float]($cx - $r), [float]($cy - $r), [float]($r * 2), [float]($r * 2))

    # Center dot
    $dot_brush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::White)
    $g.FillEllipse($dot_brush, [float]($cx - $dot), [float]($cy - $dot), [float]($dot * 2), [float]($dot * 2))
    $dot_brush.Dispose()
    $pen.Dispose()

    $outPath = Join-Path $outputDir "icon${size}.png"
    $bmp.Save($outPath, [System.Drawing.Imaging.ImageFormat]::Png)
    $bmp.Dispose()
    $g.Dispose()

    Write-Host "Created $outPath"
}

Write-Host "Icons generated successfully."
