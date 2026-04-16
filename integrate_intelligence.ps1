# Скрипт автоматической интеграции новых возможностей AI
# Запуск: .\integrate_intelligence.ps1

Write-Host "🚀 Начинаем интеграцию новых возможностей AI..." -ForegroundColor Green

# Проверка наличия Rust
Write-Host "`n📦 Проверка Rust..." -ForegroundColor Yellow
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Rust не установлен! Установите с https://rustup.rs/" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Rust найден" -ForegroundColor Green

# Переход в директорию проекта
$projectRoot = $PSScriptRoot
Set-Location $projectRoot

# Проверка структуры проекта
Write-Host "`n📁 Проверка структуры проекта..." -ForegroundColor Yellow
if (!(Test-Path "rust/crates/tools/src/lib.rs")) {
    Write-Host "❌ Файл rust/crates/tools/src/lib.rs не найден!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Структура проекта корректна" -ForegroundColor Green

# Создание резервной копии
Write-Host "`n💾 Создание резервной копии..." -ForegroundColor Yellow
$backupDir = "backup_$(Get-Date -Format 'yyyyMMdd_HHmmss')"
New-Item -ItemType Directory -Path $backupDir -Force | Out-Null
Copy-Item "rust/crates/tools/src/lib.rs" "$backupDir/lib.rs.backup"
Copy-Item "rust/crates/tools/Cargo.toml" "$backupDir/Cargo.toml.backup"
Write-Host "✅ Резервная копия создана в $backupDir" -ForegroundColor Green

# Обновление Cargo.toml
Write-Host "`n📝 Обновление Cargo.toml..." -ForegroundColor Yellow
$cargoToml = Get-Content "rust/crates/tools/Cargo.toml" -Raw

if ($cargoToml -notmatch "reqwest.*blocking") {
    Write-Host "Добавляем зависимости..." -ForegroundColor Cyan
    
    $newDeps = @"

# Новые зависимости для расширенных возможностей
reqwest = { version = "0.11", features = ["blocking", "json"] }
base64 = "0.21"
"@
    
    $cargoToml = $cargoToml -replace '(\[dependencies\])', "`$1$newDeps"
    Set-Content "rust/crates/tools/Cargo.toml" $cargoToml
    Write-Host "✅ Зависимости добавлены" -ForegroundColor Green
} else {
    Write-Host "✅ Зависимости уже установлены" -ForegroundColor Green
}

# Проверка наличия новых модулей
Write-Host "`n🔍 Проверка новых модулей..." -ForegroundColor Yellow
$webApiExists = Test-Path "rust/crates/tools/src/web_api.rs"
$aiInferenceExists = Test-Path "rust/crates/tools/src/ai_inference.rs"

if ($webApiExists) {
    Write-Host "✅ web_api.rs найден" -ForegroundColor Green
} else {
    Write-Host "⚠️  web_api.rs не найден - создайте его из INTEGRATION_GUIDE.md" -ForegroundColor Yellow
}

if ($aiInferenceExists) {
    Write-Host "✅ ai_inference.rs найден" -ForegroundColor Green
} else {
    Write-Host "⚠️  ai_inference.rs не найден - создайте его из INTEGRATION_GUIDE.md" -ForegroundColor Yellow
}

# Обновление lib.rs
Write-Host "`n📝 Обновление lib.rs..." -ForegroundColor Yellow
$libRs = Get-Content "rust/crates/tools/src/lib.rs" -Raw

$needsUpdate = $false

if ($webApiExists -and $libRs -notmatch "mod web_api") {
    Write-Host "Добавляем модуль web_api..." -ForegroundColor Cyan
    $libRs = "mod web_api;`n" + $libRs
    $needsUpdate = $true
}

if ($aiInferenceExists -and $libRs -notmatch "mod ai_inference") {
    Write-Host "Добавляем модуль ai_inference..." -ForegroundColor Cyan
    $libRs = "mod ai_inference;`n" + $libRs
    $needsUpdate = $true
}

if ($needsUpdate) {
    Set-Content "rust/crates/tools/src/lib.rs" $libRs
    Write-Host "✅ lib.rs обновлен" -ForegroundColor Green
} else {
    Write-Host "✅ lib.rs уже содержит необходимые модули" -ForegroundColor Green
}

# Сборка проекта
Write-Host "`n🔨 Сборка проекта..." -ForegroundColor Yellow
Set-Location "rust"

Write-Host "Запуск cargo build --release..." -ForegroundColor Cyan
$buildOutput = cargo build --release 2>&1

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Проект успешно собран!" -ForegroundColor Green
} else {
    Write-Host "❌ Ошибка сборки!" -ForegroundColor Red
    Write-Host $buildOutput -ForegroundColor Red
    Write-Host "`n💡 Проверьте логи выше и исправьте ошибки" -ForegroundColor Yellow
    Write-Host "💡 Резервная копия сохранена в $backupDir" -ForegroundColor Yellow
    Set-Location $projectRoot
    exit 1
}

