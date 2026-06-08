# weather — simple CLI weather tool (Open‑Meteo)

`weather` is a lightweight and fast Rust-based CLI tool that shows the current weather
or a multi‑day forecast.  
It automatically detects your region and switches between **Celsius / Fahrenheit** and **m/s / mph**.

Powered by the Open‑Meteo API — completely free and requires no API keys.

---

## ✨ Features

- Current weather based on IP (via ipinfo.io)
- Weather by city (`--city`)
- Weather forecast (`--forecast`)
- Custom number of forecast days (`--days 1–7`)
- Automatic unit detection:
  - USA → °F + mph  
  - Rest of world → °C + m/s
- Manual override:
  - `--f` → force Fahrenheit + mph  
  - `--c` → force Celsius + m/s
- Fast native Rust binary
- Works on Linux, WSL2, macOS

---

## 📦 Installation

Build the project:

```bash
cargo build --release

mkdir -p ~/.local/bin
cp target/release/weather ~/.local/bin/
chmod +x ~/.local/bin/weather

echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc 
source ~/.bashrc


# Installation man page:

sudo mkdir -p /usr/local/share/man/man1
sudo cp weather.1 /usr/local/share/man/man1/
sudo mandb

man weather
