use lalrpop_util::lalrpop_mod;

#[clippy::skip]
lalrpop_mod!(pub calculatrs);

const TEMP_PATH: &str = "/tmp/calculatrs";

fn main() {
    let arg_str = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    let ast = match calculatrs::ExprParser::new().parse(&arg_str) {
        Ok(ast) => ast,
        Err(err) => {
            println!("Unable to parse: {}", err);
            std::process::exit(1);
        }
    };

    let result = match eval(*ast) {
        Ok(val) => val,
        Err(EvalError::ShiftFloat) => {
            println!("Attempt to shift float value");
            std::process::exit(1);
        }
        Err(EvalError::UnableToReadPreviousValue) => {
            println!("Unable to read previous value from file {}", TEMP_PATH);
            std::process::exit(1);
        }
        Err(EvalError::CorruptPreviousValue) => {
            println!("Corrupt previous value in file {}", TEMP_PATH);
            std::process::exit(1);
        }
    };

    // Print
    match result {
        Evaluated::Int(i) => println!("{}", i),
        Evaluated::Float(f) => println!("{}", f),
    };

    // Save result
    let prev_res = match result {
        Evaluated::Int(i) => format!("i,{}", i),
        Evaluated::Float(f) => format!("f,{}", f),
    };
    std::fs::write(TEMP_PATH, prev_res).expect("Unable to write result to temp file.");
}

/// Abstract Syntax Tree
mod ast {
    #[derive(Debug)]
    pub enum Value<'s> {
        PreviousResult,
        Float(&'s str),
        Int(&'s str),
    }

    #[derive(Debug)]
    pub enum BinOp {
        Add,
        Sub,
        Mul,
        Div,
        ShiftLeft,
        ShiftRight,
        Exp,
    }

    #[derive(Debug)]
    pub enum Expr<'s> {
        BinOp {
            left: Box<Expr<'s>>,
            op: BinOp,
            right: Box<Expr<'s>>,
        },

        IntCast {
            expr: Box<Expr<'s>>,
        },

        FloatCast {
            expr: Box<Expr<'s>>,
        },

        Value {
            val: Value<'s>,
        },
    }
}

