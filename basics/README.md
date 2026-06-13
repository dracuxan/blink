# blink

Rust project for **Arduino Nano** experiments. Each module contains the logic for a different project — swap which one runs from `src/main.rs`.

## Modules

| Module       | File                | Description                                                                            |
| ------------ | ------------------- | -------------------------------------------------------------------------------------- |
| `blink_led`  | `src/blink_led.rs`  | Built-in LED on D13 toggles every 500ms                                                |
| `flame_pump` | `src/flame_pump.rs` | Reads flame sensor on A0, prints value to serial, runs pump via L298N when value < 125 |

## Usage

In `src/main.rs`, uncomment the module you want to flash:

```rust
#[arduino_hal::entry]
fn main() -> ! {
    blink_led::run()
    // flame_pump::run()
}
```

Then build and flash:

```bash
cargo run
```

`ravedude` opens a serial console after flashing (57600 baud).

## Requirements

- `avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`](https://crates.io/crates/ravedude)
- See the [`avr-hal` README](https://github.com/Rahix/avr-hal#readme)

## Adding a new module

1. Create `src/my_module.rs` with a `pub fn run() -> !` entry point.
2. Add `mod my_module;` in `src/main.rs`.
3. Call `my_module::run()` from `main()`.
