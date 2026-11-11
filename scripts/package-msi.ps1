<# Builds the Windows Installer package from the same folder as the portable ZIP. #>
param(
    [Parameter(Mandatory)][string]$Package,
    [Parameter(Mandatory)][string]$Out,
    [string]$WixSource = (Join-Path (Split-Path $PSScriptRoot -Parent) 'packaging/windows/Product.wxs'),
    [string]$LicenseRtf = (Join-Path (Split-Path $PSScriptRoot -Parent) 'packaging/windows/License.rtf')
)
$ErrorActionPreference = 'Stop'

$packageRoot = (Resolve-Path -LiteralPath $Package -ErrorAction Stop).Path
$metadataPath = Join-Path $packageRoot 'sunset-install.json'
if (-not (Test-Path -LiteralPath $metadataPath)) {
    $metadataPath = Join-Path $packageRoot 'aurora-install.json'
}
if (-not (Test-Path -LiteralPath $metadataPath)) { throw "Missing package metadata: $metadataPath" }
$version = (Get-Content -LiteralPath $metadataPath -Raw | ConvertFrom-Json).version
if ($version -notmatch '^(\d+)\.(\d+)\.(\d+)$') {
    throw "MSI requires a stable SemVer version; got '$version'."
}
if ([int]$Matches[1] -gt 255 -or [int]$Matches[2] -gt 255 -or [int]$Matches[3] -gt 65535) {
    throw "MSI version '$version' exceeds Windows Installer's version limits."
}
if (-not (Test-Path -LiteralPath $WixSource)) { throw "Missing WiX authoring: $WixSource" }
if (-not (Test-Path -LiteralPath $LicenseRtf)) { throw "Missing WiX license RTF: $LicenseRtf" }
$licensePath = (Resolve-Path -LiteralPath $LicenseRtf -ErrorAction Stop).Path
$wix = Get-Command wix -ErrorAction Stop
$extensions = (& $wix.Source extension list --global 2>$null | Out-String)
if ($extensions -notmatch 'WixToolset\.UI\.wixext') {
    & $wix.Source extension add --global WixToolset.UI.wixext/5.0.2
    if ($LASTEXITCODE -ne 0) { throw "Failed to install WixToolset.UI.wixext ($LASTEXITCODE)." }
}
$outPath = [IO.Path]::GetFullPath($Out)
New-Item -ItemType Directory -Force -Path (Split-Path -Parent $outPath) -ErrorAction Stop | Out-Null

$arguments = @(
    'build',
    '-arch', 'x64',
    '-ext', 'WixToolset.UI.wixext',
    '-d', "PackageRoot=$packageRoot",
    '-d', "PackageVersion=$version",
    '-d', "LicenseRtf=$licensePath",
    '-o', $outPath,
    $WixSource
)
& $wix.Source @arguments
if ($LASTEXITCODE -ne 0) { throw "WiX failed with exit code $LASTEXITCODE." }
if (-not (Test-Path -LiteralPath $outPath)) { throw 'WiX did not create the MSI.' }
Write-Host "Created $outPath (SunSet Viewer $version)"