#[derive(Debug)]
enum EvalError {
    ShiftFloat,
    UnableToReadPreviousValue,
    CorruptPreviousValue,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Evaluated {
    Int(i128),
    Float(f64),
}

/// Evaluate an AST.
fn eval(ast: ast::Expr<'_>) -> Result<Evaluated, EvalError> {
    use ast::{BinOp::*, Expr::*, Value::*, *};

    // Descend into the tree, recursively evaluating. Coerce integers into floats if needed.

    Ok(match ast {
        Expr::BinOp { left, op, right } => {
            let left_value = eval(*left)?;
            let right_value = eval(*right)?;

            // Evaluate the operation, possibly returning a type error. Coerce to float if needed.
            match (op, left_value, right_value) {
                (Add, Evaluated::Int(l), Evaluated::Int(r)) => Evaluated::Int(l + r),
                (Sub, Evaluated::Int(l), Evaluated::Int(r)) => Evaluated::Int(l - r),
                (Mul, Evaluated::Int(l), Evaluated::Int(r)) => Evaluated::Int(l * r),
                (Div, Evaluated::Int(l), Evaluated::Int(r)) => Evaluated::Int(l / r),
                (Exp, Evaluated::Int(l), Evaluated::Int(r)) => Evaluated::Int(l.pow(r as u32)),
                (ShiftLeft, Evaluated::Int(l), Evaluated::Int(r)) => Evaluated::Int(l << r),
                (ShiftRight, Evaluated::Int(l), Evaluated::Int(r)) => Evaluated::Int(l >> r),

                (ShiftLeft, _, _) | (ShiftRight, _, _) => {
                    return Err(EvalError::ShiftFloat);
                }

                (Add, Evaluated::Float(l), Evaluated::Float(r)) => Evaluated::Float(l + r),
                (Sub, Evaluated::Float(l), Evaluated::Float(r)) => Evaluated::Float(l - r),
                (Mul, Evaluated::Float(l), Evaluated::Float(r)) => Evaluated::Float(l * r),
                (Div, Evaluated::Float(l), Evaluated::Float(r)) => Evaluated::Float(l / r),
                (Exp, Evaluated::Float(l), Evaluated::Float(r)) => Evaluated::Float(l.powf(r)),

                (Add, Evaluated::Float(l), Evaluated::Int(r)) => Evaluated::Float(l + (r as f64)),
                (Sub, Evaluated::Float(l), Evaluated::Int(r)) => Evaluated::Float(l - (r as f64)),
                (Mul, Evaluated::Float(l), Evaluated::Int(r)) => Evaluated::Float(l * (r as f64)),
                (Div, Evaluated::Float(l), Evaluated::Int(r)) => Evaluated::Float(l / (r as f64)),
                (Exp, Evaluated::Float(l), Evaluated::Int(r)) => Evaluated::Float(l.powi(r as i32)),

                (Add, Evaluated::Int(l), Evaluated::Float(r)) => Evaluated::Float((l as f64) + r),
                (Sub, Evaluated::Int(l), Evaluated::Float(r)) => Evaluated::Float((l as f64) - r),
                (Mul, Evaluated::Int(l), Evaluated::Float(r)) => Evaluated::Float((l as f64) * r),
                (Div, Evaluated::Int(l), Evaluated::Float(r)) => Evaluated::Float((l as f64) / r),
                (Exp, Evaluated::Int(l), Evaluated::Float(r)) => {
                    Evaluated::Float((l as f64).powf(r))
                }
            }
        }

        IntCast { expr } => {
            let value = eval(*expr)?;
            match value {
                Evaluated::Int(i) => Evaluated::Int(i),
                Evaluated::Float(f) => Evaluated::Int(f as i128),
            }
        }

        FloatCast { expr } => {
            let value = eval(*expr)?;
            match value {
                Evaluated::Int(i) => Evaluated::Float(i as f64),
                Evaluated::Float(f) => Evaluated::Float(f),
            }
        }

        Expr::Value { val } => match val {
            Int(s) => {
                if &s[..2] == "0x" {
                    Evaluated::Int(i128::from_str_radix(&s[2..], 16).unwrap())
                } else {
                    Evaluated::Int(i128::from_str_radix(s, 10).unwrap())
                }
            }
            Float(s) => Evaluated::Float(s.parse::<f64>().unwrap()),
            PreviousResult => previous()?,
        },
    })
}

/// Attempt to read the previous result, parse it, and return it.
fn previous() -> Result<Evaluated, EvalError> {
    let value_str =
        std::fs::read_to_string(TEMP_PATH).map_err(|_| EvalError::UnableToReadPreviousValue)?;

    // The value will be of the form val,type
    let mut parts = value_str.split(',');

    let ty = parts.next();
    let value_str = parts.next();

    match (ty, value_str) {
        (Some("i"), Some(vs)) => vs
            .parse::<i128>()
            .map(Evaluated::Int)
            .map_err(|_| EvalError::CorruptPreviousValue),
        (Some("f"), Some(vs)) => vs
            .parse::<f64>()
            .map(Evaluated::Float)
            .map_err(|_| EvalError::CorruptPreviousValue),
        _ => Err(EvalError::CorruptPreviousValue),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_int {
        ($expr:expr) => {
            assert_eq!(
                eval(
                    *calculatrs::ExprParser::new()
                        .parse(stringify!($expr))
                        .unwrap()
                )
                .unwrap(),
                Evaluated::Int($expr)
            );
        };
    }

    macro_rules! test_float {
        ($expr:literal, $correct:expr) => {
            let val = eval(*calculatrs::ExprParser::new().parse($expr).unwrap()).unwrap();
            match val {
                Evaluated::Float(f) => assert!(f - $correct < 1E-10),
                Evaluated::Int(i) => panic!("Expected float {}, found int {}", $correct, i),
            }
        };
    }

    trait ToInt {
        fn to_int(self) -> i128;
    }

    trait ToFloat {
        fn to_float(self) -> f64;
    }

    impl ToInt for i128 {
        fn to_int(self) -> Self {
            self
        }
    }

    impl ToInt for f64 {
        fn to_int(self) -> i128 {
            self as i128
        }
    }

    fn int<I: ToInt>(i: I) -> i128 {
        i.to_int()
    }

    #[allow(unused_parens)]
    #[test]
    fn int_term() {
        test_int!(22);
        test_int!(-22);
        test_int!((22));
        test_int!((-22));
        #[rustfmt::skip]
        test_int!(((((22)))));
        test_int!(int(22.0));
        test_int!(int(22));
        test_int!(int(22.0 / 1.0));
        #[rustfmt::skip]
        test_int!(int((((22)))));
        assert!(calculatrs::ExprParser::new().parse("((22)").is_err());
    }

    #[test]
    fn int_expr() {
        test_int!(22 / 11 * 3 - (4 + -1) * (1 << 1) / (2 >> 1));
    }

    #[test]
    fn float_sci_not() {
        test_float!("2.0E2", 200.0);
        test_float!("-2.0E2", -200.0);
        test_float!("-2.0E-2", -0.02);
        test_float!("-0.2E3", -20.0);
        test_float!("-.2E3", -200.0);
        test_float!(".2E3", 200.0);
        test_float!("2E2", 200.0);
    }

    #[test]
    fn float_expr() {
        test_float!("2.0 ** 3", 8.0);
        test_float!("2.0 ** 3.0", 8.0);
        test_float!("1E1 ** 10", 10E10);

        test_float!(
            "22.0 / 11.0 * -3.0 * 1.5 + (4.0 + -1.0) ** float(1 << 1)",
            0.0
        );
    }
}
