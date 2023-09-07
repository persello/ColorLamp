//
//  CustomSliderView.swift
//  ColorLampClient
//
//  Created by Riccardo Persello on 21/08/23.
//

import SwiftUI

struct CustomSliderView: View {
    var scale: UInt8 = 255
    @Binding var value: UInt8
    @State var lastDragStart: CGFloat = 0

    @Environment(\.isEnabled) var isEnabled

    var fillGradient: LinearGradient = .init(colors: [.gray.opacity(0.4)], startPoint: .leading, endPoint: .trailing)

    var startIcon: Image? = nil
    var endIcon: Image? = nil

    var body: some View {
        GeometryReader { proxy in
            RoundedRectangle(cornerRadius: .infinity, style: .circular)
                .foregroundStyle(self.fillGradient.shadow(.inner(color: .black.opacity(0.3), radius: 2, y: 1)))
                .opacity(isEnabled ? 1.0 : 0.5)
                .overlay {
                    HStack {
                        if let startIcon {
                            startIcon
                                .resizable()
                                .scaledToFit()
                                .padding(16)
                        }
                        Spacer()
                        if let endIcon {
                            endIcon
                                .resizable()
                                .scaledToFit()
                                .padding(16)
                        }
                    }
                    .foregroundStyle(.secondary)
                }
                .overlay(alignment: .leading) {
                    Circle()
                        .gesture(
                            DragGesture(minimumDistance: 0, coordinateSpace: .global)
                                .onChanged({ value in
                                    let delta = (value.translation.width - lastDragStart) * CGFloat(self.scale) / (proxy.size.width - proxy.size.height)
                                    
                                    lastDragStart = value.translation.width
                                    
                                    print(self.value)
                                    
                                    if delta < 0 {
                                        if abs(delta) > CGFloat(self.value) {
                                            self.value = 0
                                        } else {
                                            self.value -= UInt8(abs(delta))
                                        }
                                    } else {
                                        if abs(delta) > CGFloat(self.scale - self.value) {
                                            self.value = self.scale
                                        } else {
                                            self.value += UInt8(delta)
                                        }
                                    }
                                })
                                .onEnded({ _ in
                                    lastDragStart = 0
                                })
                        )
                        .padding(4)
                        .offset(x: CGFloat(self.value) / CGFloat(self.scale) * (proxy.size.width - proxy.size.height))
                        .foregroundStyle(.white.gradient)
                        .opacity(isEnabled ? 1.0 : 0.5)
                        .shadow(radius: 2, y: 1)
                }
        }
    }
}

#Preview {
    let colorGradient = LinearGradient(
        gradient: Gradient(
            colors: [
                .init(hue: 0, saturation: 0.7, brightness: 1),
                .init(hue: 60.0/360.0, saturation: 0.7, brightness: 1),
                .init(hue: 120.0/360.0, saturation: 0.7, brightness: 1),
                .init(hue: 180.0/360.0, saturation: 0.7, brightness: 1),
                .init(hue: 240.0/360.0, saturation: 0.7, brightness: 1),
                .init(hue: 300.0/360.0, saturation: 0.7, brightness: 1),
                .init(hue: 1, saturation: 0.7, brightness: 1),
            ]
        ),
        startPoint: .leading,
        endPoint: .trailing
    )

    let brightnessGradient = LinearGradient(colors: [
        .gray,
        .white
    ], startPoint: .leading, endPoint: .trailing)

    var value: UInt8 = 127

    let binding = Binding<UInt8> {
        return value
    } set: { new in
        value = new
    }

    return Group {
        CustomSliderView(value: binding, fillGradient: colorGradient)
            .frame(width: 480, height: 60)

        CustomSliderView(value: binding, fillGradient: brightnessGradient, startIcon: Image(systemName: "sun.min"), endIcon: Image(systemName: "sun.max"))
            .frame(width: 480, height: 60)

        CustomSliderView(value: binding, fillGradient: brightnessGradient, startIcon: Image(systemName: "sun.min"), endIcon: Image(systemName: "sun.max"))
            .frame(width: 480, height: 60)
            .disabled(true)
    }
}
