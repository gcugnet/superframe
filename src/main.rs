#![no_std]
#![no_main]

mod chaser;
mod sequence;

use panic_rtt_target as _;

use rtic::app;
use rtic::cyccnt::{Instant, U32Ext as _};

use rtt_target::{rprintln, rtt_init_print};
use stm32l4xx_hal::gpio::{Alternate, Floating, Input, AF5, PB10, PC2, PC3};
use stm32l4xx_hal::pac::SPI2;
use stm32l4xx_hal::prelude::*;
use stm32l4xx_hal::spi::Spi;

use smart_leds::{brightness, colors::*, hsv::Hsv, SmartLedsWrite, RGB8};
use ws2812_spi::prerendered::Ws2812;

use chaser::RainbowChaser;
use sequence::{Gradient, Rainbow, Unicolor};

// on utilise un SPI2
// la broche qui mintéresse : MOSI
// une MOSI trouvée sur la Pin PC3 (connecteur 37)
// il faut aussi trouver un MISO et SCLK
// MISO du SPI2 -> en PC2
// SCLK du SPI2 -> PB10 (connecteur 25 sur CN10)

type Spi2 = stm32l4xx_hal::spi::Spi<
    SPI2,
    (
        PB10<Alternate<AF5, Input<Floating>>>,
        PC2<Alternate<AF5, Input<Floating>>>,
        PC3<Alternate<AF5, Input<Floating>>>,
    ),
>;

const NUM_LEDS: usize = 35;
const BUFFER_SIZE: usize = NUM_LEDS * 12 + 20;

#[app(device = stm32l4xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        #[init([0; BUFFER_SIZE])]
        led_buffer: [u8; BUFFER_SIZE],
        ws2812b: Ws2812<'static, Spi2>,
        chaser: RainbowChaser<Rainbow>,
    }

    #[init(schedule = [next_sequence], resources = [led_buffer])]
    fn init(cx: init::Context) -> init::LateResources {
        rtt_init_print!();
        rprintln!("superframe starting...");

        let mut cp = cx.core;
        let dp = cx.device;

        // Enable the monotonic counter.
        cp.DCB.enable_trace();
        cp.DWT.enable_cycle_counter();

        // Configure the clocks.
        let mut rcc = dp.RCC.constrain();
        let mut flash = dp.FLASH.constrain();
        let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);
        let clocks = rcc.cfgr.sysclk(80.mhz()).freeze(&mut flash.acr, &mut pwr);

        // Configure the SPI. We use SPI2, configured through the pins PC2, PC3, PB10.
        let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
        let mut gpioc = dp.GPIOC.split(&mut rcc.ahb2);

        let sck = gpiob.pb10.into_af5(&mut gpiob.moder, &mut gpiob.afrh);
        let miso = gpioc.pc2.into_af5(&mut gpioc.moder, &mut gpioc.afrl);
        let mosi = gpioc.pc3.into_af5(&mut gpioc.moder, &mut gpioc.afrl);

        let spi = Spi::spi2(
            dp.SPI2,
            (sck, miso, mosi),
            ws2812_spi::MODE,
            3.mhz(),
            clocks,
            &mut rcc.apb1r1,
        );

        // Configure the LED strip driver (pilote).
        let ws2812b = Ws2812::new(spi, cx.resources.led_buffer);

        // Configure the chaser.
        let chaser = RainbowChaser::new(ORANGE, NUM_LEDS, 200);

        cx.schedule.next_sequence(cx.start).unwrap();

        init::LateResources { ws2812b, chaser }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }

    #[task(resources = [ws2812b, chaser], schedule = [next_sequence])]
    fn next_sequence(cx: next_sequence::Context) {
        let ws2812b = cx.resources.ws2812b;
        if let Some(sequence) = cx.resources.chaser.next() {
            cx.schedule
                .next_sequence(Instant::now() + 3_200_000.cycles())
                .unwrap();

            ws2812b.write(brightness(sequence, 70)).unwrap();
        }
    }

    extern "C" {
        fn TIM2();
    }
};
