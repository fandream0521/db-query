# Test script to add database and view logs
$ErrorActionPreference = "Continue"

Write-Host "=== Starting Backend Server ===" -ForegroundColor Green
cd backend
$env:SQLITE_DB_PATH = ".\test.db"
$env:PORT = "8080"

# Start server in background job
$backendPath = Join-Path $PWD "backend"
$job = Start-Job -ScriptBlock {
    param($path)
    Set-Location $path
    $env:SQLITE_DB_PATH = ".\test.db"
    $env:PORT = "8080"
    cargo run 2>&1
} -ArgumentList $backendPath

Write-Host "Waiting for server to start..." -ForegroundColor Yellow
Start-Sleep -Seconds 10

# Check if server is running
try {
    $health = Invoke-RestMethod -Uri "http://localhost:8080/health" -Method GET -TimeoutSec 5
    Write-Host "✓ Server is running: $health" -ForegroundColor Green
} catch {
    Write-Host "✗ Server not responding yet, checking logs..." -ForegroundColor Red
    Receive-Job $job | Select-Object -Last 20
    exit 1
}

Write-Host "`n=== Testing Add Database ===" -ForegroundColor Green
Write-Host "Database URL: postgresql://postgres:postgres@localhost:5432/chat" -ForegroundColor Cyan

$body = @{
    url = "postgresql://postgres:postgres@localhost:5432/chat"
} | ConvertTo-Json

Write-Host "`nSending request..." -ForegroundColor Yellow
$startTime = Get-Date

try {
    $response = Invoke-RestMethod -Uri "http://localhost:8080/api/v1/dbs/chat-db" -Method PUT -Body $body -ContentType "application/json" -TimeoutSec 60
    $endTime = Get-Date
    $duration = ($endTime - $startTime).TotalSeconds
    Write-Host "✓ Success! (took $duration seconds)" -ForegroundColor Green
    Write-Host "Response:" -ForegroundColor Cyan
    $response | ConvertTo-Json -Depth 10
} catch {
    $endTime = Get-Date
    $duration = ($endTime - $startTime).TotalSeconds
    Write-Host "✗ Error after $duration seconds: $($_.Exception.Message)" -ForegroundColor Red
    if ($_.ErrorDetails) {
        Write-Host "Details: $($_.ErrorDetails.Message)" -ForegroundColor Red
    }
}

Write-Host "`n=== Server Logs ===" -ForegroundColor Green
Receive-Job $job | Select-Object -Last 50

Write-Host "`n=== Stopping Server ===" -ForegroundColor Yellow
Stop-Job $job
Remove-Job $job
cd ..

