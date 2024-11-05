# References

This file lists any references/tools/etc. that (may) assist with this project.

## Standards

- [DisplayID v2.1a](https://vesa.org/vesa-standards/): the main standard this project aims to implement. modern, as it's extensible without new releases of the standard - meaning it only requires a new extension to add new device kinds!
  - [its Wikipedia article has readable tables and explanations](https://en.wikipedia.org/wiki/DisplayID)
- [E-EDID (Release A, Revision 2)](https://glenwing.github.io/docs/VESA-EEDID-A2.pdf): "Extensible" EDID, which includes support for extensions that vendors were creating even without support.

## Implementations

- [`libdisplay-info`](https://gitlab.freedesktop.org/emersion/libdisplay-info): a C impl of DisplayID. Low-traffic, only implements [small portions](https://gitlab.freedesktop.org/emersion/libdisplay-info/-/issues/17) of the standard.
- [`edid-decode`](https://git.linuxtv.org/edid-decode.git/): a command-line utility implementing EDID parsing.

## Databases

- [`linuxhw`'s "EDID" repo](https://github.com/linuxhw/EDID): a big collection of (parsed) EDIDs. also contains their raw info up top
