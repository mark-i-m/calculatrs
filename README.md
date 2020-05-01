# calculatrs

A python-like calculator for simple math on the terminal. It only supports a
handful of basic operators; if you need more, just use actual python. If you
only need to do simple arithmetic, though, this should be fine.

Currently:
- All integers are 128-bit signed
- All floating-point values are 64-bit IEEE754.
- Integers and floating-point values cannot be mixed without an explicit cast:
  `int(2.0)` or `float(2)`.
- Precedence aims to be C-like, though, I haven't tested thoroughly
- Expressions can be grouped with `( <expr> )`, as one would expect.
- Integers support the following basic operations:
    - `+`, `-`, `*`, `/`: basic arithmetic. Note that we use integer division
      for integer-typed operands.
    - `<<`, `>>`: left and right shift
- Floats support the following basic operations:
    - `+`, `-`, `*`, `/`: basic arithmetic. Note that we use floating-point
      division for float-typed operands.
    - `b ** e`: `b` raised to the power `e`. `e` may be a float or an integer,
      but if it is an integer, it will be truncated to 32-bits.
