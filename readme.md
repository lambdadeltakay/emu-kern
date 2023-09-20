# Emu Kern

This is a experimental operating system to run custom made emulators on embedded devices. Currently does nothing as the basic architecture is still being designed.

## Targeted Devices

- Game Boy Advance

## To Build

The build system being used currently is a bunch of ugly shell scripts

Run `debug.sh thumbv4t-nintendo-gba` for gameboy advance support

Run `debug.sh x86_64-unknown-linux-gnu` for x86_64 linux support (constantly broken)


Replace `debug.sh` with `release.sh` for release builds

## External Code used

[Unifont](https://unifoundry.com/unifont/index.html)