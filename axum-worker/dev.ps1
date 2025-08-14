# Development script to run both frontend and backend servers

Write-Host "Starting development servers..." -ForegroundColor Green

# Kill any existing processes
Write-Host "Killing existing processes..." -ForegroundColor Yellow
Get-Process | Where-Object {$_.ProcessName -match "cargo|trunk|wrangler|node"} | Stop-Process -Force -ErrorAction SilentlyContinue

# Start backend server
Write-Host "Starting backend server (axum-worker) on port 8787..." -ForegroundColor Cyan
$backend = Start-Process -FilePath "powershell" -ArgumentList "-NoExit", "-Command", "cd C:\Users\Jason\Desktop\Claudy\Projects\fullstack-leptos-cloudflare-template\axum-worker; cargo watch -x 'run --release'" -PassThru

# Give backend time to start
Start-Sleep -Seconds 3

# Start frontend server  
Write-Host "Starting frontend server (leptos-wasm) on port 3001..." -ForegroundColor Magenta
$frontend = Start-Process -FilePath "powershell" -ArgumentList "-NoExit", "-Command", "cd C:\Users\Jason\Desktop\Claudy\Projects\fullstack-leptos-cloudflare-template\leptos-wasm; trunk serve --port 3001" -PassThru

Write-Host "`nServers started!" -ForegroundColor Green
Write-Host "Backend: http://127.0.0.1:8787" -ForegroundColor Cyan
Write-Host "Frontend: http://127.0.0.1:3001" -ForegroundColor Magenta
Write-Host "`nPress Ctrl+C to stop all servers" -ForegroundColor Yellow

# Wait for user input to keep script running
Read-Host "Press Enter to stop servers"

# Kill processes when done
Write-Host "Stopping servers..." -ForegroundColor Red
Stop-Process -Id $backend.Id -Force -ErrorAction SilentlyContinue
Stop-Process -Id $frontend.Id -Force -ErrorAction SilentlyContinue