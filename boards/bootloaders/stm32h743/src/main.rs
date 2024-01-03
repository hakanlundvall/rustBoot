#![no_std]
#![no_main]


#[cfg(feature = "defmt")]
use defmt_rtt as _; // global logger
// use defmt::info;

use stm32h7xx_hal::{pac, prelude::*};

use rustBoot_hal::stm::stm32h743::FlashWriterEraser;
use rustBoot_update::update::{update_flash::FlashUpdater, UpdateInterface};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    
    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    //GPIO init
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

    // Configure PE1 as output.
    let mut led1 = gpiob.pb0.into_push_pull_output();

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    let mut count = 0;

    if cfg!(feature = "defmt") {
        while count < 2 {
            led1.set_high();
            delay.delay_ms(500_u16);
            led1.set_low();
            delay.delay_ms(500_u16);
            count = count + 1;
        }
    }

    let flsh = dp.FLASH;
    while count < 6 {
        led1.set_high();
        delay.delay_ms(200_u16);
        led1.set_low();
        delay.delay_ms(200_u16);
        count = count + 1;
    }
    let flash_writer = FlashWriterEraser { nvm: flsh };
    while count < 8 {
        led1.set_high();
        delay.delay_ms(200_u16);
        led1.set_low();
        delay.delay_ms(200_u16);
        count = count + 1;
    }
    let updater = FlashUpdater::new(flash_writer);

    while count < 10 {
        led1.set_high();
        delay.delay_ms(200_u16);
        led1.set_low();
        delay.delay_ms(200_u16);
        count = count + 1;
    }

    updater.rustboot_start();

}

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
