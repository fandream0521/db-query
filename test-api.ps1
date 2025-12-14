# Test API Script
$ErrorActionPreference = "Stop"

Write-Host "Starting backend server..."
cd backend
Start-Job -ScriptBlock {
    Set-Location $using:PWD
    cd backend
    cargo run
} | Out-Null

Write-Host "Waiting for server to start..."
Start-Sleep -Seconds 10

$baseUrl = "http://localhost:8080"
$apiUrl = "$baseUrl/api/v1"

Write-Host "`n=== Testing Health Check ==="
curl.exe -s "$baseUrl/health"

Write-Host "`n`n=== Testing List Databases ==="
curl.exe -s "$apiUrl/dbs"

Write-Host "`n`n=== Testing Add Database ==="
$body = '{"url":"postgres://postgres:postgres@localhost:5432/postgres"}'
curl.exe -X PUT "$apiUrl/dbs/test-db" -H "Content-Type: application/json" -d $body

Write-Host "`n`n=== Testing List Databases Again ==="
curl.exe -s "$apiUrl/dbs"

Write-Host "`n`n=== Testing Get Database Metadata ==="
curl.exe -s "$apiUrl/dbs/test-db"

Write-Host "`n`n=== Testing SQL Query ==="
$queryBody = '{"sql":"SELECT table_name FROM information_schema.tables WHERE table_schema = ''public'' LIMIT 10"}'
curl.exe -X POST "$apiUrl/dbs/test-db/query" -H "Content-Type: application/json" -d $queryBody

Write-Host "`n`n=== Tests Complete ==="

