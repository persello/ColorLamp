//
//  ColorLampPeripheral.swift
//  ColorLampClient
//
//  Created by Riccardo Persello on 06/09/23.
//

import Foundation
import CoreBluetooth
import CharacteristicKit
import Combine

final class ColorLampPeripheral: PeripheralModel {
    static var requiredAdvertisedServices: [CBUUID]? = [CBUUID(string: "4E0F5E1E-FC5B-4D67-8E30-2A83B336476B")]
    static var servicesToScan: [CBUUID]? = [CBUUID(string: "4E0F5E1E-FC5B-4D67-8E30-2A83B336476B")]
    static var centralManager: CBCentralManager?
    static var centralManagerDelegate: CBCentralManagerDelegate?
    
    var delegate: CharacteristicKit.PeripheralDelegate<ColorLampPeripheral>?
    var peripheral: CBPeripheral
    var valueChangeCancellable: AnyCancellable?

    required init(from peripheral: CBPeripheral) {
        self.peripheral = peripheral
        self.initialiseDelegate()
    }
    
    var temperature: Characteristic<UInt8> = Characteristic(initialValue: 0, uuid: CBUUID(string: "CA344E9B-7445-43AA-AD20-43A33C8101E9"))
    var brightness: Characteristic<UInt8> = Characteristic(initialValue: 0, uuid: CBUUID(string: "F9DFBD73-0181-433A-8091-372E0CA8A598"))
    
    var connected: Bool {
        self.peripheral.state == .connected
    }
    
    var name: String {
        self.peripheral.name ?? "Unknown peripheral"
    }
}
