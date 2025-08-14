<#!
.SYNOPSIS
  All-in-one database reset (drop everything + recreate schema + seed) for local Postgres Docker container.
.DESCRIPTION
  1. Copies drop_all.sql and fullSchemaa_.sql into the running container.
  2. Executes drop_all.sql (ignores errors if objects missing).
  3. Executes fullSchemaa_.sql to rebuild schema + seed reference data.
  4. Optional vacuum / analyze (disabled by default).
.PARAMETER Container
  Docker container name (default: local_postgres_db)
.PARAMETER DatabaseName
  Database name (default: leptos_db)
.PARAMETER User
  Database user (default: leptos_user)
.PARAMETER Force
  Skip confirmation prompt.
.PARAMETER Analyze
  Run ANALYZE after rebuild.
.EXAMPLE
  ./reset_db.ps1 -Force
#>
[CmdletBinding()]
param(
  [string]$Container = "local_postgres_db",
  [string]$DatabaseName = "leptos_db",
  [string]$User = "leptos_user",
  [switch]$Force,
  [switch]$Analyze
)

$ErrorActionPreference = 'Stop'

$dropFile = Join-Path $PSScriptRoot 'drop_all.sql'
$schemaFile = Join-Path $PSScriptRoot 'fullSchemaa_.sql'

if (!(Test-Path $dropFile)) { Write-Error "Missing drop script: $dropFile" }
if (!(Test-Path $schemaFile)) { Write-Error "Missing schema file: $schemaFile" }

Write-Host "== Database Reset Utility ==" -ForegroundColor Cyan
Write-Host "Container : $Container" -ForegroundColor Gray
Write-Host "Database  : $DatabaseName" -ForegroundColor Gray
Write-Host "User      : $User" -ForegroundColor Gray
Write-Host "Drop File : $dropFile" -ForegroundColor Gray
Write-Host "Schema    : $schemaFile" -ForegroundColor Gray

if (-not $Force) {
  $confirm = Read-Host "This will ERASE all data in $DatabaseName. Type 'RESET' to continue"
  if ($confirm -ne 'RESET') { Write-Host 'Aborted.'; exit 1 }
}

function Test-ContainerRunning($name) {
  $status = docker ps --format '{{.Names}}' | Where-Object { $_ -eq $name }
  return [bool]$status
}

if (-not (Test-ContainerRunning $Container)) {
  Write-Error "Container '$Container' is not running."; exit 1
}

Write-Host "Copying SQL files into container..." -ForegroundColor Yellow
# Use ${Container} to avoid PowerShell parsing confusion with colon in "$Container:/..."
& docker cp -- $dropFile "${Container}:/tmp/drop_all.sql"
& docker cp -- $schemaFile "${Container}:/tmp/fullSchema.sql"

Write-Host "Executing drop_all.sql ..." -ForegroundColor Yellow
& docker exec ${Container} psql -U $User -d $DatabaseName -f /tmp/drop_all.sql | Write-Host

Write-Host "Recreating schema (fullSchemaa_.sql) ..." -ForegroundColor Yellow
& docker exec ${Container} psql -U $User -d $DatabaseName -f /tmp/fullSchema.sql | Write-Host

if ($Analyze) {
  Write-Host "Running ANALYZE ..." -ForegroundColor Yellow
  & docker exec ${Container} psql -U $User -d $DatabaseName -c 'ANALYZE;' | Write-Host
}

Write-Host "✅ Reset complete." -ForegroundColor Green
