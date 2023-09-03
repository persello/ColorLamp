//
//  BluetoothCoordinator.swift
//  ColorLampClient
//
//  Created by Riccardo Persello on 25/08/23.
//

import Foundation
import CoreBluetooth
import OSLog

@Observable
class BluetoothCoordinator: NSObject {

    enum CoordinatorError: Error {
        case peripheralNotFound
        case characteristicNotFound
    }

    static let serviceUUID = CBUUID(string: "4E0F5E1E-FC5B-4D67-8E30-2A83B336476B")
    static let temperatureCharacteristicUUID = CBUUID(string: "CA344E9B-7445-43AA-AD20-43A33C8101E9")
    static let brightnessCharacteristicUUID = CBUUID(string: "F9DFBD73-0181-433A-8091-372E0CA8A598")

    static let logger = Logger(subsystem: "com.persello.ColorLampClient", category: "BluetoothCoordinator")

    private var centralManager: CBCentralManager!

    private var temperatureCharacteristic: CBCharacteristic?
    private var brightnessCharacteristic: CBCharacteristic?

    private var temperatureCallback: ((UInt8) -> Void)?
    private var brightnessCallback: ((UInt8) -> Void)?
    
    private var peripheral: CBPeripheral?

    var discoveredPeripherals: Set<CBPeripheral> = Set()
    var connected: Bool {
        peripheral != nil
    }

    override init() {
        super.init()
        self.centralManager = CBCentralManager(delegate: self, queue: .global(qos: .userInteractive))
    }

    // MARK: BluetoothCoordinator functions.

    func scan() {
        self.centralManager.scanForPeripherals(withServices: [Self.serviceUUID])
    }

    func connect(to peripheral: CBPeripheral) {
        Self.logger.info("Connecting to \(peripheral.name ?? "unknown peripheral") [\(peripheral.identifier)].")
        self.centralManager.connect(peripheral)
    }

    func setBrightness(_ brightness: Float) throws {
        guard let peripheral else {
            throw CoordinatorError.peripheralNotFound
        }

        guard let brightnessCharacteristic else {
            self.peripheral = nil
            throw CoordinatorError.characteristicNotFound
        }

        var integerBrightness = UInt8(brightness)
        let data = withUnsafeBytes(of: &integerBrightness) { pointer in
            Data(pointer)
        }

        peripheral.writeValue(data, for: brightnessCharacteristic, type: .withoutResponse)
    }

    func setTemperature(_ temperature: Float) throws {
        guard let peripheral else {
            throw CoordinatorError.peripheralNotFound
        }

        guard let temperatureCharacteristic else {
            self.peripheral = nil
            throw CoordinatorError.characteristicNotFound
        }

        var integerTemperature = UInt8(temperature)
        let data = withUnsafeBytes(of: &integerTemperature) { pointer in
            Data(pointer)
        }

        peripheral.writeValue(data, for: temperatureCharacteristic, type: .withoutResponse)
    }

    func onTemperatureChange(_ callback: @escaping (UInt8) -> Void) {
        self.temperatureCallback = callback
    }

    func onBrightnessChange(_ callback: @escaping (UInt8) -> Void) {
        self.brightnessCallback = callback
    }
}

// MARK: CBCentralManagerDelegate conformance.

extension BluetoothCoordinator: CBCentralManagerDelegate {
    func centralManagerDidUpdateState(_ central: CBCentralManager) {
        Self.logger.info("Central state updated: \(String(describing: central.state)).")
    }

    func centralManager(_ central: CBCentralManager, didDiscover peripheral: CBPeripheral, advertisementData: [String : Any], rssi RSSI: NSNumber) {
        // On discovery, update the set of available peripherals.

        self.discoveredPeripherals.insert(peripheral)

        Self.logger.info("Discovered \(peripheral.name ?? "unknown peripheral") [\(peripheral.identifier)].")
    }

    func centralManager(_ central: CBCentralManager, didConnect peripheral: CBPeripheral) {
        // On connection, discover the peripheral's services.

        Self.logger.info("Discovering Color Lamp Service on \(peripheral.name ?? "unknown peripheral") [\(peripheral.identifier)].")

        peripheral.delegate = self
        peripheral.discoverServices([Self.serviceUUID])

        // Store the peripheral
        self.peripheral = peripheral
    }

