# Start Backend Server
cd backend
$env:SQLITE_DB_PATH = ".\test.db"
$env:PORT = "8080"
Write-Host "Starting backend server on port 8080..."
Write-Host "SQLite DB path: $env:SQLITE_DB_PATH"
cargo run

