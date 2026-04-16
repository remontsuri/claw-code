# Установка ROCm библиотек для RX 6750 XT (gfx1031)

## Шаг 1: Скачать библиотеки

Перейдите: https://github.com/likelovewant/ROCmLibs-for-gfx1103-AMD780M-APU/releases

Скачайте **один из этих файлов** (выберите по версии HIP SDK):

### Для HIP SDK 6.4.2 (если у вас ROCm 6.x):
`rocm.gfx1031.for.hip.6.4.2.7z`

### Для HIP SDK 6.2.4:
`rocm.gfx1031.for.hip.sdk.6.2.4.littlewu.s.logic.7z`

### Для HIP SDK 5.7 (если у вас ROCm 5.7):
`rocm gfx1031 for hip sdk 5.7 optimized with little wu logic and I8II support.7z`

## Шаг 2: Проверить версию ROCm

```powershell
# Проверьте какая версия установлена
Get-ChildItem "C:\Program Files\AMD\ROCm" -Directory
```

У вас установлены: ROCm 5.7 и 7.1

## Шаг 3: Backup текущих библиотек

```powershell
# Backup ROCm 5.7
Copy-Item "C:\Program Files\AMD\ROCm\5.7\bin" "C:\Program Files\AMD\ROCm\5.7\bin_backup" -Recurse -Force

# Backup ROCm 7.1 (если будете использовать)
Copy-Item "C:\Program Files\AMD\ROCm\7.1\bin" "C:\Program Files\AMD\ROCm\7.1\bin_backup" -Recurse -Force
```

## Шаг 4: Распаковать и заменить

1. Распакуйте скачанный .7z архив (используйте 7-Zip)
2. Внутри будет `rocblas.dll` и папка `library`
3. Замените файлы:

```powershell
# Для ROCm 5.7
# Скопируйте rocblas.dll в C:\Program Files\AMD\ROCm\5.7\bin\
# Замените папку library в C:\Program Files\AMD\ROCm\5.7\bin\rocblas\library\
```

## Шаг 5: Запустить Ollama

```powershell
# Остановить Ollama
Get-Process | Where-Object {$_.ProcessName -like "*ollama*"} | Stop-Process -Force

# Запустить с правильными настройками
$env:HSA_OVERRIDE_GFX_VERSION = "10.3.0"
ollama serve
```

Должно показать: **"Supported GPU detected"** или **"inference compute id=GPU"**

## Шаг 6: Проверить

```powershell
# В другом терминале
ollama ps
```

Должно показать `PROCESSOR: GPU` вместо `CPU`

## Если не работает

1. Попробуйте другую версию библиотек (6.4.2 вместо 5.7)
2. Убедитесь что заменили ВСЕ файлы (rocblas.dll + папку library)
3. Перезагрузите компьютер
4. Проверьте что HSA_OVERRIDE_GFX_VERSION=10.3.0 установлен

## Автоматический скрипт

Создайте файл `install_rocm_gfx1031.ps1`:

```powershell
# Требует запуска от администратора

Write-Host "=== Установка ROCm библиотек для gfx1031 ===" -ForegroundColor Green

# 1. Backup
Write-Host "`nСоздаем backup..." -ForegroundColor Yellow
Copy-Item "C:\Program Files\AMD\ROCm\5.7\bin" "C:\Program Files\AMD\ROCm\5.7\bin_backup_$(Get-Date -Format 'yyyyMMdd_HHmmss')" -Recurse -Force

# 2. Укажите путь к распакованному архиву
$sourcePath = Read-Host "Введите путь к распакованной папке с библиотеками"

if (Test-Path $sourcePath) {
    # 3. Копируем rocblas.dll
    Copy-Item "$sourcePath\rocblas.dll" "C:\Program Files\AMD\ROCm\5.7\bin\" -Force
    
    # 4. Копируем library
    Copy-Item "$sourcePath\library" "C:\Program Files\AMD\ROCm\5.7\bin\rocblas\" -Recurse -Force
    
    Write-Host "`nБиблиотеки установлены!" -ForegroundColor Green
    Write-Host "Запустите: ollama serve" -ForegroundColor Cyan
} else {
    Write-Host "Путь не найден: $sourcePath" -ForegroundColor Red
}
```

Запустите от администратора:
```powershell
powershell -ExecutionPolicy Bypass -File install_rocm_gfx1031.ps1
```
