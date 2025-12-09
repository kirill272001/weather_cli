# Weather CLI\nCLI weather + time
# Weather CLI

CLI застосунок на Rust, який показує **погоду** та **локальний час** для вибраного міста.

Використані технології:
- Rust (async/await)
- Tokio — async runtime
- Reqwest — HTTP-клієнт
- Serde — JSON-десеріалізація
- Clap — парсинг аргументів командного рядка
- Open-Meteo API — погода
- WorldTimeAPI — час

---

## 📦 Вимоги

- Встановлений Rust (рекомендовано 1.70+)
- Підключення до інтернету

---

## 🚀 Запуск

У корені проєкту:

```bash
cargo run -- <city>

