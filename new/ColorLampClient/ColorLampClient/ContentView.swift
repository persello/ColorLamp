//
//  ContentView.swift
//  ColorLampClient
//
//  Created by Riccardo Persello on 21/08/23.
//

import SwiftUI

struct ContentView: View {
    @State private var peripheral: ColorLampPeripheral?
    @State private var discoveredPeripherals: [ColorLampPeripheral] = []
    
    var body: some View {
        VStack(alignment: .leading) {
            
            Text("Devices")
                .foregroundStyle(.secondary)
                .fontWeight(.semibold)
            
            List {
                ForEach(discoveredPeripherals) { peripheral in
                    Button(peripheral.name) {
                        self.peripheral = peripheral
                        peripheral.connect()
                    }
                }
            }
            .clipShape(RoundedRectangle(cornerRadius: 16))
            .padding(.bottom, 36)
            
            if let peripheral {
                SlidersView(peripheral: peripheral)
            }
        }
        .task {
            if let stream = ColorLampPeripheral.discover(removeAfter: 5) {
                for await discovered in stream {
                    self.discoveredPeripherals = discovered
                }
            }
        }
        .padding()
    }
}

#Preview {
    NavigationStack {
        ContentView()
            .navigationTitle("Color Lamp")
    }
}

struct SlidersView: View {
    @ObservedObject var peripheral: ColorLampPeripheral
    
    let colorGradient = LinearGradient(
        gradient: Gradient(
            colors: [
                .init(hue: 10.0/360.0, saturation: 0.7, brightness: 1),
                .init(hue: 60.0/360.0, saturation: 0.5, brightness: 1),
                .init(hue: 60.0/360.0, saturation: 0.0, brightness: 1),
                .init(hue: 210.0/360.0, saturation: 0.4, brightness: 1),
            ]
        ),
        startPoint: .leading,
        endPoint: .trailing
    )
    
    let brightnessGradient = LinearGradient(colors: [
        .gray,
        .white
    ], startPoint: .leading, endPoint: .trailing)
    
    var body: some View {
        Group {
            CustomSliderView(
                value: $peripheral.temperature.value,
                fillGradient: colorGradient
            )
            .frame(height: 60)
            .padding(.vertical, 8)
            
            CustomSliderView(
                value: $peripheral.brightness.value,
                fillGradient: brightnessGradient,
                startIcon: Image(systemName: "sun.min"),
                endIcon: Image(systemName: "sun.max")
            )
            .frame(height: 60)
            .padding(.vertical, 8)
        }
        .disabled(!peripheral.connected)
    }
}
