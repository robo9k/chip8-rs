# TODOs

- clock rate?
- interrupts, e.g. for hardware key presses, timers?
- pausing the vm

traits for hardware

- beeper
  - state: beeping, not beeping
  - functions: start beeping, stop beeping
- keypad
  - key definitions
  - state: pressed, not pressed
  - functions: key down, key up
- screen
  - pixel
    - state: set, unset
  - functions: clear, draw pixels

split up vm into

- cpu
  - registers
    - v*, i
    - pc, sp
  - stack
  - timers
- memory
- rng?

vm

- functions
  - load rom
  - reset state (registers, timers, maybe hardware)
  - halt
