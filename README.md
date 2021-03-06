# calculatrs

A python-like calculator for simple math on the terminal. It only supports a
handful of basic operators; if you need more, just use actual python. If you
only need to do simple arithmetic, though, this should be fine.

Currently:
- All integers are 128-bit signed
- All floating-point values are 64-bit IEEE754.
- Integers are coerced to floats in mixed expressions.
- Values can be explicitly cast to a type: `int(2.0)` or `float(2)`. For
  integers, this truncates the value.
- Integers support the following basic operations:
    - `+`, `-`, `*`, `/`: basic arithmetic. Note that we use integer division
      for integer-typed operands.
    - `<<`, `>>`: left and right shift
    - `b ** e`: `b` raised to the power `e`. If `e` is a float, `b` is cast to
      a float also. If `e` is an integer, `e` is cast to a 32-bit unsigned
      integer.
- Floats support the following basic operations:
    - `+`, `-`, `*`, `/`: basic arithmetic. Note that we use floating-point
      division for float-typed operands.
    - `b ** e`: `b` raised to the power `e`. `e` may be a float or an integer,
      but if it is an integer, it will be truncated to 32-bits.
- Precedence aims to be sane, though I haven't tested thoroughly:

  |`+`, `-`| Lower Precedence |
  |--------|------------------|
  |All others| Higher Precence|

- All operators are right-associative.
- Expressions can be grouped with `( <expr> )`, as one would expect.
- The special `_` value represents the previous (typed) result, which is stored
  in `/tmp/calculatrs`. It can be used any place a value is expected. If an
  error occurs, no change is made to the saved value.
- In order to make it easier to use directly in the terminal, the following
  alternate syntaxes exist:
    - `*` == `x`
    - `**` == `^^`

## Building

You will need [stable Rust](https://rustup.rs).

```sh
cargo build
```

## Installing

You can install from cargo:

```
cargo install calculatrs
```

The installed binary is called `c`.

## Usage

Suppose you install the binary on your path

```console
> c 10 / 5
2
> c '10 / 5'
2
```

Beware of your shell expanding `*` into "list all files".
