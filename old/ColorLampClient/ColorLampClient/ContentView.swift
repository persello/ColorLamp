//
//  ContentView.swift
//  ColorLampClient
//
//  Created by Riccardo Persello on 21/08/23.
//

import SwiftUI

struct ContentView: View {
    
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
    
    @State private var brightness: Float = 0.0
    @State private var temperature: Float = 0.0
    private var coordinator = BluetoothCoordinator()
    
    var body: some View {
        VStack(alignment: .leading) {
            
            Text("Devices")
                .foregroundStyle(.secondary)
                .fontWeight(.semibold)
            
            List {
                ForEach(
                    Array(
                        coordinator.discoveredPeripherals),
                    id: \.identifier
                ) { peripheral in
                    Button(peripheral.name ?? "Unknown peripheral") {
                        coordinator.connect(to: peripheral)
                    }
                }
            }
            .clipShape(RoundedRectangle(cornerRadius: 16))
            .padding(.bottom, 36)
            
            Group {
                CustomSliderView(
                    value: $temperature,
                    fillGradient: colorGradient
                )
                .frame(height: 60)
                .padding(.vertical, 8)
                
                CustomSliderView(
                    value: $brightness,
                    fillGradient: brightnessGradient,
                    startIcon: Image(systemName: "sun.min"),
                    endIcon: Image(systemName: "sun.max")
                )
                .frame(height: 60)
                .padding(.vertical, 8)
            }
            .disabled(!coordinator.connected)
        }
        .onAppear {
            coordinator.scan()
            coordinator.onBrightnessChange { brightness in
                self.brightness = Float(brightness) / 255.0
            }
            coordinator.onTemperatureChange { temperature in
                self.temperature = Float(temperature) / 255.0
            }
        }
        .onChange(of: brightness, { oldValue, newValue in
            guard oldValue != newValue else { return }
            try? coordinator.setBrightness(newValue * 255.0)
        })
        .onChange(of: temperature, { oldValue, newValue in
            guard oldValue != newValue else { return }
            try? coordinator.setTemperature(newValue * 255.0)
        })
        .padding()
    }
}

#Preview {
    NavigationStack {
        ContentView()
            .navigationTitle("Color Lamp")
    }
}
