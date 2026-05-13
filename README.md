<div align="center">

<img src="src-tauri/icons/128x128.png" alt="FocusTrace Logo" width="96" height="96" />

# FocusTrace

**Локальный трекер продуктивности для Windows**

Отслеживает активные окна, приложения и время фокусировки — полностью офлайн, без облака и слежки.

[![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.x-blue.svg?logo=tauri)](https://tauri.app/)
[![React](https://img.shields.io/badge/React-18-61DAFB.svg?logo=react&logoColor=white)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.x-3178C6.svg?logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![SQLite](https://img.shields.io/badge/SQLite-3.x-003B57.svg?logo=sqlite&logoColor=white)](https://www.sqlite.org/)
[![License](https://img.shields.io/badge/license-EULA-green.svg)](LICENSE.txt)
[![Platform](https://img.shields.io/badge/platform-Windows%2010%2B-0078D4.svg?logo=windows&logoColor=white)](https://www.microsoft.com/windows)
[![Version](https://img.shields.io/badge/version-0.2.2-lightgrey.svg)](https://github.com/ShEIH24/FocusTrace/releases)

</div>

---

## О проекте

**FocusTrace** — desktop-приложение для Windows, которое в фоновом режиме отслеживает, какие приложения вы используете и сколько времени проводите за продуктивной работой. Все данные хранятся **только на вашем устройстве** — никаких серверов, никакой телеметрии.

### Что делает приложение

- 🪟 **Отслеживание окон** — фиксирует активное приложение и заголовок окна каждую секунду
- ⏱️ **Учёт времени** — считает продолжительность каждой сессии работы
- 😴 **Определение AFK** — автоматически обнаруживает периоды простоя
- 📊 **Аналитика** — рассчитывает Focus Score и строит отчёты по дням и неделям
- 🏷️ **Классификация** — делит приложения на продуктивные, нейтральные и отвлекающие
- 🔒 **Приватность** — данные не покидают компьютер

---

## Скриншот

<img width="1102" height="752" alt="image" src="https://github.com/user-attachments/assets/e6aff675-30f7-435c-bad9-8828e67d5301" />

---

## Технологии

| Слой | Технология | Описание |
|------|-----------|----------|
| Backend | [Rust](https://www.rust-lang.org/) | Нативный бинарник, сбор данных через Win32 API |
| Desktop | [Tauri 2](https://tauri.app/) | Фреймворк для desktop-приложений на Rust |
| Frontend | [React 18](https://react.dev/) + [TypeScript](https://www.typescriptlang.org/) | UI компоненты |
| Сборка | [Vite 5](https://vitejs.dev/) | Бандлер и dev-сервер |
| Стили | [Tailwind CSS](https://tailwindcss.com/) | Утилитарный CSS фреймворк |
| State | [Zustand](https://zustand-demo.pmnd.rs/) | Глобальное состояние |
| Запросы | [React Query](https://tanstack.com/query) | Кэширование и синхронизация данных |
| База данных | [SQLite](https://www.sqlite.org/) + [sqlx](https://github.com/launchbear/sqlx) | Локальное хранилище |
| Async | [Tokio](https://tokio.rs/) | Асинхронный рантайм Rust |

---

## Быстрый старт

### Системные требования

- Windows 10 версии 1809 или новее
- [Microsoft WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/) (устанавливается автоматически)

### Установка

1. Скачай последний инсталлятор со страницы [Releases](https://github.com/ShEIH24/FocusTrace/releases)
2. Запусти `FocusTrace_0.2.2_Setup.exe`
3. Следуй инструкциям мастера установки
4. Запусти FocusTrace из меню «Пуск» или с рабочего стола

---

## Установка браузерного расширения

Расширение FocusTrace позволяет отслеживать посещаемые сайты в реальном времени и классифицировать их как продуктивные или отвлекающие.

### Как это работает

Расширение подключается к запущенному десктопному приложению по WebSocket (только локально, `127.0.0.1:9919`) и передаёт домен активной вкладки. Никакие данные не уходят в интернет.

### Шаг 1 — Получи токен в приложении

1. Запусти FocusTrace
2. Перейди в **Настройки → Браузерный трекинг**
3. Нажми **Копировать** рядом с полем «Токен подключения»

### Шаг 2 — Установи расширение

#### Chrome и Microsoft Edge

1. Открой `chrome://extensions` (или `edge://extensions`)
2. Включи **Режим разработчика** (переключатель в правом верхнем углу)
3. Нажми **«Загрузить распакованное»**
4. Выбери папку `extensions/focustrace` внутри директории установки FocusTrace  
   *(по умолчанию: `%LOCALAPPDATA%\FocusTrace\extensions\focustrace`)*
5. Расширение появится в списке с иконкой FocusTrace

#### Firefox

1. Открой `about:debugging#/runtime/this-firefox`
2. Нажми **«Загрузить временное дополнение»**
3. Выбери файл `extensions/focustrace/manifest.firefox.json`
4. Расширение активируется до следующего перезапуска Firefox

> **Постоянная установка в Firefox**: подпишите расширение через [Firefox Add-on Hub](https://addons.mozilla.org/developers/) или используйте Firefox Developer Edition с отключённой проверкой подписи (`xpinstall.signatures.required = false` в `about:config`).

### Шаг 3 — Введи токен в расширении

1. Нажми на иконку FocusTrace в панели расширений браузера
2. В поле **«Токен»** вставь скопированный токен
3. Нажми **«Сохранить»** — статус должен смениться на **«Подключено»**

### Шаг 4 — Классифицируй сайты

В приложении, в разделе **Настройки → Браузерный трекинг**, выбери популярные сайты из готовых списков или добавь свои домены вручную (по одному на строку).

---

## Сборка из исходников

### Требования

| Инструмент | Версия | Ссылка |
|-----------|--------|--------|
| Rust | 1.77+ | [rustup.rs](https://rustup.rs/) |
| Node.js | 20 LTS | [nodejs.org](https://nodejs.org/) |
| pnpm | 9+ | [pnpm.io](https://pnpm.io/) |
| Visual Studio Build Tools | 2022 | [visualstudio.microsoft.com](https://visualstudio.microsoft.com/downloads/) |

Компонент Visual Studio: **Desktop development with C++**

### Клонирование и запуск

```bash
# Клонировать репозиторий
git clone https://github.com/ShEIH24/FocusTrace.git
cd FocusTrace

# Установить зависимости
pnpm install

# Запустить в режиме разработки
pnpm tauri dev
```

Первый запуск компилирует все Rust-зависимости — это займёт **3–5 минут**. Последующие запуски значительно быстрее.

### Сборка релиза

```bash
pnpm tauri build
```

Готовый бинарник и установщик появятся в `src-tauri/target/release/bundle/`.

---

## Структура проекта

```
FocusTrace/
├── src-tauri/          # Rust backend
│   ├── src/
│   │   ├── commands/   # Tauri IPC команды
│   │   ├── tracker/    # Сбор данных (Win32 API)
│   │   ├── analytics/  # Подсчёт продуктивности
│   │   ├── db/         # SQLite слой (sqlx)
│   │   ├── workers/    # Фоновые задачи (Tokio)
│   │   ├── config/     # Конфигурация
│   │   └── events/     # Внутренняя event-система
│   └── migrations/     # SQL миграции
├── src/                # React frontend
│   ├── pages/          # Страницы приложения
│   ├── components/     # UI компоненты
│   ├── store/          # Zustand сторы
│   ├── hooks/          # Кастомные хуки
│   ├── ipc/            # Типизированный Tauri API
│   └── types/          # TypeScript типы
└── installer/          # Inno Setup скрипт
```

---

## Конфиденциальность

FocusTrace собирает и хранит **только на вашем устройстве**:

- Имена исполняемых файлов и заголовки окон
- Временные метки и длительность сессий
- Периоды активности и простоя

Приложение **не записывает** нажатия клавиш, содержимое буфера обмена и скриншоты. Данные хранятся в `%APPDATA%\FocusTrace\` и никуда не передаются.

---

## Лицензия

Распространяется по условиям [лицензионного соглашения конечного пользователя](LICENSE.txt).

---

<div align="center">

Сделано с ❤️ и Rust · [GitHub](https://github.com/ShEIH24) · [Сообщить об ошибке](https://github.com/ShEIH24/FocusTrace/issues)

</div>
