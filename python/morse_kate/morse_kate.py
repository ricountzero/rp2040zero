import machine, neopixel
from time import sleep

morse_code = {
    'K': '-.-',
    'A': '.-',
    'T': '-',
    'E': '.'
}

LEDpin = 16
LED_PURPLE = (128, 0, 128)
DOT_DURATION = 0.2
DASH_DURATION = 0.6
SPACE_DURATION = 0.2
LETTER_SPACE_DURATION = 0.6

led = neopixel.NeoPixel(machine.Pin(LEDpin), 1)

def blink(duration):
    led[0] = LED_PURPLE
    led.write()
    sleep(duration)
    led[0] = (0, 0, 0)
    led.write()
    sleep(SPACE_DURATION)

def blink_morse(letter):
    for symbol in morse_code[letter]:
        if symbol == '.':
            blink(DOT_DURATION)
        elif symbol == '-':
            blink(DASH_DURATION)

while True:
    for letter in "KATE":
        blink_morse(letter)
        led[0] = (0, 0, 0)
        led.write()
        sleep(LETTER_SPACE_DURATION)