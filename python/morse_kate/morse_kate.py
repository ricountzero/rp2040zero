import machine, neopixel
from time import sleep

LEDpin = 16
LED_PURPLE = (128, 0, 128)
DOT_DURATION = 0.2
DASH_DURATION = 0.6
SPACE_DURATION = 0.2
LETTER_SPACE_DURATION = 0.6

morse_code = {
    'K': '-.-',
    'A': '.-',
    'T': '-',
    'E': '.'
}

led = neopixel.NeoPixel(machine.Pin(LEDpin), 1)

def set_led_color(color):
    led[0] = color
    led.write()

def sleep_duration(duration):
    sleep(duration)

def turn_on_led():
    set_led_color(LED_PURPLE)

def turn_off_led():
    set_led_color((0, 0, 0))

def blink(duration):
    turn_on_led()
    sleep_duration(duration)
    turn_off_led()
    sleep_duration(SPACE_DURATION)

def get_morse_symbols(letter):
    return morse_code[letter]

def process_symbol(symbol):
    duration = DOT_DURATION if symbol == '.' else DASH_DURATION
    blink(duration)

def process_letter(letter):
    symbols = get_morse_symbols(letter)
    for symbol in symbols:
        process_symbol(symbol)
    turn_off_led()
    sleep_duration(LETTER_SPACE_DURATION)

def main():
    while True:
        for letter in "KATE":
            process_letter(letter)

if __name__ == "__main__":
    main()
