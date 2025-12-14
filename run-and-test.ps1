# Run server and test database addition with visible logs
$ErrorActionPreference = "Continue"

Write-Host "=== Compiling and Starting Backend Server ===" -ForegroundColor Green
cd backend

# Compile first
Write-Host "Compiling..." -ForegroundColor Yellow
cargo build 2>&1 | Out-Null
if ($LASTEXITCODE -ne 0) {
    Write-Host "Compilation failed!" -ForegroundColor Red
    exit 1
}

Write-Host "Starting server..." -ForegroundColor Yellow
$env:SQLITE_DB_PATH = ".\test.db"
$env:PORT = "8080"

# Start server in a separate window so we can see logs
Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd '$PWD'; `$env:SQLITE_DB_PATH='.\test.db'; `$env:PORT='8080'; cargo run" -WindowStyle Normal

Write-Host "Waiting 10 seconds for server to start..." -ForegroundColor Yellow
Start-Sleep -Seconds 10

# Test health check
Write-Host "`n=== Testing Health Check ===" -ForegroundColor Green
try {
    $health = Invoke-RestMethod -Uri "http://localhost:8080/health" -Method GET -TimeoutSec 5
    Write-Host "✓ Server is running: $health" -ForegroundColor Green
} catch {
    Write-Host "✗ Server not responding" -ForegroundColor Red
    exit 1
}

# Test adding database
Write-Host "`n=== Testing Add Database ===" -ForegroundColor Green
Write-Host "URL: postgresql://postgres:postgres@localhost:5432/chat" -ForegroundColor Cyan

$body = @{
    url = "postgresql://postgres:postgres@localhost:5432/chat"
} | ConvertTo-Json

Write-Host "`nSending request (this may take a while if connection fails)..." -ForegroundColor Yellow
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
    Write-Host "✗ Error after $duration seconds" -ForegroundColor Red
    Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Red
    if ($_.ErrorDetails) {
        Write-Host "Details: $($_.ErrorDetails.Message)" -ForegroundColor Red
    }
    Write-Host "`nCheck the server window for detailed logs!" -ForegroundColor Yellow
}

cd ..
Write-Host "`n=== Test Complete ===" -ForegroundColor Green
Write-Host "Check the server window for detailed logs of the connection attempt." -ForegroundColor Yellow

