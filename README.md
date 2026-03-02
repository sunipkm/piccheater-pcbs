# PICTURE-C/D Heater Controller PCBs
- `dac-brd`: Raspberry Pi Pico 2 based board with 2x 8 channel DACs supplied by a 2.048V precision reference voltage.
- `amp-mezzanine-v2`: A mezzanine board that supports stacking 4x AMP boards. The mezzanine takes power and control signals and pass them to the AMP boards.
- `amp-brd-v3`: A DC-DC converter board with controllable reference voltage that allows adjusting its output voltage between 0V (1V control) and 25.7V (0V control, V<sub>in</sub> > 25.7V). An on-board INA233 monitors the output voltage and current, and a DS28EA00 1-Wire temperature sensor reports board temperature.
- `amp-brd-v4`: A future, in-development version that integrates monitoring and control features using a USB-PD chip. It would allow all 16 boards on one stack.


This repository is a submodule of [`piccheater`](https://github.com/sunipkm/piccheater).