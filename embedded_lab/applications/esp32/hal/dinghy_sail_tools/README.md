## 🧭 Dinghy Sail Tools: A Race GPS Display

This project contains the firmware for a GPS display and logger designed for dinghy sailing races. The main objective>

### Current Status

The project is in its early stages. The primary focus is on laying a solid architectural foundation and implementing >

**Currently Implemented:**
- **NMEA Sentence Parsing**: The firmware can successfully parse NMEA data from the GPS module.
- **Display Output**: The parsed data is formatted and displayed on a small screen.

### Key Features (Planned & In Progress)

- **`no_std` Firmware**: The entire firmware is built without the standard library, ensuring maximum reliability, min>
- **Real-time Data Display**: The device will show crucial sailing metrics such as speed, heading, and GPS coordinate>
- **Configurable UI via WASM**: The display layout will be dynamically customized through a **WebAssembly** interface>
- **Modular Architecture**: The firmware is built with a flexible architecture to easily integrate new sensors and di>

### Hardware

- **Microcontroller**: ESP32
- **GPS Module**: NEO-8M
- **Display**: Sharp Memory LCD
- **Power**: 18650 Li-ion battery, with a target autonomy of 6 hours.

*Note: While the primary focus is on the software architecture and WASM integration, future iterations may address th>








