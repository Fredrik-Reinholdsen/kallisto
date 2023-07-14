use hid::page::Keyboard;
use usbd_human_interface_device as hid;
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};

#[repr(u16)]
#[derive(Clone, Copy, TryFromPrimitive)]
pub enum KeyPress {
    ErrorRollOver = 0x01,
    POSTFail = 0x02,
    ErrorUndefine = 0x03,
    A = 0x04,
    B = 0x05,
    C = 0x06,
    D = 0x07,
    E = 0x08,
    F = 0x09,
    G = 0x0A,
    H = 0x0B,
    I = 0x0C,
    J = 0x0D,
    K = 0x0E,
    L = 0x0F,
    M = 0x10,
    N = 0x11,
    O = 0x12,
    P = 0x13,
    Q = 0x14,
    R = 0x15,
    S = 0x16,
    T = 0x17,
    U = 0x18,
    V = 0x19,
    W = 0x1A,
    X = 0x1B,
    Y = 0x1C,
    Z = 0x1D,
    Keyboard1 = 0x1E,
    Keyboard2 = 0x1F,
    Keyboard3 = 0x20,
    Keyboard4 = 0x21,
    Keyboard5 = 0x22,
    Keyboard6 = 0x23,
    Keyboard7 = 0x24,
    Keyboard8 = 0x25,
    Keyboard9 = 0x26,
    Keyboard0 = 0x27,
    ReturnEnter = 0x28,
    Escape = 0x29,
    DeleteBackspace = 0x2A,
    Tab = 0x2B,
    Space = 0x2C,
    Minus = 0x2D,
    Equal = 0x2E,
    LeftBrace = 0x2F,
    RightBrace = 0x30,
    Backslash = 0x31,
    NonUSHash = 0x32,
    Semicolon = 0x33,
    Apostrophe = 0x34,
    Grave = 0x35,
    Comma = 0x36,
    Dot = 0x37,
    ForwardSlash = 0x38,
    CapsLock = 0x39,
    F1 = 0x3A,
    F2 = 0x3B,
    F3 = 0x3C,
    F4 = 0x3D,
    F5 = 0x3E,
    F6 = 0x3F,
    F7 = 0x40,
    F8 = 0x41,
    F9 = 0x42,
    F10 = 0x43,
    F11 = 0x44,
    F12 = 0x45,
    PrintScreen = 0x46,
    ScrollLock = 0x47,
    Pause = 0x48,
    Insert = 0x49,
    Home = 0x4A,
    PageUp = 0x4B,
    DeleteForward = 0x4C,
    End = 0x4D,
    PageDown = 0x4E,
    RightArrow = 0x4F,
    LeftArrow = 0x50,
    DownArrow = 0x51,
    UpArrow = 0x52,
    KeypadNumLockAndClear = 0x53,
    KeypadDivide = 0x54,
    KeypadMultiply = 0x55,
    KeypadSubtract = 0x56,
    KeypadAdd = 0x57,
    KeypadEnter = 0x58,
    Keypad1 = 0x59,
    Keypad2 = 0x5A,
    Keypad3 = 0x5B,
    Keypad4 = 0x5C,
    Keypad5 = 0x5D,
    Keypad6 = 0x5E,
    Keypad7 = 0x5F,
    Keypad8 = 0x60,
    Keypad9 = 0x61,
    Keypad0 = 0x62,
    KeypadDot = 0x63,
    NonUSBackslash = 0x64,
    Application = 0x65,
    Power = 0x66,
    KeypadEqual = 0x67,
    F13 = 0x68,
    F14 = 0x69,
    F15 = 0x6A,
    F16 = 0x6B,
    F17 = 0x6C,
    F18 = 0x6D,
    F19 = 0x6E,
    F20 = 0x6F,
    F21 = 0x70,
    F22 = 0x71,
    F23 = 0x72,
    F24 = 0x73,
    Execute = 0x74,
    Help = 0x75,
    Menu = 0x76,
    Select = 0x77,
    Stop = 0x78,
    Again = 0x79,
    Undo = 0x7A,
    Cut = 0x7B,
    Copy = 0x7C,
    Paste = 0x7D,
    Find = 0x7E,
    Mute = 0x7F,
    VolumeUp = 0x80,
    VolumeDown = 0x81,
    LockingCapsLock = 0x82,
    LockingNumLock = 0x83,
    LockingScrollLock = 0x84,
    KeypadComma = 0x85,
    KeypadEqualSign = 0x86,
    Kanji1 = 0x87,
    Kanji2 = 0x88,
    Kanji3 = 0x89,
    Kanji4 = 0x8A,
    Kanji5 = 0x8B,
    Kanji6 = 0x8C,
    Kanji7 = 0x8D,
    Kanji8 = 0x8E,
    Kanji9 = 0x8F,
    LANG1 = 0x90,
    LANG2 = 0x91,
    LANG3 = 0x92,
    LANG4 = 0x93,
    LANG5 = 0x94,
    LANG6 = 0x95,
    LANG7 = 0x96,
    LANG8 = 0x97,
    LANG9 = 0x98,
    AlternateErase = 0x99,
    SysReqAttention = 0x9A,
    Cancel = 0x9B,
    Clear = 0x9C,
    Prior = 0x9D,
    Return = 0x9E,
    Separator = 0x9F,
    Out = 0xA0,
    Oper = 0xA1,
    ClearAgain = 0xA2,
    CrSelProps = 0xA3,
    ExSel = 0xA4,
    //0xA5-0xDF Reserved
    LeftControl = 0xE0,
    LeftShift = 0xE1,
    LeftAlt = 0xE2,
    LeftGUI = 0xE3,
    RightControl = 0xE4,
    RightShift = 0xE5,
    RightAlt = 0xE6,
    RightGUI = 0xE7,
    //0xE8-0xFF Reserved
    LayerIncrement = 0x0100,
    LayerDecrement = 0x0101,
    LayerSet0 = 0x0102,
    LayerSet1 = 0x0103,
    LayerSet2 = 0x0104,
    LayerSet3 = 0x0105,
    LayerSet4 = 0x0106,
    LayerHold0 = 0x0107,
    LayerHold1 = 0x0108,
    LayerHold2 = 0x0109,
    LayerHold3 = 0x010A,
    LayerHold4 = 0x010B,
}

