# create-portable-tauri.ps1
param(
    [Parameter(Mandatory=$true)]
    [string]$AppName,

    [Parameter(Mandatory=$false)]
    [string]$BuildPath = "src-tauri\target\release",

    [Parameter(Mandatory=$false)]
    [string]$OutputDir = "portable-dist"
)

Write-Host "Creating portable Tauri application: $AppName" -ForegroundColor Green

# Create output directory
if (Test-Path $OutputDir) {
    Remove-Item $OutputDir -Recurse -Force
}
New-Item -ItemType Directory -Path $OutputDir | Out-Null

# Copy main executable
$exePath = Join-Path $BuildPath "$AppName.exe"
if (-not (Test-Path $exePath)) {
    Write-Error "Executable not found at $exePath. Make sure you've run 'bun tauri build' first."
    exit 1
}

Write-Host "Copying main executable..." -ForegroundColor Yellow
Copy-Item $exePath $OutputDir

# Function to find and copy VC++ Runtime DLLs
function Copy-VCRuntimeDLLs {
    Write-Host "Looking for Visual C++ Runtime DLLs..." -ForegroundColor Yellow

    $vcRedistPaths = @(
        "${env:ProgramFiles}\Microsoft Visual Studio\2022\*\VC\Redist\MSVC\*\x64\Microsoft.VC143.CRT",
        "${env:ProgramFiles(x86)}\Microsoft Visual Studio\2019\*\VC\Redist\MSVC\*\x64\Microsoft.VC143.CRT",
        "${env:ProgramFiles}\Microsoft Visual Studio\2019\*\VC\Redist\MSVC\*\x64\Microsoft.VC143.CRT",
        "C:\Windows\System32"  # System DLLs as fallback
    )

    $requiredDlls = @("vcruntime140.dll", "vcruntime140_1.dll", "msvcp140.dll")
    $foundDlls = @()

    foreach ($dll in $requiredDlls) {
        $found = $false
        foreach ($path in $vcRedistPaths) {
            $dllPath = Get-ChildItem -Path $path -Filter $dll -ErrorAction SilentlyContinue | Select-Object -First 1
            if ($dllPath) {
                Copy-Item $dllPath.FullName $OutputDir
                Write-Host "  Found and copied: $dll" -ForegroundColor Green
                $foundDlls += $dll
                $found = $true
                break
            }
        }
        if (-not $found) {
            Write-Warning "Could not find $dll"
        }
    }

    return $foundDlls
}

# Function to download and extract WebView2 Runtime
function Get-WebView2Runtime {
    Write-Host "Downloading WebView2 Fixed Version Runtime..." -ForegroundColor Yellow

    $webView2Dir = Join-Path $OutputDir "WebView2"
    New-Item -ItemType Directory -Path $webView2Dir | Out-Null

    # Download WebView2 Fixed Version
    $webView2Url = "https://msedge.sf.dl.delivery.mp.microsoft.com/filestreamingservice/files/d9a8ed47-3d1c-4c0d-8b5b-3a8c5c4b3b2e/MicrosoftEdgeWebView2RuntimeInstallerX64.exe"
    $tempFile = Join-Path $env:TEMP "webview2_installer.exe"

    try {
        Invoke-WebRequest -Uri $webView2Url -OutFile $tempFile -UseBasicParsing

        # Extract the installer (it's essentially a 7z archive)
        $extractPath = Join-Path $env:TEMP "webview2_extracted"
        if (Test-Path $extractPath) {
            Remove-Item $extractPath -Recurse -Force
        }

        # Try to extract using expand (built-in Windows utility)
        & expand $tempFile $extractPath -F:*

        # Look for WebView2Loader.dll in extracted files
        $webViewLoader = Get-ChildItem -Path $extractPath -Filter "WebView2Loader.dll" -Recurse -ErrorAction SilentlyContinue
        if ($webViewLoader) {
            Copy-Item $webViewLoader.FullName $OutputDir
            Write-Host "  WebView2Loader.dll copied successfully" -ForegroundColor Green
        } else {
            Write-Warning "WebView2Loader.dll not found in extracted files"
        }

        # Clean up
        Remove-Item $tempFile -ErrorAction SilentlyContinue
        Remove-Item $extractPath -Recurse -ErrorAction SilentlyContinue

    } catch {
        Write-Warning "Failed to download WebView2 runtime. You may need to include it manually."
        Write-Host "Download from: https://developer.microsoft.com/en-us/microsoft-edge/webview2/" -ForegroundColor Cyan
    }
}

# Alternative WebView2 approach - copy from system if available
function Copy-SystemWebView2 {
    Write-Host "Looking for system WebView2 files..." -ForegroundColor Yellow

    $webView2Paths = @(
        "${env:ProgramFiles(x86)}\Microsoft\EdgeWebView\Application",
        "${env:ProgramFiles}\Microsoft\EdgeWebView\Application"
    )

    foreach ($path in $webView2Paths) {
        if (Test-Path $path) {
            $versions = Get-ChildItem $path -Directory | Sort-Object Name -Descending
            if ($versions.Count -gt 0) {
                $latestVersion = $versions[0]
                $loaderPath = Join-Path $latestVersion.FullName "WebView2Loader.dll"
                if (Test-Path $loaderPath) {
                    Copy-Item $loaderPath $OutputDir
                    Write-Host "  Copied WebView2Loader.dll from system" -ForegroundColor Green
                    return $true
                }
            }
        }
    }
    return $false
}

# Execute the functions
$copiedVCDlls = Copy-VCRuntimeDLLs

# Try system WebView2 first, then download if not found
if (-not (Copy-SystemWebView2)) {
    Get-WebView2Runtime
}

# Create a simple README
$readmeContent = @"
$AppName - Portable Version
=========================

This is a portable version of $AppName that includes all necessary runtime dependencies.

Files included:
- $AppName.exe (main application)
- WebView2Loader.dll (WebView2 runtime)
- Visual C++ Runtime DLLs: $(($copiedVCDlls -join ', '))

To run the application, simply double-click $AppName.exe

No installation required!
"@

$readmeContent | Set-Content (Join-Path $OutputDir "README.txt")

Write-Host "`nPortable package created successfully in: $OutputDir" -ForegroundColor Green
Write-Host "Contents:" -ForegroundColor Cyan
Get-ChildItem $OutputDir | ForEach-Object { Write-Host "  - $($_.Name)" }

Write-Host "`nTest this package on a clean Windows machine to ensure all dependencies are included." -ForegroundColor Yellow
