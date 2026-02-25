<br/>
<div align="center">
  <h1 align="center">🌤️ weather</h1>
  <p align="center">
    <strong>A beautiful terminal weather app with dynamic ASCII animations driven by real-time data.</strong>
  </p>
</div>

<br/>

## 📖 About The Project

**weather** is an aesthetically pleasing, terminal-based application that brings you the weather in a uniquely visual way. Driven by real-time data from [Open-Meteo](https://open-meteo.com/), it renders immersive ASCII animations of current weather conditions right in your command line.

Experience rain drops falling across your terminal, lightning strikes on a stormy night, flying airplanes, or calm clear skies with a perfectly rendered ASCII sun and birds.

<div align="center">

| Thunderstorm Night | Snow |
| :---: | :---: |
| <img src="docs/thunderstorm-night.gif" width="600" alt="Thunderstorm Night"> | <img src="docs/snow.gif" width="600" alt="Snow"> |

</div>

## ✨ Key Features

- 🎯 **Real-Time Accuracy:** Powered by Open-Meteo for precise, up-to-date meteorological data.
- 🎨 **Dynamic ASCII Animations:** Enjoy immersive visuals including rain, snow, thunderstorms, passing clouds, day/night cycles, airplanes, and even rare UFO sightings!
- 🌍 **Auto-Location Detection:** Automatically fetches weather for your current IP address with zero manual setup.
- ⚙️ **Highly Customizable:** Easily switch between metric/imperial units, toggle UI elements, and manage your preferred default layout via a `config.toml`.
- 💻 **Aesthetic Terminal UI:** A clean, minimalist layout that perfectly complements the bold ASCII visuals seamlessly.

---

## 📑 Table of Contents

- [📖 About The Project](#-about-the-project)
- [✨ Key Features](#-key-features)
- [📑 Table of Contents](#-table-of-contents)
- [🚀 Getting Started](#-getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [💻 Usage](#-usage)
  - [Keyboard Controls](#keyboard-controls)
  - [Command Line Overrides](#command-line-overrides)
- [⚙️ Configuration](#️-configuration)
- [📝 Roadmap](#-roadmap)
- [🤝 Contributing](#-contributing)
- [📜 License \& Credits](#-license--credits)

---

## 🚀 Getting Started

Follow these steps to get a local copy up and running on your machine.

### Prerequisites

Ensure you have the Rust package manager (`cargo`) installed. If not, follow the instructions at [rustup.rs](https://rustup.rs/).

### Installation

Clone the repository and install it using Cargo:

```bash
# 1. Clone the repository
git clone https://github.com/toxicbishop/weather.git

# 2. Navigate into the directory
cd weather

# 3. Build and install via Cargo
cargo install --path .
```

*Package Managers:*
- **Windows Winget:** `winget install -i Veirt.weather`
- **Arch Linux:** `yay -S weather`
- **macOS:** `brew install Veirt/veirt/weather`

---

## 💻 Usage 

Simply run the app to see the live weather for your area:
```bash
weather
```

> **Note:** `weather` requires a terminal size of at least `70x20` characters to display its ASCII layouts correctly.

### Keyboard Controls
- **`q`** or **`Q`**: Gracefully quit the application.
- **`Ctrl+C`**: Force exit.

### Command Line Overrides
You can temporarily override your configuration values for a single run using CLI flags:

```bash
# Normal Usage
weather

# Force metric or imperial units
weather --metric
weather --imperial

# UI adjustments
weather --hide-hud
weather --hide-location

# Simulate specific weather conditions (Great for testing!)
weather --simulate rain
weather --simulate snow --night

# Easter Egg Simulation (UFO appears on a clear night)
weather --simulate clear --night

# Simulate rain
weather --simulate rain

# Simulate snow
weather --simulate snow

# Simulate thunderstorm
weather --simulate thunderstorm

# Simulate fog
weather --simulate fog

# Simulate drizzle
weather --simulate drizzle

# Simulate freezing rain
weather --simulate freezing-rain

# Simulate rain showers
weather --simulate rain-showers

# Simulate snow showers
weather --simulate snow-showers

# Simulate snow grains
weather --simulate snow-grains

# Simulate thunderstorm hail
weather --simulate thunderstorm-hail

# Simulate partly cloudy
weather --simulate partly-cloudy

# Simulate cloudy
weather --simulate cloudy

# Simulate overcast
weather --simulate overcast

# Simulate fog
weather --simulate fog

# Simulate drizzle
weather --simulate drizzle

# Simulate freezing rain
weather --simulate freezing-rain

# Simulate rain showers
weather --simulate rain-showers

# Simulate snow showers
weather --simulate snow-showers

# Simulate snow grains
weather --simulate snow-grains

# Simulate thunderstorm hail
weather --simulate thunderstorm-hail

# Simulate partly cloudy
weather --simulate partly-cloudy

# Simulate cloudy
weather --simulate cloudy

# Simulate overcast
weather --simulate overcast

# Simulate fog
weather --simulate fog

# Simulate drizzle
weather --simulate drizzle

# Simulate freezing rain
weather --simulate freezing-rain

# Simulate rain showers
weather --simulate rain-showers

# Simulate snow showers
weather --simulate snow-showers

# Simulate snow grains
weather --simulate snow-grains

# Simulate thunderstorm hail
weather --simulate thunderstorm-hail

# Simulate partly cloudy
weather --simulate partly-cloudy

# Simulate cloudy
weather --simulate cloudy

# Simulate overcast
weather --simulate overcast

# Simulate fog
weather --simulate fog

# Simulate drizzle
weather --simulate drizzle

# Simulate freezing rain
weather --simulate freezing-rain

# Simulate rain showers
weather --simulate rain-showers

# Simulate snow showers
weather --simulate snow-showers

# Simulate snow grains
weather --simulate snow-grains

# Simulate thunderstorm hail
weather --simulate thunderstorm-hail

# Simulate partly cloudy
weather --simulate partly-cloudy

# Simulate cloudy
weather --simulate cloudy

# Simulate overcast
weather --simulate overcast

# Simulate fog
weather --simulate fog

# Simulate drizzle
weather --simulate drizzle

# Simulate freezing rain
weather --simulate freezing-rain

# Simulate rain showers
weather --simulate rain-showers

# Simulate snow showers
weather --simulate snow-showers

# Simulate snow grains
weather --simulate snow-grains

# Simulate thunderstorm hail
weather --simulate thunderstorm-hail

# Simulate partly cloudy
weather --simulate partly-cloudy

# Simulate cloudy
weather --simulate cloudy

# Simulate overcast
weather --simulate overcast

# Simulate fog
weather --simulate fog

# Simulate drizzle
weather --simulate drizzle

# Simulate freezing rain
weather --simulate freezing-rain

# Simulate rain showers
weather --simulate rain-showers

# Simulate snow showers
weather --simulate snow-showers

# Simulate snow grains
weather --simulate snow-grains

# Simulate thunderstorm hail
weather --simulate thunderstorm-hail

# Simulate partly cloudy
weather --simulate partly-cloudy

# Simulate cloudy
weather --simulate cloudy

# Simulate overcast
weather --simulate overcast

# Simulate fog
weather --simulate fog

# Simulate drizzle
weather --simulate drizzle

# Simulate freezing rain
weather --simulate freezing-rain

# Simulate rain showers
weather --simulate rain-showers

# Simulate snow showers
weather --simulate snow-showers

# Simulate snow grains
weather --simulate snow-grains

# Simulate thunderstorm hail
weather --simulate thunderstorm-hail

# Simulate partly cloudy
weather --simulate partly-cloudy

# Simulate cloudy
weather --simulate cloudy

# Simulate overcast
weather --simulate overcast

# Simulate fog
weather --simulate fog

# Simulate drizzle
weather --simulate drizzle

# Simulate freezing rain
weather --simulate freezing-rain

# Simulate rain showers
weather --simulate rain-showers

# Simulate snow showers
weather --simulate snow-showers

# Simulate snow grains
weather --simulate snow-grains

# Simulate thunderstorm hail
weather --simulate thunderstorm-hail

# Simulate partly cloudy
weather --simulate partly-cloudy

# Simulate cloudy
weather --simulate cloudy

# Simulate overcast
weather --simulate overcast

# Simulate fog
weather --simulate fog

# Simulate drizzle
weather --simulate drizzle

# Simulate freezing rain
weather --simulate freezing-rain

# Simulate rain showers
weather --simulate rain-showers

# Simulate snow showers
weather --simulate snow-showers

# Simulate snow grains
weather --simulate snow-grains

# Simulate thunderstorm hail
weather --simulate thunderstorm-hail

# Simulate partly cloudy
weather --simulate partly-cloudy

# Simulate cloudy
weather --simulate cloudy

# Simulate overcast
weather --simulate overcast

# Simulate fog
weather --simulate fog

# Simulate drizzle
weather --simulate drizzle

# Simulate freezing rain
weather --simulate freezing-rain

# Simulate rain showers
weather --simulate rain-showers

# Simulate snow showers
weather --simulate snow-showers

# Simulate snow grains
weather --simulate snow-grains

# Simulate thunderstorm hail
weather --simulate thunderstorm-hail

# Simulate partly cloudy
weather --simulate partly-cloudy

# Simulate cloudy
weather --simulate cloudy

# Simulate overcast
weather --simulate overcast

# Simulate fog
weather --simulate fog

# Simulate drizzle
weather --simulate drizzle

# Simulate freezing rain
weather --simulate freezing-rain

# Simulate rain showers
weather --simulate rain-showers

# Simulate snow showers
weather --simulate snow-showers

# Simulate snow grains
weather --simulate snow-grains

# Simulate thunderstorm hail
weather --simulate thunderstorm-hail

# Simulate partly cloudy
weather --simulate partly-cloudy

# Simulate cloudy
weather --simulate cloudy

# Simulate overcast
weather --simulate overcast

# Simulate fog
weather --simulate fog

# Simulate drizzle
weather --simulate drizzle

# Simulate freezing rain
weather --simulate freezing-rain

# Simulate rain showers
weather --simulate rain-showers

# Simulate snow showers
weather --simulate snow-showers

# Simulate snow grains
weather --simulate snow-grains

# Simulate thunderstorm hail
weather --simulate thunderstorm-hail

# Simulate partly cloudy
weather --simulate partly-cloudy

# Simulate cloudy
weather --simulate cloudy

# Simulate overcast
weather --simulate overcast

# Simulate fog
weather --simulate fog

# Simulate drizzle
weather --simulate
```
## ⚙️ Configuration

`weather` functions beautifully out of the box but can be deeply customized using a `config.toml` file.

**File Locations:**
- **Windows:** `~\AppData\Roaming\weather\config.toml`
- **Linux:** `~/.config/weather/config.toml` 
- **macOS:** `~/Library/Application Support/weather/config.toml`

**Example `config.toml`:**

```toml
# Display Settings
hide_hud = false       # Hide the standard readout interface
silent = false         # Start the app without init messages

# Location Settings
[location]
latitude = 40.7128     # Kept if auto is false
longitude = -74.0060
auto = true            # Automatically fetch coordinate via IP
hide = false           # Hide location name in the top bar

# Unit Preferences
[units]
temperature = "celsius" # Options: "celsius", "fahrenheit"
wind_speed = "kmh"      # Options: "kmh", "ms", "mph", "kn"
precipitation = "mm"    # Options: "mm", "inch"
```

---

## 📝 Roadmap

- [x] Initial release features & auto-location fetching
- [x] GNU GPL-3.0 License migration
- [x] Add dynamic UFO and Airplane animations
- [ ] Support for multiple API keys (OpenWeatherMap, WeatherAPI, etc.)
- [ ] Implement keybindings to pause/speed up animations interactively
- [ ] Additional weather simulation patterns

---

## 🤝 Contributing

Contributions make the open-source community an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## 📜 License & Credits

Distributed under the **GPL-3.0-or-later** License. See [`LICENSE`](LICENSE) for more information.

- **Data source:** [Open-Meteo](https://open-meteo.com/) (CC BY 4.0 license)
- **ASCII Art:** Adapted from original artists at [asciiart.eu](https://www.asciiart.eu/) (including Joan G. Stark, Hayley Jane Wakenshaw, and others). 
