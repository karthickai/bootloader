#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_halt;

use nb::block;

use cortex_m_rt::entry;
use stm32f7xx_hal::{delay::Delay, device, prelude::*, serial::{self, Serial},};
use cortex_m_semihosting::hprintln;


#[entry]
fn main() -> ! {
    let p = device::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Constrain clocking registers
    let rcc = p.RCC.constrain();

    // Configure clock and freeze it
    let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();

    // Get delay provider
    let mut delay = Delay::new(cp.SYST, clocks);

    hprintln!("Welcome to Rust STM32 Cortex-M Bootloader Project").unwrap();
    let gpiob = p.GPIOB.split();
    let mut led1 = gpiob.pb0.into_push_pull_output();
    let gpioc = p.GPIOC.split();
    let btn = gpioc.pc13.into_floating_input();
            match btn.is_high() {
            Ok(true) => {
                led1.set_high().expect("GPIO can never fail");
                hprintln!("BOOTLOADER Started").unwrap();
            },
            Ok(false) => {
                led1.set_low().expect("GPIO can never fail");
                hprintln!("User Application Started").unwrap();                 
    
            },
            _ => unreachable!(),
        };


    // hprintln!("UART INITIALISATION").unwrap();

    let gpiod = p.GPIOD.split();
    let tx = gpiod.pd5.into_alternate_af7();
    let rx = gpiod.pd6.into_alternate_af7();

    let serial = Serial::new(
        p.USART2,
        (tx, rx),
        clocks,
        serial::Config {
            baud_rate: 115_200.bps(),
            oversampling: serial::Oversampling::By8,
        },
    );

    let (mut tx, mut rx) = serial.split();

    loop {

        let received = block!(rx.read()).unwrap_or('E' as u8);
        block!(tx.write(received)).ok();

    }
