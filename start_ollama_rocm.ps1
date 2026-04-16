# Start Ollama with ROCm for AMD RX 6750 XT

Write-Host "Stopping Ollama..."
Get-Process | Where-Object {$_.ProcessName -like "*ollama*"} | Stop-Process -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 2

# Добавить ROCm в PATH
$env:PATH = "C:\Program Files\AMD\ROCm\5.7\bin;$env:PATH"
$env:HIP_PATH = "C:\Program Files\AMD\ROCm\5.7\"
$env:HIP_PLATFORM = "amd"

# Настройки Ollama для AMD
$env:HSA_OVERRIDE_GFX_VERSION = "10.3.0"
$env:GPU_MAX_ALLOC_PERCENT = "100"

Write-Host "Starting Ollama with ROCm..."
ollama serve
