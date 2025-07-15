# Rust Embedded Systems Laboratory

This repository documents my learning journey into embedded systems programming with Rust. It's a career change project where I'm applying a structured approach to transfer my ten years of experience in the maritime industry to the technology field.

The main goal is to become proficient in Rust and its `#[no_std]` ecosystem by building hands-on projects.

## Project Organization

The code is organized as a **Cargo Workspace** to clearly separate reusable software components from the projects that use them.

* **`/drivers`**: Contains reusable library `crates`. The goal here is to develop drivers for various sensors and peripherals.

* **`/applications`**: Contains executable projects that run on specific hardware targets (e.g., ESP32). These applications are used to integrate and test the drivers in a real-world context.

## Learning Objectives

Through this lab, I am focusing on the following points:

* Mastering Rust in a `#[no_std]` context.
* Understanding and implementing drivers for communication buses (I2C, SPI, UART).
* Exploring how to apply these skills to maritime-specific problems, such as processing NMEA data.
* Learning software development best practices (testing, documentation, version control).

This repository is a work in progress and will evolve as I learn.

## License

Distributed under the dual MIT and Apache 2.0 licenses. See `LICENSE-MIT` and `LICENSE-APACHE` files for more details.
