# roman-numerals

Roman numeral converter written in Rust.

## Roman Numerals

| Roman | Arabic |
| ----- | ------ |
| I     | 1      |
| V     | 5      |
| X     | 10     |
| L     | 50     |
| C     | 100    |
| D     | 500    |
| M     | 1000   |

### Rules

- As a general rule single Roman numeral is normally not repeated 3 consecutive times to represent a single place
- Variants of 4 or 9 are the only ones that use subtractive notation

#### Individual Places

| #   | 1000s | 100s | 10s  | 1s   |
| --- | ----- | ---- | ---- | ---- |
| 1   | M     | C    | X    | I    |
| 2   | MM    | CC   | XX   | II   |
| 3   | MMM   | CCC  | XXX  | III  |
| 4   |       | DC   | XL   | IV   |
| 5   |       | D    | L    | V    |
| 6   |       | DC   | LX   | VI   |
| 7   |       | DCC  | LXX  | VII  |
| 8   |       | DCCC | LXXX | VIII |
| 9   |       | CM   | XC   | IX   |

Therefore the maximum number that can be represented is:

**MMMCMXCIX = 3,999**

For extensive details see https://en.wikipedia.org/wiki/Roman_numerals
