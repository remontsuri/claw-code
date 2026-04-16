# Установка Ollama с поддержкой AMD RX 6750 XT (gfx1031)

Write-Host "=== Установка Ollama для AMD RX 6750 XT ===" -ForegroundColor Green

# 1. Остановить текущий Ollama
Write-Host "`n1. Останавливаем текущий Ollama..." -ForegroundColor Yellow
Get-Process | Where-Object {$_.ProcessName -like "*ollama*"} | Stop-Process -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 2

# 2. Скачать модифицированный Ollama
Write-Host "`n2. Скачиваем модифицированный Ollama с поддержкой AMD..." -ForegroundColor Yellow
Write-Host "Откройте браузер и перейдите по ссылке:" -ForegroundColor Cyan
Write-Host "https://github.com/likelovewant/ollama-for-amd/releases" -ForegroundColor Cyan
Write-Host "`nСкачайте последний релиз для Windows (ollama-windows-amd64.zip)"
Write-Host "Распакуйте и замените файлы в C:\Program Files\Ollama\" -ForegroundColor Yellow

Write-Host "`n3. Или используйте официальный Ollama с правильными ROCm библиотеками:" -ForegroundColor Yellow
Write-Host "Перейдите: https://github.com/likelovewant/ROCmLibs-for-gfx1103-AMD780M-APU/releases" -ForegroundColor Cyan
Write-Host "Найдите релиз для gfx1031 или универсальный"
Write-Host "Замените файлы в: C:\Program Files\AMD\ROCm\5.7\bin" -ForegroundColor Yellow

Write-Host "`n4. После установки запустите:" -ForegroundColor Green
Write-Host "ollama serve" -ForegroundColor Cyan
Write-Host "`nДолжно показать: 'Supported GPU detected'" -ForegroundColor Green

Write-Host "`n=== Инструкции ===" -ForegroundColor Green
Write-Host "1. Сделайте backup ROCm библиотек перед заменой"
Write-Host "2. RX 6750 XT = gfx1031 (RDNA 2)"
Write-Host "3. Если не 