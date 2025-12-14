# Simple API Test Script
$baseUrl = "http://localhost:8080"
$apiUrl = "$baseUrl/api/v1"

Write-Host "=== Testing Health Check ==="
try {
    $response = Invoke-WebRequest -Uri "$baseUrl/health" -Method GET -UseBasicParsing
    Write-Host "Status: $($response.StatusCode)"
    Write-Host "Response: $($response.Content)"
} catch {
    Write-Host "Error: $_"
}

Write-Host "`n=== Testing List Databases (should be empty) ==="
try {
    $response = Invoke-WebRequest -Uri "$apiUrl/dbs" -Method GET -UseBasicParsing
    Write-Host "Status: $($response.StatusCode)"
    Write-Host "Response: $($response.Content)"
} catch {
    Write-Host "Error: $_"
}

Write-Host "`n=== Testing Add Database ==="
$body = @{
    url = "postgres://postgres:postgres@localhost:5432/postgres"
} | ConvertTo-Json

try {
    $response = Invoke-WebRequest -Uri "$apiUrl/dbs/test-db" -Method PUT -Body $body -ContentType "application/json" -UseBasicParsing
    Write-Host "Status: $($response.StatusCode)"
    Write-Host "Response: $($response.Content)"
} catch {
    Write-Host "Error: $($_.Exception.Message)"
    if ($_.Exception.Response) {
        $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
        $responseBody = $reader.ReadToEnd()
        Write-Host "Response Body: $responseBody"
    }
}

Write-Host "`n=== Testing List Databases (should have test-db) ==="
try {
    $response = Invoke-WebRequest -Uri "$apiUrl/dbs" -Method GET -UseBasicParsing
    Write-Host "Status: $($response.StatusCode)"
    Write-Host "Response: $($response.Content)"
} catch {
    Write-Host "Error: $_"
}

Write-Host "`n=== Testing Get Database Metadata ==="
try {
    $response = Invoke-WebRequest -Uri "$apiUrl/dbs/test-db" -Method GET -UseBasicParsing
    Write-Host "Status: $($response.StatusCode)"
    Write-Host "Response: $($response.Content)"
} catch {
    Write-Host "Error: $($_.Exception.Message)"
    if ($_.Exception.Response) {
        $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
        $responseBody = $reader.ReadToEnd()
        Write-Host "Response Body: $responseBody"
    }
}

Write-Host "`n=== Testing SQL Query ==="
$queryBody = @{
    sql = "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' LIMIT 10"
} | ConvertTo-Json

try {
    $response = Invoke-WebRequest -Uri "$apiUrl/dbs/test-db/query" -Method POST -Body $queryBody -ContentType "application/json" -UseBasicParsing
    Write-Host "Status: $($response.StatusCode)"
    Write-Host "Response: $($response.Content)"
} catch {
    Write-Host "Error: $($_.Exception.Message)"
    if ($_.Exception.Response) {
        $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
        $responseBody = $reader.ReadToEnd()
        Write-Host "Response Body: $responseBody"
    }
}

Write-Host "`n=== Testing Invalid SQL (should fail) ==="
$invalidQueryBody = @{
    sql = "INSERT INTO users (name) VALUES ('test')"
} | ConvertTo-Json

try {
    $response = Invoke-WebRequest -Uri "$apiUrl/dbs/test-db/query" -Method POST -Body $invalidQueryBody -ContentType "application/json" -UseBasicParsing
    Write-Host "Status: $($response.StatusCode)"
    Write-Host "Response: $($response.Content)"
} catch {
    Write-Host "Expected Error: $($_.Exception.Message)"
    if ($_.Exception.Response) {
        $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
        $responseBody = $reader.ReadToEnd()
        Write-Host "Response Body: $responseBody"
    }
}

Write-Host "`n=== Tests Complete ==="

