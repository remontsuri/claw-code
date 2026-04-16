# Предложения по улучшению структуры проекта

## 1. Реорганизация runtime крейта

### Проблема
`runtime/src/` содержит 20+ файлов без логической группировки. Модули смешивают разные уровни абстракции.

### Решение
```
runtime/src/
├── lib.rs
├── core/              # Базовые примитивы
│   ├── mod.rs
│   ├── config.rs
│   ├── session.rs
│   └── usage.rs
├── execution/         # Выполнение команд и инструментов
│   ├── mod.rs
│   ├── bash.rs
│   ├── conversation.rs
│   └── sandbox.rs
├── mcp/              # MCP-специфичная логика
│   ├── mod.rs
│   ├── client.rs
│   ├── stdio.rs
│   └── utils.rs
├── io/               # Файловые операции
│   ├── mod.rs
│   ├── file_ops.rs
│   └── compact.rs
├── hooks/            # Система хуков
│   ├── mod.rs
│   ├── types.rs
│   └── executor.rs  # НОВОЕ: реализация выполнения хуков
└── integrations/     # Внешние интеграции
    ├── mod.rs
    ├── oauth.rs
    ├── remote.rs
    └── prompt.rs
```

## 2. Выделение отдельного крейта для хуков

### Проблема
Хуки парсятся в `plugins`, но не выполняются. Нет четкой границы ответственности.

### Решение
Создать новый крейт `hooks`:
```
rust/crates/hooks/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── types.rs       # HookEvent, HookConfig
    ├── executor.rs    # Выполнение хуков
    ├── registry.rs    # Регистрация и поиск хуков
    └── lifecycle.rs   # PreToolUse, PostToolUse логика
```

## 3. Разделение tools на категории

### Проблема
Все инструменты в одном файле `tools/src/lib.rs`. Сложно масштабировать.

### Решение
```
tools/src/
├── lib.rs            # Публичный API и регистрация
├── registry.rs       # Центральный реестр инструментов
├── builtin/          # Встроенные инструменты
│   ├── mod.rs
│   ├── shell.rs      # bash, powershell
│   ├── files.rs      # read, write, edit
│   ├── search.rs     # grep, glob
│   └── web.rs        # fetch, search
├── workflow/         # Инструменты рабочего процесса
│   ├── mod.rs
│   ├── agent.rs
│   ├── task.rs
│   └── team.rs
└── integration/      # Интеграционные инструменты
    ├── mod.rs
    ├── mcp.rs
    ├── lsp.rs
    └── skill.rs
```

## 4. Улучшение CLI структуры

### Проблема
`main.rs` слишком большой (600+ строк), смешивает логику UI и бизнес-логику.

### Решение
```
claw-cli/src/
├── main.rs           # Только entry point
├── cli/              # CLI логика
│   ├── mod.rs
│   ├── args.rs       # Существующий
│   ├── commands.rs   # Обработка команд
│   └── repl.rs       # REPL логика
├── ui/               # UI компоненты
│   ├── mod.rs
│   ├── render.rs     # Существующий
│   ├── input.rs      # Существующий
│   └── theme.rs      # Выделить из render.rs
├── session/          # Управление сессией
│   ├── mod.rs
│   ├── app.rs        # Существующий
│   └── state.rs      # Выделить из app.rs
└── init/             # Инициализация проекта
    ├── mod.rs
    └── templates.rs  # Шаблоны конфигов
```

## 5. Создание крейта для общих типов

### Проблема
Дублирование типов между крейтами. Циклические зависимости.

### Решение
Создать `common` или `types` крейт:
```
rust/crates/common/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── error.rs      # Общие типы ошибок
    ├── events.rs     # События системы
    ├── config.rs     # Общие конфиг-типы
    └── traits.rs     # Общие трейты
```

## 6. Улучшение структуры plugins

### Проблема
Плагины не выполняются, только парсятся. Нет четкой архитектуры.

### Решение
```
plugins/src/
├── lib.rs
├── types.rs          # Plugin, PluginManifest
├── loader.rs         # Загрузка плагинов
├── registry.rs       # Реестр плагинов
├── executor.rs       # НОВОЕ: выполнение плагинов
├── lifecycle.rs      # НОВОЕ: жизненный цикл
└── bundled/          # Встроенные плагины
    ├── mod.rs
    └── ...
```

## 7. Документация архитектуры

### Создать новые файлы
```
docs/
├── ARCHITECTURE.md   # Общая архитектура
├── CRATES.md         # Описание каждого крейта
├── HOOKS.md          # Система хуков
├── PLUGINS.md        # Система плагинов
└── CONTRIBUTING.md   # Руководство для контрибьюторов
```

## 8. Тестовая структура

### Проблема
Тесты разбросаны, нет интеграционных тестов.

### Решение
```
rust/
├── tests/            # Интеграционные тесты
│   ├── integration/
│   │   ├── cli_tests.rs
│   │   ├── hook_tests.rs
│   │   └── plugin_tests.rs
│   └── fixtures/     # Тестовые данные
└── crates/*/tests/   # Unit тесты остаются в крейтах
```

## 9. Конфигурация и настройки

### Проблема
Конфигурация размазана по разным местам.

### Решение
```
rust/crates/config/   # НОВЫЙ крейт
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── loader.rs     # Загрузка конфигов
    ├── merger.rs     # Слияние конфигов
    ├── validator.rs  # Валидация
    └── defaults.rs   # Дефолтные значения
```

## 10. Workspace зависимости

### Улучшить Cargo.toml
```toml
[workspace.dependencies]
# Общие зависимости
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["rt-multi-thread"] }
anyhow = "1"
thiserror = "1"

# Внутренние крейты
common = { path = "crates/common" }
config = { path = "crates/config" }
hooks = { path = "crates/hooks" }
```

## Приоритеты реализации

### Фаза 1 (Критично)
1. Создать `hooks` крейт с выполнением хуков
2. Реорганизовать `runtime` по модулям
3. Разделить `tools` на категории

### Фаза 2 (Важно)
4. Рефакторинг `claw-cli/main.rs`
5. Создать `common` крейт
6. Улучшить `plugins` с выполнением

### Фаза 3 (Желательно)
7. Добавить документацию архитектуры
8. Организовать интеграционные тесты
9. Создать `config` крейт
10. Оптимизировать workspace зависимости

## Ожидаемые результаты

- ✅ Четкое разделение ответственности
- ✅ Упрощение навигации по коду
- ✅ Возможность параллельной разработки
- ✅ Лучшая тестируемость
- ✅ Упрощение добавления новых фич
- ✅ Уменьшение времени компиляции (за счет модульности)
