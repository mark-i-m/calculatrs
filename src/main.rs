use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub calculatrs);

fn main() {
    let arg_str = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    match calculatrs::ExprResultParser::new().parse(&arg_str) {
        Ok(result) => println!("{}", result),
        Err(err) => {
            println!("{}", err);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_int {
        ($expr:expr) => {
            assert_eq!(
                calculatrs::IntExprParser::new()
                    .parse(stringify!($expr))
                    .unwrap(),
                $expr
            );
        };
    }

    macro_rules! test_float {
        ($expr:literal, $correct:expr) => {
            assert!(calculatrs::FloatExprParser::new().parse($expr).unwrap() - ($correct) < 1E-10);
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
        assert!(calculatrs::IntExprParser::new().parse("((22)").is_err());
    }

    #[test]
    fn int_expr() {
        test_int!(22 / 11 * 3 - (4 + -1) * (1 << 1) / (2 >> 1));
    }

    #[test]
    fn float_expr() {
        test_float!("2.0 ** 3", 8.0);
        test_float!("2.0 ** 3.0", 8.0);

        test_float!(
            "22.0 / 11.0 * -3.0 * 1.5 + (4.0 + -1.0) ** float(1 << 1)",
            0.0
        );
    }

    #[test]
    fn expr_result() {
        assert_eq!(
            calculatrs::ExprResultParser::new()
                .parse("((((22))))")
                .unwrap(),
            "22".to_owned()
        );
    }
}