    func centralManager(_ central: CBCentralManager, didDisconnectPeripheral peripheral: CBPeripheral, error: Error?) {
        if let error {
            Self.logger.warning("Disconnected from \(peripheral.name ?? "unknown peripheral") [\(peripheral.identifier)]: \(error.localizedDescription).")
        }

        // Remove reference to the peripheral.
        self.peripheral = nil
    }
    
    func centralManager(_ central: CBCentralManager, didDisconnectPeripheral peripheral: CBPeripheral, timestamp: CFAbsoluteTime, isReconnecting: Bool, error: Error?) {
        if let error {
            Self.logger.warning("Disconnected from \(peripheral.name ?? "unknown peripheral") [\(peripheral.identifier)]: \(error.localizedDescription).")
        }

        // Remove reference to the peripheral.
        self.peripheral = nil
    }
}

// MARK: CBPeripheralDelegate conformance.

extension BluetoothCoordinator: CBPeripheralDelegate {
    func peripheral(_ peripheral: CBPeripheral, didDiscoverServices error: Error?) {
        if let error {
            Self.logger.warning("Error during service discovery for \(peripheral.name ?? "unknown peripheral") [\(peripheral.identifier)]: \(error.localizedDescription).")
            return
        }

        guard let services = peripheral.services,
              let colorLampService = services.first(where: { $0.uuid == Self.serviceUUID }) else {
            Self.logger.warning("Color Lamp service not found in \(peripheral.name ?? "unknown peripheral") [\(peripheral.identifier)].")
            return
        }

        // If the Color Lamp service is discovered, store it.
        peripheral.discoverCharacteristics([Self.brightnessCharacteristicUUID, Self.temperatureCharacteristicUUID], for: colorLampService)
    }

    func peripheral(_ peripheral: CBPeripheral, didDiscoverCharacteristicsFor service: CBService, error: Error?) {
        if let error {
            Self.logger.warning("Error during characteristics discovery for \(peripheral.name ?? "unknown peripheral") [\(peripheral.identifier)]: \(error.localizedDescription).")
            return
        }

        // If a known characteristic is discovered, store it.

        if let characteristic = service.characteristics?.first(where: { $0.uuid == Self.brightnessCharacteristicUUID }) {
            Self.logger.info("Found brightness characteristic in \(service) in \(peripheral.name ?? "unknown peripheral") [\(peripheral.identifier)].")
            self.brightnessCharacteristic = characteristic

            // Subscribe to the brightness characteristic.
            peripheral.setNotifyValue(true, for: characteristic)
            peripheral.readValue(for: characteristic)
        }

        if let characteristic = service.characteristics?.first(where: { $0.uuid == Self.temperatureCharacteristicUUID }) {
            Self.logger.info("Found temperature characteristic in \(service) in \(peripheral.name ?? "unknown peripheral") [\(peripheral.identifier)].")
            self.temperatureCharacteristic = characteristic

            // Subscribe to the temperature characteristic.
            peripheral.setNotifyValue(true, for: characteristic)
            peripheral.readValue(for: characteristic)
        }
    }

    func peripheral(_ peripheral: CBPeripheral, didUpdateValueFor characteristic: CBCharacteristic, error: Error?) {
        if let error {
            Self.logger.warning("Error during update for characteristic \(characteristic.uuid) in \(peripheral.name ?? "unknown peripheral") [\(peripheral.identifier)]: \(error.localizedDescription).")
            return
        }

        if characteristic.uuid == Self.brightnessCharacteristicUUID {
            if let value = characteristic.value?.withUnsafeBytes({ pointer in
                pointer.load(as: UInt8.self)
            }) {
                self.brightnessCallback?(value)
            }
        } else if characteristic.uuid == Self.temperatureCharacteristicUUID {
            if let value = characteristic.value?.withUnsafeBytes({ pointer in
                pointer.load(as: UInt8.self)
            }) {
                self.temperatureCallback?(value)
            }
        }
    }
}
