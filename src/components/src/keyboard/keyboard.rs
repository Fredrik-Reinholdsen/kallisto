use rp_pico::hal as hal;

use heapless::spsc::Producer;
use usbd_human_interface_device as hid;
use hid::page::Keyboard;
use heapless::spsc::Queue;

use crate::keyboard::types::*;

const N_KEYS: usize = 21;

pub const DEBOUNCE_US: u32 = 20_000;
pub const DOUBLE_PRESS_MAX_US: u32 = 200_000;
pub const HOLD_PRESS_MIN_US: u32 = 500_000;
pub const HOLD_PRESS_PERIOD: u32 = 50_000;
pub const N_LAYERS: usize = 5;


pub struct LayeredKeyboard<'t, 'q, const N: usize> {
    report_tx: Producer<'q, Keyboard, 32>,
    layers: [[Option<LayerKeyMap>; N]; 5],
    key_states: [KeyState; N],
    last_state_b: [bool; N],
    event_queue: Queue<KeyEvent, 64>,
    // Id of the any currently held key with a held press mapping
    // If multiple such keys are held then only the id of the most recent
    // is stored in this option
    held_key: Option<usize>,
    double_key : Option<usize>,
    // Array of timestamps of when each key was last pressed
    last_press_t: [u32; N],
    timer: &'t hal::Timer,
    pub layer: usize,
    last_layer: usize,
    is_layer_held: bool,
}

impl<'t, 'q, const N: usize> LayeredKeyboard <'t,'q, N> {
    pub fn new(
        timer: &'t hal::Timer,
        layers: [[Option<LayerKeyMap>; N]; N_LAYERS],
        report_tx: Producer<'q, Keyboard, 32>
    ) -> Self {
        LayeredKeyboard {
            report_tx,
            timer,
            layers,
            event_queue: Queue::new(),
            key_states: [KeyState::None; N],
            layer: 0,
            held_key: None,
            double_key : None,
            last_layer: 0,
            last_press_t: [timer.get_counter_low(); N],
            last_state_b: [false; N],
            is_layer_held: false,
        }
    }

    // Updates the last state array to the current state
    fn update_last_state(&mut self, pin_states: u64) {
        self.last_state_b
            .iter_mut()
            .enumerate()
            .for_each(|(i, s)| *s = (pin_states >> i) & 0x1 == 1);
        
    }

    fn get_key_map(&self, id: usize) -> Option<LayerKeyMap> {
        self.layers[self.layer][id]
    }

    // Function that gets run each time a key gets pressed
    fn key_pressed(&mut self, id: usize) {
        self.key_states[id] = KeyState::FirstPress;
    }

    // Function that gets run each time a key is released
    fn key_released(&mut self, id: usize) {
        let key_map = self.get_key_map(id);
        let now = self.timer.get_counter_low();

        if self.held_key.is_some() {
            let held_id = self.held_key.unwrap();
            if held_id == id {
                self.held_key = None;
            }
        }

        // Buttons with held press mapping fire their key pressed
        // when released rather than on the press if the total key
        // down time is less than than the minimum held press time.
        if key_map.is_some() && key_map.unwrap().held_press.is_some() && (
            self.key_states[id] == KeyState::Pressed || self.key_states[id] == KeyState::FirstPress) {
            if now - self.last_press_t[id] < HOLD_PRESS_MIN_US  {
                key_map.unwrap().pressed.into_keyboard_iter()
                    .filter(|r| *r != Keyboard::NoEventIndicated)
                    .for_each(|r| {let _ = self.report_tx.enqueue(r); });
            }
        }
        self.key_states[id] = KeyState::None;

    }

