# Решение проблемы Ollama + AMD RX 6750 XT

## Проблема
Ollama не видит GPU, использует только CPU

## Причина
RX 6750 XT (RDNA 2, gfx1031) требует правильные ROCm библиотеки

## Решение

### 1. Определить gfx код вашей карты
RX 6750 XT = **gfx1031**

### 2. Скачать правильные ROCm библиотеки
Источник: https://github.com/likelovewant/ROCmLibs-for-gfx1103-AMD780M-APU/releases

Нужно найти релиз для gfx1031 или использовать универсальные библиотеки

### 3. Заменить библиотеки в Ollama
Путь: `C:\Program Files\AMD\ROCm\5.7\bin` или `C:\Program Files\AMD\ROCm\7.1\bin`

**ВАЖНО: Сделайте backup перед заменой!**

```powershell
# Backup
Copy-Item "C:\Program Files\AMD\ROCm\5.7\bin" "C:\Program Files\AMD\ROCm\5.7\bin_backup" -Recurse

# Замените файлы из скачанного архива
```

### 4. Альтернатива: Использовать модифицированный Ollama
https://github.com/likelovewant/ollama-for-amd

Этот форк поддерживает больше AMD карт

### 5. Проверить
```powershell
ollama serve
# Должно показать "Supported GPU detected"
```

## Текущий статус
- ROCm 5.7 и 7.1 установлены
- HSA_OVERRIDE_GFX_VERSION=10.3.0 установлен
- Ollama запущен но использует CPU

## Следующий шаг
Скачать и установить правильные ROCm библиотеки для gfx1031
