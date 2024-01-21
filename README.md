# Nymeria(WIP)

Currently supports reading microphone usage in MacOS to choose between "Free"(Green) or "Busy"(Red) status.
The goal is to create an intermediate state "Busy"(Yellow) that will read from your calendar and determine if you have something scheduled at the time.

## Goal

Use a traffic lights-like display to indicate if the microphone is being used. The goal is the same
of radio stations' "On air" signs.

## Why ?

Indicate to people wanting to enter your office if you are occupied in a meeting or recording.

## How

[MacOS] Uses [ioreg](https://developer.apple.com/library/archive/documentation/DeviceDrivers/Conceptual/IOKitFundamentals/TheRegistry/TheRegistry.html)
to identify if the microphone is currently being used. That information will be served by an HTTP API written in Typescript.

Uses an [ESP32](https://www.espressif.com/en/products/socs/esp32) to control the sign and request the current status
every 15 seconds.

## Next steps

- [ ] Change MacOS agent to run as an tray application
- [ ] Use environment vars to config esp32's code build
- [ ] Allow manually changing the status in the MacOS app
- [ ] Create configuration section in the tray application
- [ ] Document esp32 connections
- [ ] Support multiple machines
- [ ] Support Linux
- [ ] Create modular integration to calendar (Simple and common interface to allow third-party connectors)
