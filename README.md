# ColorLamp

This repository contains a sample firmware and a sample client app for an hypotetical colored lamp.
The goal of this repository is to showcase two libraries discussed in my Bachelor's Thesis: the [`bluedroid`](https://github.com/persello/bluedroid) crate for ESP32 devices, and the [`CharacteristicKit`](https://github.com/persello/CharacteristicKit) package for iOS/macOS.

This repository is split in two folders:

- `old` contains the implementations of both the firmware and the client software without using libraries.
- `new` contains the same softwares, but implemented using both auxiliary libraries.

|                   | Firmware                     | Client app                        |
| ----------------- | ---------------------------- | --------------------------------- |
| Without libraries | [old/firmware](old/firmware) | [old/ColorLampClient](old/ColorLampClient) |
| With libraries    | [new/firmware](new/firmware) | [new/ColorLampClient](new/ColorLampClient) |

## Bluetooth interface

The Bluetooth LE interface of this lamp is composed of a single service that exposes two characteristics, one for the color (hue, 0-360) and another one for the brightness (0-100). Both characteristics are read/write, and the client can subscribe to notifications for both of them, since the firmware is capable of also reading manual controls from the lamp.

The service is identified by the UUID `4E0F5E1E-FC5B-4D67-8E30-2A83B336476B`, while the characteristics are identified by the UUIDs `CA344E9B-7445-43AA-AD20-43A33C8101E9` (color) and `F9DFBD73-0181-433A-8091-372E0CA8A598` (brightness).

Both characteristics are encoded as 8-bit unsigned integers.

## Firmware

The firmware is written in Rust, and it does three things:

- When a read request is received, it reads the current state of the lamp and sends it back to the client.
- When a write request is received, it updates the current state of the lamp.
- Every 10 seconds, it simulates a manual change of the lamp state, notifying the client.
