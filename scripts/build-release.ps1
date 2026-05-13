<#
.SYNOPSIS
    FocusTrace production release build script.

.DESCRIPTION
    1. Verifies prerequisites (Rust, Node, pnpm, Tauri CLI)
    2. Generates a signing key-pair if not present
    3. Patches tauri.conf.json with the public key
    4. Builds the optimised release installers (MSI + NSIS)
    5. Reports artifact paths

.PARAMETER Version
    Semantic version to embed (e.g. "1.0.0").
    If omitted the version already in tauri.conf.json is used.

.PARAMETER SkipSigning
    Build without update-signature support (dev / CI-less mode).

.EXAMPLE
    .\scripts\build-release.ps1 -Version 1.2.3
#>
param(
    [string]$Version = "",
    [switch]$SkipSigning
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# ── helpers ───────────────────────────────────────────────────────────────────
function Write-Step { param([string]$msg) Write-Host "`n==> $msg" -ForegroundColor Cyan }
function Write-Ok   { param([string]$msg) Write-Host "  ✓ $msg"  -ForegroundColor Green }
function Write-Warn { param([string]$msg) Write-Host "  ! $msg"  -ForegroundColor Yellow }
function Die        { param([string]$msg) Write-Host "`nERROR: $msg" -ForegroundColor Red; exit 1 }

# ── locate repo root ──────────────────────────────────────────────────────────
$RepoRoot = Split-Path $PSScriptRoot -Parent
Set-Location $RepoRoot
Write-Ok "Repo root: $RepoRoot"

# ── prerequisites ─────────────────────────────────────────────────────────────
Write-Step "Checking prerequisites"

foreach ($cmd in @("rustc","cargo","node","pnpm")) {
    if (-not (Get-Command $cmd -ErrorAction SilentlyContinue)) {
        Die "'$cmd' not found in PATH. Install it and re-run."
    }
    Write-Ok "$cmd $(& $cmd --version 2>&1 | Select-Object -First 1)"
}

# Tauri CLI via pnpm dlx (no global install required)
$TauriCli = "pnpm tauri"
Write-Ok "Tauri CLI: $TauriCli"

# ── version bump ──────────────────────────────────────────────────────────────
$ConfPath = "$RepoRoot\src-tauri\tauri.conf.json"
$Conf     = Get-Content $ConfPath -Raw | ConvertFrom-Json

if ($Version) {
    Write-Step "Setting version to $Version"
    $Conf.version = $Version
    $Conf | ConvertTo-Json -Depth 20 | Set-Content $ConfPath -Encoding UTF8
    Write-Ok "tauri.conf.json updated"
} else {
    $Version = $Conf.version
    Write-Ok "Using existing version $Version"
}

# ── signing key management ────────────────────────────────────────────────────
$KeyDir   = "$env:USERPROFILE\.tauri"
$KeyFile  = "$KeyDir\focustrace.key"
$PubFile  = "$KeyDir\focustrace.key.pub"

if (-not $SkipSigning) {
    Write-Step "Signing key"

    if (-not (Test-Path $KeyFile)) {
        Write-Warn "No key found — generating at $KeyFile"
        if (-not (Test-Path $KeyDir)) { New-Item -ItemType Directory $KeyDir | Out-Null }

        & pnpm tauri signer generate -w $KeyFile
        if ($LASTEXITCODE -ne 0) { Die "Key generation failed." }
        Write-Ok "Key generated"
    } else {
        Write-Ok "Key already exists at $KeyFile"
    }

    $Pubkey = (Get-Content $PubFile -Raw).Trim()

    # Patch pubkey into tauri.conf.json
    $ConfRaw = Get-Content $ConfPath -Raw
    # Replace the placeholder or any existing pubkey value
    $ConfRaw = $ConfRaw -replace '"pubkey":\s*"[^"]*"', """pubkey"": ""$Pubkey"""
    Set-Content $ConfPath $ConfRaw -Encoding UTF8
    Write-Ok "Public key injected into tauri.conf.json"

    # Export private key as env var for Tauri signing
    $env:TAURI_SIGNING_PRIVATE_KEY = (Get-Content $KeyFile -Raw).Trim()
    Write-Ok "TAURI_SIGNING_PRIVATE_KEY set"
} else {
    Write-Warn "Skipping signing (--SkipSigning)"
}

# ── install frontend deps ─────────────────────────────────────────────────────
Write-Step "Installing Node dependencies"
& pnpm install --frozen-lockfile
if ($LASTEXITCODE -ne 0) { Die "pnpm install failed." }
Write-Ok "Node deps ready"

# ── build ─────────────────────────────────────────────────────────────────────
Write-Step "Building release (this takes a few minutes)"
$BuildStart = Get-Date

& pnpm tauri build
if ($LASTEXITCODE -ne 0) { Die "Build failed." }

$Elapsed = [math]::Round(((Get-Date) - $BuildStart).TotalSeconds)
Write-Ok "Build completed in ${Elapsed}s"

# ── collect artifacts ─────────────────────────────────────────────────────────
Write-Step "Release artifacts"

$BundleDir = "$RepoRoot\src-tauri\target\release\bundle"

$Artifacts = @(
    (Get-ChildItem "$BundleDir\msi\*.msi"  -ErrorAction SilentlyContinue),
    (Get-ChildItem "$BundleDir\nsis\*.exe" -ErrorAction SilentlyContinue)
) | ForEach-Object { $_ } | Where-Object { $_ }

if (-not $Artifacts) {
    Write-Warn "No installers found in $BundleDir"
} else {
    foreach ($a in $Artifacts) {
        $sizeMb = [math]::Round($a.Length / 1MB, 1)
        Write-Ok "$($a.Name)  ($sizeMb MB)  →  $($a.FullName)"
    }
}

# ── summary ───────────────────────────────────────────────────────────────────
Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor DarkGray
Write-Host "  FocusTrace $Version  build complete" -ForegroundColor White
if (-not $SkipSigning) {
    Write-Host "  Pubkey:  $PubFile" -ForegroundColor DarkGray
    Write-Host "  Add it to GitHub Secrets as TAURI_SIGNING_PRIVATE_KEY" -ForegroundColor DarkGray
    Write-Host "  (value is the PRIVATE key file, not the .pub file)"    -ForegroundColor DarkGray
}
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor DarkGray
