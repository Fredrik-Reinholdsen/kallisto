use kallisto_components::keyboard::types::*;

pub const N_KEYS: usize = 42;
pub const BASE_LAYER: [Option<LayerKeyMap>; N_KEYS] = [
    // Left half, Row 1
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::CapsLock,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Q,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::W,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::E,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::R,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::T,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Space,
            modifier: None,
        }),
        held_press: None,
    }),
    // Left half, Row 2
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Tab,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::A,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::S,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::D,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::F,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::G,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Escape,
            modifier: None,
        }),
        held_press: Some(KeyMapping {
            key: KeyPress::LeftControl,
            modifier: None,
        }),
    }),
    // Left half, Row 3
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::LeftShift,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Z,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::X,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::C,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::V,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::B,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::LayerHold1,
            modifier: None,
        }),
        held_press: None,
    }),
    // Right half, Row 1
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::DeleteBackspace,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Y,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::U,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::I,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::O,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::P,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::DeleteForward,
            modifier: None,
        }),
        held_press: None,
    }),
    // Right half, Row 2
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::ReturnEnter,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::H,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::J,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::K,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::L,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard7,
            modifier: Some(ModifierKey::RightShift),
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Minus,
            modifier: Some(ModifierKey::RightAlt),
        }),
        held_press: None,
    }),
    // Right half, Row 3
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::LayerHold2,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::N,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::M,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Comma,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Dot,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Dot,
            modifier: Some(ModifierKey::RightShift),
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::LeftGUI,
            modifier: None,
        }),
        held_press: None,
    }),
];

pub const SYMBOL_LAYER: [Option<LayerKeyMap>; N_KEYS] = [
    // Left half, Row 1
    None,
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard1,
            modifier: Some(ModifierKey::RightShift),
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard2,
            modifier: Some(ModifierKey::RightAlt),
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard7,
            modifier: Some(ModifierKey::RightAlt),
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard0,
            modifier: Some(ModifierKey::RightAlt),
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Minus,
            modifier: Some(ModifierKey::RightShift),
        }),
        held_press: None,
    }),
    None,
    // Left half, Row 2
    None,
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard3,
            modifier: Some(ModifierKey::RightShift),
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard4,
            modifier: Some(ModifierKey::RightAlt),
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard8,
            modifier: Some(ModifierKey::RightShift),
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard9,
            modifier: Some(ModifierKey::RightShift),
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard5,
            modifier: Some(ModifierKey::RightShift),
        }),
        held_press: None,
    }),
    None,
    // Left half, Row 3
    None,
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::NonUSBackslash,
            modifier: Some(ModifierKey::RightAlt),
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::RightBrace,
            modifier: Some(ModifierKey::RightAlt),
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard8,
            modifier: Some(ModifierKey::RightAlt),
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard9,
            modifier: Some(ModifierKey::RightAlt),
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard6,
            modifier: Some(ModifierKey::RightShift),
        }),
        held_press: None,
    }),
    None,
    // Right half, Row 1
    None,
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Backslash,
            modifier: Some(ModifierKey::RightShift),
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Backslash,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard2,
            modifier: Some(ModifierKey::RightShift),
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Minus,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard0,
            modifier: Some(ModifierKey::RightShift),
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::LeftBrace,
            modifier: None,
        }),
        held_press: None,
    }),
    // Right half, Row 2
    None,
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::LeftArrow,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::DownArrow,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::UpArrow,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::RightArrow,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Semicolon,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Apostrophe,
            modifier: None,
        }),
        held_press: None,
    }),
    // Right half, Row 3
    None,
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::ForwardSlash,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::ForwardSlash,
            modifier: Some(ModifierKey::RightShift),
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::NonUSBackslash,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::NonUSBackslash,
            modifier: Some(ModifierKey::RightShift),
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Comma,
            modifier: Some(ModifierKey::RightShift),
        }),
        held_press: None,
    }),
    None,
];

pub const NUM_LAYER: [Option<LayerKeyMap>; N_KEYS] = [
    // Left half, Row 1
    None,
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::F1,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::F2,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::F3,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::F4,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::F5,
            modifier: None,
        }),
        held_press: None,
    }),
    None,
    // Left half, Row 2
    None,
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard1,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard2,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard3,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard4,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard5,
            modifier: None,
        }),
        held_press: None,
    }),
    None,
    // Left half, Row 3
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    // Right half, Row 1
    None,
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::F6,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::F7,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::F8,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::F9,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::F10,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::F11,
            modifier: None,
        }),
        held_press: None,
    }),
    // Right half, Row 2
    None,
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard6,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard7,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard8,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard9,
            modifier: None,
        }),
        held_press: None,
    }),
    Some(LayerKeyMap {
        pressed: Some(KeyMapping {
            key: KeyPress::Keyboard0,
            modifier: None,
        }),
        held_press: None,
    }),
    None,
    // Right half, Row 3
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];
