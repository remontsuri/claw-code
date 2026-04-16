# Start Ollama with GPU (Vulkan) support

# Очистить конфликтующие переменные
Remove-Item Env:\CUDA_VISIBLE_DEVICES -ErrorAction SilentlyContinue
Remove-Item Env:\HSA_OVERRIDE_GFX_VERSION -ErrorAction SilentlyContinue
Remove-Item Env:\OLLAMA_GPU_LAYERS -ErrorAction SilentlyContinue

# Установить Vulkan
$env:OLLAMA_VULKAN = "1"

# Запустить Ollama
Write-Host "Starting Ollama with Vulkan GPU support..."
ollama serve
