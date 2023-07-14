
use rp_pico::hal as hal;
use core::convert::Infallible;
use embedded_hal::digital::v2::{InputPin, OutputPin};

pub const N_ROWS: usize = 3;
pub const N_COLS: usize = 7;
pub const N_KEYS : usize = N_COLS * N_ROWS;
pub const PIN_SETTLE_TIME_US: u32 = 200;
pub const DEBOUNCE_US: u32 = 50_000;

pub struct KeyMatrix<'t> {
    // Hold the time a change event happended for
    // each key in the matrix
    last_events: [u32; N_ROWS * N_COLS],
    last_key_states: [bool; N_COLS * N_ROWS],
    // Hold the time the last row was polled
    last_polled: [u32; N_ROWS],
    timer: &'t hal::Timer,
}

impl<'t> KeyMatrix<'t> {

    pub fn new(timer: &'t hal::Timer) -> Self {
        Self {
            timer,
            last_key_states: [false; N_COLS * N_ROWS],
            last_events: [timer.get_counter_low(); N_ROWS * N_COLS],
            last_polled: [timer.get_counter_low(); N_ROWS],
        }
    }
    // Returns the current state of the pins
    // as a 32-bit mask where each bit corresponds
    // to a pin, in this case only the lowest 21 bits
    // are used
    pub fn poll_row_states(
        &mut self,
        row_pins: &mut [&mut dyn OutputPin<Error = Infallible>],
        col_pins: &[&dyn InputPin<Error = Infallible>],
    ) -> u32  {
        let mut key_states: u32 = 0;
        for row in (0..row_pins.len()).into_iter() {
            row_pins[row].set_low().unwrap();
            // Delay until pin has setteled
            let t0 = self.timer.get_counter_low();
            while self.timer.get_counter_low() - t0 < PIN_SETTLE_TIME_US {
                cortex_m::asm::nop();
            }
            let now = self.timer.get_counter_low();
            for col in (0..col_pins.len()).into_iter() {
                let id = col + row * N_COLS;
                let key_state = col_pins[col].is_low().unwrap();
                if key_state != self.last_key_states[id] &&
                    now - self.last_events[id] > DEBOUNCE_US {
                    self.last_events[id] = now;
                    key_states |= (key_state as u32) << id;
                    self.last_key_states[id] = key_state;
                } else {
                    key_states |= (self.last_key_states[id] as u32) << id;
                }
            }
            row_pins[row].set_high().unwrap();
        }
        key_states
    }

    pub fn poll_row_state(
        &mut self,
        row: usize,
        row_pins: &mut [&mut dyn OutputPin<Error = Infallible>],
        col_pins: &[&dyn InputPin<Error = Infallible>],
    ) -> u32  {
        let mut key_states: u32 = 0;
        row_pins[row].set_low().unwrap();
        // Delay until pin has setteled
        let t0 = self.timer.get_counter_low();
        while self.timer.get_counter_low() - t0  < PIN_SETTLE_TIME_US {
            cortex_m::asm::nop();
        }
        let now = self.timer.get_counter_low();
        // Loop through columns and check if key is pressed
        for col in (0..col_pins.len()).into_iter() {
            let id = col + row * N_COLS;
            let key_state = col_pins[col].is_low().unwrap();
            if key_state != self.last_key_states[id] &&
                now - self.last_events[id] > DEBOUNCE_US {
                self.last_events[id] = now;
                key_states |= (key_state as u32) << id;
                self.last_key_states[id] = key_state;
            } else {
                key_states |= (self.last_key_states[id] as u32) << id;
            }
        }
        row_pins[row].set_high().unwrap();
        key_states
    }
}