    // Takes a set of key states as a u64 and processes
    // it, and outputs a keyboard HID report
    pub fn get_report(&mut self, pin_states: u64) {
        // Process the key pin states and map it to
        // key-press and key-release events
        let mut is_pressed: bool;
        let now: u32 = self.timer.get_counter_low();

        for id in (0..N).into_iter() {
            is_pressed = (pin_states >> id) & 0x1 == 1;
            if !is_pressed {
                if self.last_state_b[id] {
                    self.key_released(id);
                }
            } else {
                if !self.last_state_b[id] && now - self.last_press_t[id] > DEBOUNCE_US {
                    self.last_press_t[id] = now;
                    self.key_pressed(id);
                }
            }
        }
        self.update_last_state(pin_states);

        let now = self.timer.get_counter_low();

        // Processes the map for layer events
        for id in (0..self.key_states.len()).into_iter() {
            // Map key states to key events that will then get mapped to key-presses
            let state = self.key_states[id];

            let key_map = match self.get_key_map(id) {
                Some(m) => m,
                None => {
                    if self.layer == 0 {
                        continue;
                    }
                    if self.layers[0][id].is_some() {
                        self.layers[0][id].unwrap()
                    } else {
                        continue;
                    }

                },
            };

            match state {
                KeyState::None => continue,
                KeyState::Pressed | KeyState::FirstPress => {
                    // If a key with a held press mapping is held and
                    // another key is pressed then pre-emptivley fire
                    // the held-press mapping of that key along with
                    // the newly pressed key
                    if self.held_key.is_some() && self.held_key.unwrap() != id {
                        let held_id = self.held_key.unwrap();
                        self.key_states[held_id] = KeyState::HeldPressed;
                        let _ = self.event_queue.enqueue(KeyEvent::HeldPress(held_id));
                        self.held_key = None;
                    } else {
                        // Transistion key state to held press if it has been held for longer
                        // than the set hold press time, else set the key state to pressed
                        if key_map.held_press.is_some() {
                            self.held_key = Some(id);
                        } else {
                            let _ = self.event_queue.enqueue(KeyEvent::Pressed(id));
                        }
                    }
                }
                KeyState::HeldPressed => {
                    let _ = self.event_queue.enqueue(KeyEvent::HeldPress(id));
                }
            };

            // Transistion key state to held press if it has been held for longer
            // than the set hold press time, else set the key state to pressed
            if state == KeyState::Pressed && now - self.last_press_t[id] >= HOLD_PRESS_MIN_US {
                self.key_states[id] = KeyState::HeldPressed;
            }

            if state == KeyState::FirstPress {
                self.key_states[id] = KeyState::Pressed;
            }
        }

        let mut is_layer_held = false;
        while !self.event_queue.is_empty() {
            let event = self.event_queue.dequeue().unwrap();

            let (id, mapping) = match event {
                KeyEvent::Pressed(id) => {
                    let key_map = match self.layers[self.layer][id] {
                        Some(m) => m,
                        None => {
                            // If you are not on the base layer,
                            // and the pressed key does not have a mapping on that layer
                            // then return base layer mapping if it is Some, else continue 
                            if self.layer == 0 {
                                continue;
                            }
                            if self.layers[0][id].is_some() {
                                self.layers[0][id].unwrap()
                            } else {
                                continue;
                            }
                        }
                    };
                    if key_map.pressed.is_none() {
                        continue;
                    } else {
                        (id, key_map.pressed.unwrap())
                    }
                }
                KeyEvent::HeldPress(id) => {
                    let key_map = match self.layers[self.layer][id] {
                        Some(m) => m,
                        None => {
                            if self.layer == 0 {
                                continue;
                            }
                            if self.layers[0][id].is_some() {
                                self.layers[0][id].unwrap()
                            } else {
                                continue;
                            }
                        },
                    };
                    if key_map.held_press.is_none() {
                        if key_map.pressed.is_none() {
                                continue;
                        }
                        (id, key_map.pressed.unwrap())
                    } else {
                        (id, key_map.held_press.unwrap())
                    }
                }
                KeyEvent::DoublePress(id) => continue,

            };

            // Look at the pressed keys and see if any layer key was pressed,
            // in which case that layer event is processed
            match mapping.key {
                KeyPress::LayerSet0 | KeyPress::LayerSet1 |KeyPress::LayerSet2 |KeyPress::LayerSet3 |KeyPress::LayerSet4 => {
                    self.layer = mapping.key as usize - 0x102;
                    // Reset key states on layer transition
                    self.key_states = [KeyState::None; N];
                    self.last_layer = mapping.key as usize - 0x102;
                    continue;
                }
                KeyPress::LayerHold0 |KeyPress::LayerHold1 | KeyPress::LayerHold2 | KeyPress::LayerHold3 | KeyPress::LayerHold4 => {
                    let held_layer = mapping.key as usize - 0x107;
                    if self.layer != held_layer {
                        // Here to avoid bug when two hold layer buttons
                        // causing permanent transition to one of the held layers
                        // instead of the base layer
                        if !self.is_layer_held {
                            self.last_layer = self.layer;
                            self.key_states = [KeyState::None; N];
                            self.key_states[id] = KeyState::Pressed;
                        }
                        self.layer = held_layer;
                    }
                    is_layer_held = true;
                    continue;
                }
                _ => {}
            }
            // Maps the key state into a series of HID key events
            mapping.into_keyboard_iter()
                .filter(|k| *k != Keyboard::NoEventIndicated)
                .for_each(|k| { let _ = self.report_tx.enqueue(k); });
        }

        self.is_layer_held = is_layer_held;

        // If no layer is held then roll-back
        if !self.is_layer_held  && self.last_layer != self.layer {
            self.layer = self.last_layer;
            // Reset key states on layer transition
            self.key_states = [KeyState::None; N];
        }
    }
}
