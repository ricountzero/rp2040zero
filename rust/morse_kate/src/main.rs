#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use embassy_time::{Duration, Timer};
use smart_leds::RGB8;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

const LED_PURPLE: RGB8 = RGB8::new(128, 0, 128);
const DOT_DURATION_MS: u64 = 200;
const DASH_DURATION_MS: u64 = 600;
const SPACE_DURATION_MS: u64 = 200;
const LETTER_SPACE_DURATION_MS: u64 = 600;

const MORSE_CODE: &[(char, &str)] = &[
    ('K', "-.-"),
    ('A', ".-"),
    ('T', "-"),
    ('E', "."),
];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let Pio { mut common, sm0, .. } = Pio::new(p.PIO0, Irqs);

    let mut data = [RGB8::default()];
    let program = PioWs2812Program::new(&mut common);
    let mut ws2812 = PioWs2812::new(&mut common, sm0, p.DMA_CH0, p.PIN_16, &program);

    loop {
        for letter in "KATE".chars() {
            if let Some((_, pattern)) = MORSE_CODE.iter().find(|(c, _)| *c == letter) {
                for symbol in pattern.chars() {
                    match symbol {
                        '.' => blink(&mut ws2812, &mut data, DOT_DURATION_MS).await,
                        '-' => blink(&mut ws2812, &mut data, DASH_DURATION_MS).await,
                        _ => {}
                    }
                }
            }
            // Letter space
            data[0] = RGB8::new(0, 0, 0);
            ws2812.write(&data).await;
            Timer::after(Duration::from_millis(LETTER_SPACE_DURATION_MS)).await;
        }
    }
}

async fn blink(ws2812: &mut PioWs2812<'_, PIO0, 0, 1>, data: &mut [RGB8; 1], duration_ms: u64) {
    data[0] = LED_PURPLE;
    ws2812.write(data).await;
    Timer::after(Duration::from_millis(duration_ms)).await;
    data[0] = RGB8::new(0, 0, 0);
    ws2812.write(data).await;
    Timer::after(Duration::from_millis(SPACE_DURATION_MS)).await;
}

