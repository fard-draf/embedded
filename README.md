# ⛵ Dinghy Sail Tools & Embedded Rust Playground

This repository is a collection of my projects in **embedded Rust**, focusing on low-level programming and microcontroller development. My goal is to build expertise through a hands-on, modular approach, tackling small-scale projects before moving on to more ambitious challenges. All firmware development within this repository is done in a **`no_std`** environment.

---

## 🧭 Dinghy Sail Tools: A Race GPS Display

This project contains the firmware for a GPS display and logger designed for dinghy sailing races. The main objective is to create a reliable and resource-efficient device that provides sailors with essential real-time data.

### Current Status

The project is in its early stages. The primary focus is on laying a solid architectural foundation and implementing core functionalities.

**Currently Implemented:**
- **NMEA Sentence Parsing**: The firmware can successfully parse NMEA data from the GPS module.
- **Display Output**: The parsed data is formatted and displayed on a small screen.

### Key Features (Planned & In Progress)

- **`no_std` Firmware**: The entire firmware is built without the standard library, ensuring maximum reliability, minimal binary size, and precise control over memory and power consumption.
- **Real-time Data Display**: The device will show crucial sailing metrics such as speed, heading, and GPS coordinates.
- **Configurable UI via WASM**: The display layout will be dynamically customized through a **WebAssembly** interface. This is the core technical challenge of the project, exploring WASM as a powerful tool for embedded UI configuration within a `no_std` context.
- **Modular Architecture**: The firmware is built with a flexible architecture to easily integrate new sensors and display types, ensuring a scalable and maintainable codebase.

### Hardware

- **Microcontroller**: ESP32
- **GPS Module**: NEO-8M
- **Display**: Sharp Memory LCD
- **Power**: 18650 Li-ion battery, with a target autonomy of 6 hours.

*Note: While the primary focus is on the software architecture and WASM integration, future iterations may address the challenge of waterproofing for a final prototype.*
