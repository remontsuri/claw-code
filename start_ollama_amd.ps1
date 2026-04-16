# Start Ollama with AMD GPU (ROCm) support for RX 6750 XT

Write-Host "Stopping existing Ollama processes..."
Get-Process | Where-Object {$_.ProcessName -like "*ollama*"} | Stop-Process -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 2

# Очистить конфликтующие переменные
Remove-Item Env:\CUDA_VISIBLE_DEVICES -ErrorAction SilentlyContinue

# Настройки для AMD RX 6750 XT (RDNA 2 - gfx1031)
$env:OLLAMA_GPU_DRIVER = "rocm"
$env:HSA_OVERRIDE_GFX_VERSION = "10.3.0"  # Для RDNA 2
$env:OLLAMA_VULKAN = "1"
$env:GPU_MAX_ALLOC_PERCENT = "100"
$env:GPU_MAX_HEAP_SIZE = "100"

Write-Host "=== Ollama GPU Settings ==="
Write-Host "GPU Driver: $env:OLLAMA_GPU_DRIVER"
Write-Host "HSA Override: $env:HSA_OVERRIDE_GFX_VERSION"
Write-Host "Vulkan: $env:OLLAMA_VULKAN"
Write-Host "=========================="
Write-Host ""
Write-Host "Starting Ollama server..."
Write-Host "Press Ctrl+C to stop"
Write-Host ""

ollama serve
