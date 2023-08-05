#![no_std]
#![no_main]

pub mod key_map;

// The macro for our start-up function
use rp_pico::entry;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Pull in any important traits
use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::{self, pac};

// Import the Timer for Ws2812:
use rp_pico::hal::timer::Timer;
use hal::i2c::I2C;
use hal::gpio::PinState;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use kallisto_components::keyboard::key_matrix::KeyMatrix;
use kallisto_components::keyboard::keyboard::LayeredKeyboard;
use kallisto_components::keyboard::types::*;
use crate::key_map::*;

use embedded_hal::digital::v2::{InputPin, OutputPin};
use embedded_hal::timer::CountDown;
use embedded_hal::blocking::i2c::Read;

use heapless::spsc::Queue;
use usb_device::class_prelude::*;
use fugit::ExtU32;
use fugit::RateExtU32;

use usbd_human_interface_device as hid;
use hid::page::Keyboard;
use hid::prelude::*;

use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;
use core::fmt::Write;
use usb_device::class_prelude::*;
use usb_device::prelude::*;

use hal::pio::PIOExt;
use smart_leds::{brightness, SmartLedsWrite, RGB8};
use ws2812_pio::Ws2812;

// Currently 3 consecutive LEDs are driven by this example
// to keep the power draw compatible with USB:
const STRIP_LEN: usize = 21;
const N_KEYS: usize = 42;

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );


    let mut timer = Timer::new(pac.TIMER, &mut pac.RESETS);

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Set up smart leds
    let sin = hal::rom_data::float_funcs::fsin::ptr();

    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let mut keyboard = UsbHidClassBuilder::new()
        .add_device(
            usbd_human_interface_device::device::keyboard::NKROBootKeyboardConfig::default(),
        )
        .build(&usb_bus);

    //https://pid.codes
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0x0001))
        .manufacturer("Nabla Electronics")
        .product("Kallisto")
        .serial_number("002")
        .build();


    // Instanciate a Ws2812 LED strip:
    let mut ws = Ws2812::new(
        // Use pin 6 on the Raspberry Pi Pico (which is GPIO4 of the rp2040 chip)
        // for the LED data output:
        pins.gpio14.into_mode(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    // Set up i2c
    let sda_pin = pins.gpio2.into_mode::<hal::gpio::FunctionI2C>();
    let scl_pin = pins.gpio3.into_mode::<hal::gpio::FunctionI2C>();

    let mut i2c = I2C::new_controller(
        pac.I2C1,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        125_000_000.Hz()
    );
    
    // Set up all pins
    let mut row_pin_0 = pins.gpio27.into_push_pull_output();
    let mut row_pin_1 = pins.gpio26.into_push_pull_output();
    let mut row_pin_2 = pins.gpio22.into_push_pull_output();

    // Setting the output slew rate to fast is absolutley 
    // needed in order to prevent row cross-talk
    row_pin_0.set_slew_rate(hal::gpio::OutputSlewRate::Fast);
    row_pin_1.set_slew_rate(hal::gpio::OutputSlewRate::Fast);
    row_pin_2.set_slew_rate(hal::gpio::OutputSlewRate::Fast);

    // Set up our pins for the key matrix
    let mut row_pins: &mut [&mut dyn OutputPin<Error = core::convert::Infallible>] = &mut [
        &mut row_pin_0,
        &mut row_pin_1,
        &mut row_pin_2,
    ];

    let col_pins: &[&dyn InputPin<Error = core::convert::Infallible>] = &[
        &pins.gpio21.into_pull_up_input(),
        &pins.gpio20.into_pull_up_input(),
        &pins.gpio19.into_pull_up_input(),
        &pins.gpio18.into_pull_up_input(),
        &pins.gpio17.into_pull_up_input(),
        &pins.gpio16.into_pull_up_input(),
        &pins.gpio15.into_pull_up_input(),
    ];

    let mut led_pin = pins.led.into_push_pull_output();
    let mut key_matrix = KeyMatrix::new(&timer);
    let mut event_queue = Queue::<Keyboard, 32>::new();
    let (tx, mut rx) = event_queue.split();
    let mut kallisto = LayeredKeyboard::new(
        &timer,
        [BASE_LAYER, SYMBOL_LAYER, NUM_LAYER, [None; N_KEYS], [None; N_KEYS]],
        tx,
    );


    let mut leds: [RGB8; STRIP_LEN] = [(0, 0, 0).into(); STRIP_LEN];
    let mut t = 0.0;
    let strip_brightness = 64u8; // Limit brightness to 64/256

    // Slow down timer by this factor (0.1 will result in 10 seconds):
    let animation_speed = 0.1;


    let mut keyboard_timer = timer.count_down();
    let mut tick_timer = timer.count_down();
    let mut led_timer = timer.count_down();

    led_timer.start(17.millis());
    keyboard_timer.start(1.millis());
    tick_timer.start(1.millis());

    let mut i2c_buf: [u8; 4] = [0; 4];
    let mut i: usize;
    let mut report_buf: [Keyboard; 32];

    led_pin.set_high().unwrap();
    delay.delay_ms(1000);
    led_pin.set_low().unwrap();

    loop {

        if tick_timer.wait().is_ok() {
            let _ = keyboard.tick();
        }

        if keyboard_timer.wait().is_ok() {
            let mut r_state: u64 = 0;
            for row in (0..3).into_iter() {
                r_state |= key_matrix.poll_row_state(row, &mut row_pins, &col_pins) as u64;
            }
            let l_state: u64 = match i2c.read(0x33, &mut i2c_buf) {
                Ok(()) => u32::from_be_bytes(i2c_buf) as u64,
                Err(_) => 0,
            };
            
            let state = l_state | (r_state << 21);
            kallisto.get_report(state);
            // The above code may be  somewhat time consuming so we check if we need to tick here
            if tick_timer.wait().is_ok() {
                let _ = keyboard.tick();
            }
            report_buf = [Keyboard::NoEventIndicated; 32];
            i = 0;
            while rx.ready() {
                report_buf[i] = rx.dequeue().unwrap();
                i += 1;
            }
            let _ = keyboard.device().write_report(report_buf);
        }

        if tick_timer.wait().is_ok() {
            let _ = keyboard.tick();
        }

        if usb_dev.poll(&mut [&mut keyboard]) {
            match keyboard.device().read_report() {
                Err(UsbError::WouldBlock) => {
                    //do nothing
                }
                Err(e) => {
                    loop {
                        led_pin.set_high().unwrap();
                        delay.delay_ms(100);
                        led_pin.set_low().unwrap();
                        delay.delay_ms(100);
                    }
                }
                Ok(leds) => {
                    led_pin.set_state(PinState::from(leds.num_lock)).ok();
                }
            }
        }

        if led_timer.wait().is_ok() {
            for (i, led) in leds.iter_mut().enumerate() {
                // An offset to give 3 consecutive LEDs a different color:
                let hue_offs = match i % 3 {
                    1 => 0.25,
                    2 => 0.5,
                    _ => 0.0,
                };

                let sin_11 = sin((t + hue_offs) * 2.0 * core::f32::consts::PI);
                // Bring -1..1 sine range to 0..1 range:
                let sin_01 = (sin_11 + 1.0) * 0.5;

                let hue = 360.0 * sin_01;
                let sat = 1.0;
                let val = 1.0;

                let rgb = hsv2rgb_u8(hue, sat, val);
                *led = rgb.into();
            }

            ws.write(brightness(leds.iter().copied(), strip_brightness))
                .unwrap();

            t += (17.0 / 1000.0) * animation_speed;
            while t > 1.0 {
                t -= 1.0;
            }
        }
    }
}

pub fn hsv2rgb(hue: f32, sat: f32, val: f32) -> (f32, f32, f32) {
    let c = val * sat;
    let v = (hue / 60.0) % 2.0 - 1.0;
    let v = if v < 0.0 { -v } else { v };
    let x = c * (1.0 - v);
    let m = val - c;
    let (r, g, b) = if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (r + m, g + m, b + m)
}

pub fn hsv2rgb_u8(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let r = hsv2rgb(h, s, v);

    (
        (r.0 * 255.0) as u8,
        (r.1 * 255.0) as u8,
        (r.2 * 255.0) as u8,
    )
}
