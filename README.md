# Kallisto

A custom split keyboard, with custom electronics and firmware.

![kallisto](https://github.com/Fredrik-Reinholdsen/kallisto/assets/11893023/27eccc83-5426-4d77-808a-6b297a9c914a)

The hardware is based around a variant of the _Raspberry Pi Pico_ board, and the _RP2040_ microcontroller.
The keyboard features:

- 42 Kailh 1350 mechanical keyswitches (21 per half), 4 extra buttons and one rotary encoder.
- Fully programmable key-map were any key can be mapped to any key/media function.
- Layers
- Custom RGB lighting with adjustable brightness
- Onboard flash memory that hold the key-map.

All electronics were designed using KiCad, and all firmware is written in Rust.


## Key Mapping
Any physical key on the keyboard may be mapped to any key, with or without an additional modifier.
Buttons have a few different press modes, similar to the *QMK* firmware, that you can map separatley to different keys.

The different key press types/events are:
- Press
- Held Press
- Double Press

### Held Press
A held press is when a button is pressed and held, for a set ammount of time (0.5s by default).
If a button has a held press mapping is pressed and then released before the held-time window,
then the regular _Press_ mapping while fire upon the release, not the press.
If another button is pressed before the end of the held-time window, then the held press mapping while fire immediatley,
and the other pressed button will fire right after.
If a button with a held press mapping is pressed and held for more than the held-time window, and no other button is pressed in between,
then the held press key mapping will fire.
