# Firmware

Created from [esp-rs/esp-idf-template](https://github.com/esp-rs/esp-idf-template) ([README](https://github.com/esp-rs/esp-idf-template/blob/master/README.md))

Important resources: 

- [`esp-idf-hal` examples](https://github.com/esp-rs/esp-idf-hal/tree/master/examples)
- [Rust on ESP Book](https://docs.esp-rs.org/book/writing-your-own-application/std.html) (`std` Applications)

## Development

Building:

```
cargo build
```

Flashing:

```
espflash flash target/xtensa-esp32s2-espidf/debug/firmware
```
