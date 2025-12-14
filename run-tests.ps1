# Run API Tests
$baseUrl = "http://localhost:8080"
$apiUrl = "$baseUrl/api/v1"

Write-Host "=== 1. Health Check ==="
try {
    $response = Invoke-RestMethod -Uri "$baseUrl/health" -Method GET
    Write-Host "✓ Health: $response" -ForegroundColor Green
} catch {
    Write-Host "✗ Health check failed: $_" -ForegroundColor Red
    exit 1
}

Write-Host "`n=== 2. List Databases (empty) ==="
try {
    $response = Invoke-RestMethod -Uri "$apiUrl/dbs" -Method GET
    Write-Host "✓ Databases: $($response | ConvertTo-Json -Compress)" -ForegroundColor Green
} catch {
    Write-Host "✗ Failed: $_" -ForegroundColor Red
}

Write-Host "`n=== 3. Add Database ==="
$body = @{
    url = "postgres://postgres:postgres@localhost:5432/postgres"
} | ConvertTo-Json

try {
    $response = Invoke-RestMethod -Uri "$apiUrl/dbs/test-db" -Method PUT -Body $body -ContentType "application/json"
    Write-Host "✓ Database added: $($response | ConvertTo-Json -Compress)" -ForegroundColor Green
} catch {
    Write-Host "✗ Failed: $_" -ForegroundColor Red
    if ($_.Exception.Response) {
        $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
        $responseBody = $reader.ReadToEnd()
        Write-Host "  Response: $responseBody" -ForegroundColor Yellow
    }
}

Write-Host "`n=== 4. List Databases (should have test-db) ==="
try {
    $response = Invoke-RestMethod -Uri "$apiUrl/dbs" -Method GET
    Write-Host "✓ Databases: $($response | ConvertTo-Json -Compress)" -ForegroundColor Green
} catch {
    Write-Host "✗ Failed: $_" -ForegroundColor Red
}

Write-Host "`n=== 5. Get Database Metadata ==="
try {
    $response = Invoke-RestMethod -Uri "$apiUrl/dbs/test-db" -Method GET
    Write-Host "✓ Metadata retrieved (tables: $($response.tables.Count), views: $($response.views.Count))" -ForegroundColor Green
} catch {
    Write-Host "✗ Failed: $_" -ForegroundColor Red
    if ($_.Exception.Response) {
        $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
        $responseBody = $reader.ReadToEnd()
        Write-Host "  Response: $responseBody" -ForegroundColor Yellow
    }
}

Write-Host "`n=== 6. Execute SQL Query ==="
$queryBody = @{
    sql = "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' LIMIT 10"
} | ConvertTo-Json

try {
    $response = Invoke-RestMethod -Uri "$apiUrl/dbs/test-db/query" -Method POST -Body $queryBody -ContentType "application/json"
    Write-Host "✓ Query executed: $($response.rowCount) rows, ${response.executionTimeMs}ms" -ForegroundColor Green
} catch {
    Write-Host "✗ Failed: $_" -ForegroundColor Red
    if ($_.Exception.Response) {
        $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
        $responseBody = $reader.ReadToEnd()
        Write-Host "  Response: $responseBody" -ForegroundColor Yellow
    }
}

Write-Host "`n=== 7. Test Invalid SQL (INSERT - should fail) ==="
$invalidQueryBody = @{
    sql = "INSERT INTO users (name) VALUES ('test')"
} | ConvertTo-Json

try {
    $response = Invoke-RestMethod -Uri "$apiUrl/dbs/test-db/query" -Method POST -Body $invalidQueryBody -ContentType "application/json"
    Write-Host "✗ Should have failed but succeeded!" -ForegroundColor Red
} catch {
    Write-Host "✓ Correctly rejected non-SELECT statement" -ForegroundColor Green
    if ($_.Exception.Response) {
        $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
        $responseBody = $reader.ReadToEnd()
        $errorObj = $responseBody | ConvertFrom-Json
        Write-Host "  Error: $($errorObj.error)" -ForegroundColor Yellow
    }
}

Write-Host "`n=== 8. Test SQL without LIMIT (should auto-add LIMIT 1000) ==="
$noLimitQueryBody = @{
    sql = "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'"
} | ConvertTo-Json

try {
    $response = Invoke-RestMethod -Uri "$apiUrl/dbs/test-db/query" -Method POST -Body $noLimitQueryBody -ContentType "application/json"
    Write-Host "✓ Query executed (auto-added LIMIT): $($response.rowCount) rows" -ForegroundColor Green
} catch {
    Write-Host "✗ Failed: $_" -ForegroundColor Red
}

Write-Host "`n=== All Tests Complete ===" -ForegroundColor Cyan

