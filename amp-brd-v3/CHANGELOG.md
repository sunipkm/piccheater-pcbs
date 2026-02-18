# AMP-BOARD v3 CHANGELOG
### Rev. A1 - 2026-02-17
- DS28EA00: `PIOB` tied to `GND` instead of floating. This change is per
  PIC-D temperature sensors.
- I<sup>2</sup>C: 10k pull-up resistors added to `SCL` and `SDA` lines.
- DC-DC converter: Increased CTRL<sub>IN</sub> pull-down resistor from 10k
  to 100k to ensure low output voltage when CTRL<sub>IN</sub> AND EN are
  both floating.
- Changed V<sub>OUT</sub> connector from 4-pin PicoBlade / 2.54mm 2 pos header 
  to 2-pin PicoBlade for ease of wiring in the final version. Prevents
  multiple wires from being crimped inside the same DB-37 pin.