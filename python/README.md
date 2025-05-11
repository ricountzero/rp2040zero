# RP2040 Zero Morse Code Project

This project demonstrates Morse code communication using an RP2040 Zero microcontroller. The program displays the word "KATE" in Morse code using the built-in NeoPixel LED.

## Prerequisites

- RP2040 Zero microcontroller
- Thonny IDE installed on your computer
- USB cable to connect the RP2040 Zero to your computer

## Setup Instructions

1. **Prepare the Microcontroller**
   - Connect your RP2040 Zero to your computer using a USB cable
   - Hold the BOOTSEL button while connecting the USB cable to enter bootloader mode
   - Copy the `RPI_PICO-20250415-v1.25.0.uf2` file to the microcontroller's drive
   - Wait for the file to be copied and the microcontroller to restart

2. **Install Required Libraries**
   - Open Thonny IDE
   - Go to Tools → Options → Interpreter
   - Select "MicroPython (Raspberry Pi Pico)" as the interpreter
   - Install the required libraries:
     - `neopixel` (should be included in the firmware)

3. **Upload and Run the Code**
   - Open `morse_kate.py` in Thonny IDE
   - Click the "Save" button and select "Raspberry Pi Pico" as the location
   - Click the "Run" button (green play icon) to execute the code

## Troubleshooting

If the LED doesn't light up:
1. Check that the RP2040 Zero is properly connected
2. Verify that the correct interpreter is selected in Thonny
3. Make sure the code was saved to the Pico
4. Try resetting the microcontroller by unplugging and replugging the USB cable