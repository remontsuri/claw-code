# Test Ollama API directly

Write-Host "=== Testing Ollama API ==="

# Test 1: Version
Write-Host "`n1. Checking version..."
curl http://127.0.0.1:11434/api/version

# Test 2: List models
Write-Host "`n`n2. Listing models..."
curl http://127.0.0.1:11434/api/tags | ConvertFrom-Json | Select-Object -ExpandProperty models | Select-Object name, size

# Test 3: Simple chat completion (OpenAI compatible endpoint)
Write-Host "`n`n3. Testing chat completion..."
$body = @{
    model = "llama3.2"
    messages = @(
        @{
            role = "user"
            content = "Say 'hello' in one word"
        }
    )
    max_tokens = 10
    stream = $false
} | ConvertTo-Json

curl -Method POST -Uri "http://127.0.0.1:11434/v1/chat/completions" -ContentType "application/json" -Body $body

Write-Host "`n`n=== Done ==="
