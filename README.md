# BeagleBone Black with OLED Display (SH1106) Example

## I2C
```
#!/bin/bash

config-pin p9.17 i2c
config-pin p9.18 i2c
```

`config-pin -q p9.17 && config-pin -q p9.18`

## Compile
`cargo build --target arm-unknown-linux-gnueabihf --release`