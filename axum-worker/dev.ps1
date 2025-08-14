#!/usr/bin/env pwsh

# Exit on error
$ErrorActionPreference = "Stop"

# Variables to track processes
$TrunkJob = $null

# Cleanup function to ensure Trunk is always stopped
function Cleanup {
    Write-Host "Stopping Trunk..." -ForegroundColor Yellow
    if ($TrunkJob -and !$TrunkJob.HasExited) {
        Stop-Process -Id $TrunkJob.Id -Force -ErrorAction SilentlyContinue
    }
}

# Register cleanup to run on script exit
Register-EngineEvent -SourceIdentifier PowerShell.Exiting -Action { Cleanup }

try {
    # Start Trunk in watch mode, outputting to the axum-worker/static directory
    Set-Location "leptos-wasm"
    Write-Host "Starting Trunk in watch mode..." -ForegroundColor Green
    $TrunkJob = Start-Process -FilePath "trunk" -ArgumentList "watch" -PassThru -NoNewWindow
    
    # Wait a moment to ensure Trunk has started
    Start-Sleep -Seconds 2
    
    # Start the axum worker in watch mode
    Set-Location "../axum-worker"
    Write-Host "Starting Wrangler dev..." -ForegroundColor Green
    npx wrangler dev
}
finally {
    # Cleanup will be called automatically, but we can also call it explicitly
    Cleanup
}