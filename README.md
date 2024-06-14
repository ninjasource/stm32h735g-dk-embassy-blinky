# stm32h735g-dk-embassy-blinky
Sets up power and clocks and blinks an led (LD1) on the stm32h735g-dk using the async embassy runtime.

The stm32h735g-dk uses an external oscillator so the default clock and power settings for embassy won't to.

## Setup

Install `probe-rs` from here: https://probe.rs/

Install the Arm cross compiler
`rustup target add thumbv7em-none-eabihf`

## To run

```
cargo run --release
```