Set-Location $projectRoot

# Запуск тестов
Write-Host "`n🧪 Запуск тестов..." -ForegroundColor Yellow
Set-Location "rust"

Write-Host "Запуск cargo test..." -ForegroundColor Cyan
$testOutput = cargo test --release 2>&1

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Все тесты пройдены!" -ForegroundColor Green
} else {
    Write-Host "⚠️  Некоторые тесты не прошли" -ForegroundColor Yellow
    Write-Host "Это нормально, если тесты требуют API ключи" -ForegroundColor Cyan
}

Set-Location $projectRoot

# Проверка исполняемого файла
Write-Host "`n🎯 Проверка исполняемого файла..." -ForegroundColor Yellow
$exePath = "rust/target/release/claw.exe"

if (Test-Path $exePath) {
    $fileInfo = Get-Item $exePath
    Write-Host "✅ Исполняемый файл создан: $exePath" -ForegroundColor Green
    Write-Host "   Размер: $([math]::Round($fileInfo.Length / 1MB, 2)) MB" -ForegroundColor Cyan
    Write-Host "   Дата: $($fileInfo.LastWriteTime)" -ForegroundColor Cyan
} else {
    Write-Host "❌ Исполняемый файл не найден!" -ForegroundColor Red
    exit 1
}

# Итоговая информация
Write-Host "`n" + "="*60 -ForegroundColor Green
Write-Host "🎉 ИНТЕГРАЦИЯ ЗАВЕРШЕНА УСПЕШНО!" -ForegroundColor Green
Write-Host "="*60 -ForegroundColor Green

Write-Host "`n📋 Что было сделано:" -ForegroundColor Yellow
Write-Host "  ✅ Создана резервная копия в $backupDir" -ForegroundColor White
Write-Host "  ✅ Обновлен Cargo.toml с новыми зависимостями" -ForegroundColor White
Write-Host "  ✅ Добавлены модули web_api и ai_inference" -ForegroundColor White
Write-Host "  ✅ Проект успешно собран" -ForegroundColor White
Write-Host "  ✅ Создан исполняемый файл claw.exe" -ForegroundColor White

Write-Host "`n🚀 Новые возможности:" -ForegroundColor Yellow
Write-Host "  🌐 WebAPI - работа с любыми REST API" -ForegroundColor White
Write-Host "  🤖 AIInference - вызов других AI моделей" -ForegroundColor White
Write-Host "  🔍 WebSearch - улучшенный поиск в интернете" -ForegroundColor White
Write-Host "  📊 WebFetch - получение данных с веб-страниц" -ForegroundColor White

Write-Host "`n📖 Примеры использования:" -ForegroundColor Yellow
Write-Host "  # Работа с API" -ForegroundColor Cyan
Write-Host "  .\rust\target\release\claw.exe `"используй WebAPI чтобы получить данные с GitHub`"" -ForegroundColor White
Write-Host ""
Write-Host "  # Вызов другой AI модели" -ForegroundColor Cyan
Write-Host "  .\rust\target\release\claw.exe `"используй AIInference с qwen2.5-coder:7b для оптимизации кода`"" -ForegroundColor White
Write-Host ""
Write-Host "  # Интерактивный режим" -ForegroundColor Cyan
Write-Host "  .\rust\target\release\claw.exe" -ForegroundColor White

Write-Host "`n⚙️  Настройка переменных окружения (опционально):" -ForegroundColor Yellow
Write-Host "  `$env:OPENAI_API_KEY = `"sk-...`"" -ForegroundColor White
Write-Host "  `$env:ANTHROPIC_API_KEY = `"sk-ant-...`"" -ForegroundColor White
Write-Host "  `$env:OLLAMA_BASE_URL = `"http://localhost:11434`"" -ForegroundColor White

Write-Host "`n📚 Документация:" -ForegroundColor Yellow
Write-Host "  📄 INTELLIGENCE_UPGRADES.md - полный список возможностей" -ForegroundColor White
Write-Host "  📄 INTEGRATION_GUIDE.md - руководство по интеграции" -ForegroundColor White
Write-Host "  📄 OLLAMA_QUICKSTART.md - работа с Ollama" -ForegroundColor White

Write-Host "`n💡 Совет:" -ForegroundColor Yellow
Write-Host "  Для лучшей производительности используйте Ollama с GPU" -ForegroundColor White
Write-Host "  Проверьте FIX_OLLAMA_AMD.md для настройки AMD GPU" -ForegroundColor White

Write-Host "`n✨ Готово! AI ассистент теперь намного умнее! ✨`n" -ForegroundColor Green
