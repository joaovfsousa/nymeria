# Nymeria

## Goal

Identify if I'm in a call at the moment and display that in an remote display.
Inspired by Radio Station on-air ligh signs.

## How

Uses [ioreg](https://developer.apple.com/library/archive/documentation/DeviceDrivers/Conceptual/IOKitFundamentals/TheRegistry/TheRegistry.html)
to identify if the microphone is currently being used. That information will be served by an http api written in Typescript.

Uses a [ESP32](https://www.espressif.com/en/products/socs/esp32) to control the sign and request the current status
every 15 seconds.