#[repr(u8)]
#[derive(Clone, Copy, TryFromPrimitive)]
pub enum ModifierKey {
    LeftControl = 0xE0,
    LeftShift = 0xE1,
    LeftAlt = 0xE2,
    LeftGUI = 0xE3,
    RightControl = 0xE4,
    RightShift = 0xE5,
    RightAlt = 0xE6,
    RightGUI = 0xE7,
}

#[derive(Debug)]
pub enum KeyEvent {
    Pressed(usize),
    DoublePress(usize),
    HeldPress(usize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyState {
    FirstPress,
    Pressed,
    HeldPressed,
    None,
}

#[derive(Copy, Clone)]
pub struct KeyMapping {
    pub key: KeyPress,
    pub modifier: Option<ModifierKey>,
}

#[derive(Copy, Clone)]
pub struct LayerKeyMap {
    pub pressed: Option<KeyMapping>,
    pub held_press: Option<KeyMapping>,
}

pub trait IntoKeyboardIter<const N: usize> {
    fn into_keyboard_iter(&self) -> core::array::IntoIter<Keyboard, N>;
}

pub trait IntoKeyboard {
    fn into_keyboard(&self) -> Keyboard;
}

impl IntoKeyboard for ModifierKey {
    fn into_keyboard(&self) -> Keyboard {
        (*self as u8).into()
    }

}

impl IntoKeyboard for Option<ModifierKey> {
    fn into_keyboard(&self) -> Keyboard {
        if self.is_none() {
            Keyboard::NoEventIndicated
        } else {
            self.unwrap().into_keyboard()
        }
    }

}

impl IntoKeyboard for KeyPress {
    fn into_keyboard(&self) -> Keyboard {
        let e_repr = *self as u16;
        if e_repr > 0xFF {
            Keyboard::NoEventIndicated
        } else {
            ((e_repr & 0xFF) as u8).into()
        }
    }

}

impl IntoKeyboardIter<2> for KeyMapping {
    fn into_keyboard_iter(&self) -> core::array::IntoIter<Keyboard, 2> {
        [self.key.into_keyboard(), self.modifier.into_keyboard()].into_iter()
        
    }
}

impl IntoKeyboardIter<2> for Option<KeyMapping> {
    fn into_keyboard_iter(&self) -> core::array::IntoIter<Keyboard, 2> {
        match self {
            Some(mapping) => mapping.into_keyboard_iter(),
            None => [Keyboard::NoEventIndicated, Keyboard::NoEventIndicated].into_iter()
        }
    }
}


impl IntoKeyboardIter<1> for dyn IntoKeyboard {
    fn into_keyboard_iter(&self) -> core::array::IntoIter<Keyboard, 1> {
        [self.into_keyboard()].into_iter()
    }

}
