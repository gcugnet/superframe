#![no_std]
#![no_main]

use panic_rtt_target as _;

use rtic::app;

use rtt_target::{rprintln, rtt_init_print};
use stm32l4xx_hal::gpio::{Alternate, Floating, Input, AF5, PB10, PC2, PC3};
use stm32l4xx_hal::pac::SPI2;
use stm32l4xx_hal::prelude::*;
use stm32l4xx_hal::spi::Spi;

use ws2812_spi::Ws2812;

use smart_leds::{colors::*, SmartLedsWrite, RGB8};

// on utilise un SPI2
// la broche qui mintéresse : MOSI
// une MOSI trouvée sur la Pin PC3 (connecteur 37)
// il faut aussi trouver un MISO et SCK
// MISO du SPI2 -> en PC2
// SCK du SPI2 -> PB10

#[app(device = stm32l4xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        ws2812b: Ws2812<
            stm32l4xx_hal::spi::Spi<
                SPI2,
                (
                    PB10<Alternate<AF5, Input<Floating>>>,
                    PC2<Alternate<AF5, Input<Floating>>>,
                    PC3<Alternate<AF5, Input<Floating>>>,
                ),
            >,
        >,
    }

    #[init(spawn = [leds_on])]
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
        let clocks = rcc.cfgr.hclk(4.mhz()).freeze(&mut flash.acr, &mut pwr);

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
        let ws2812b = Ws2812::new(spi);

        cx.spawn.leds_on().unwrap();

        init::LateResources { ws2812b }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }

    #[task(resources = [ws2812b])]
    fn leds_on(cx: leds_on::Context) {
        let ws2812b = cx.resources.ws2812b;
        let leds_state: [RGB8; 3] = [DEEP_PINK; 3];
        ws2812b.write(leds_state.iter().cloned()).unwrap();
    }

    extern "C" {
        fn TIM2();
    }
};
