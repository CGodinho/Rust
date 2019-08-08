# Variables

## Mutable / Immutable

Immutable variables are created with **let**:

`let x = 5;`

`x = 6;` <span style="color:red"> -> ERROR</span>

For mutable, use reserve word **mut**:

`let mut x = 5;`

`let mut x = 6;`


## Reassign

Using let it is possible to redefine a immutable variable:

`let spaces = "   ";`

`let spaces = spaces.len();`

## Constants

`const MAX_POINTS: u32 = 100_000;`

**NOTE:** Use uppercase for constants.