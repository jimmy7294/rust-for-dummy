#[cfg(test)]
mod tests {
    use crate::borrow::*;
    use crate::doubtful::*;

    #[test]
    fn test_borrow() {
        assert_eq!(str_len("Compiling"), 9);
        assert_eq!(str_len(&"Solarion".to_string()), 8);
    }

    #[test]
    fn test_doubtful() {
        let mut s = String::from("Aurilius");
        doubtful(&mut s);
        assert_eq!(s, "Aurilius😀")
    }

    use crate::to_url::to_url;
    #[test]
    fn test_to_url() {
        let s = "The impediment of action advances action. What stands in the way becomes the way.";
        assert_eq!(to_url(s), "The%20impediment%20of%20action%20advances%20action.%20What%20stands%20in%20the%20way%20becomes%20the%20way.");
    }

    use crate::string_literals::*;
    #[test]
    fn test_string_literals() {
        assert_eq!(is_empty(""), true);
        assert_eq!(is_empty("Rust"), false);
        assert_eq!(
            is_ascii("You're not everything you could be. And you know it."),
            true
        );
        assert_eq!(is_ascii(" 😁"), false);
        assert_eq!(contains("Face the demands of life voluntarily. Respond to a challenge, instead of bracing for catastrophe.", "challenge"), true);
        assert_eq!(contains("Face the demands of life voluntarily. Respond to a challenge, instead of bracing for catastrophe.", "love"), false);
        assert_eq!(split_at("You can do an awful lot by writing down what happened to you and thinking it through", 10), ("You can do", " an awful lot by writing down what happened to you and thinking it through"));
        assert_eq!(find("You can do an awful lot by writing down what happened to you and thinking it through", 'w'), 15);
    }

    use crate::name_initials::*;

    #[test] // this is called an attribute
    fn test_name_initials() {
        // Define Test struc
        struct Test {
            // here we define a struct named Test that has 2 fields, both of them are vectors of string literals
            names: Vec<&'static str>,
            result: Vec<&'static str>,
        }
        let cases = vec![
            Test {
                names: vec![
                    "Roger Federer",
                    "Rafael Nadal",
                    "Novak Djokovic",
                    "Carlos Alcaraz",
                    "Janik Sinner",
                ],
                result: vec!["R. F.", "R. N.", "N. D.", "C. A.", "J. S."],
            },
            Test {
                names: vec![
                    "Chris Hemsworth",
                    "Cris Evans",
                    "Robert Downey",
                    "Mark Ruffalo",
                    "Scarlett Johansson",
                    "Jeremy Renners",
                    "Sofia Reyes",
                ],
                result: vec![
                    "C. H.", "C. E.", "R. D.", "M. R.", "S. J.", "J. R.", "S. R.",
                ],
            },
        ];
        for case in cases {
            assert_eq!(initials(case.names), case.result);
        }
    }

    use crate::ownership::*;
    #[test]
    fn test_owership() {
        struct Test<'a> {
            s: String,
            result: &'a str,
        }
        let cases = vec![
            Test {
                s: "helloFromTheOtherSide".to_string(),
                result: "hello",
            },
            Test {
                s: "how_you_doin".to_string(),
                result: "how",
            },
            Test {
                s: "Winteriscoming".to_string(),
                result: "Winteriscoming",
            },
            Test {
                s: "TheimpedimentOfActionAdvancesAction".to_string(),
                result: "Theimpediment",
            },
        ];

        for case in cases {
            assert_eq!(first_subword(case.s), case.result.to_string());
        }
    }

    // copy.rs
    use crate::copy::*;
    #[test]
    fn test_copy_nbr_function() {
        struct TestNbr {
            n: i32,
            result: (i32, f64, f64),
        }
        let nbr_cases = vec![
            TestNbr {
                n: 0,
                result: (0, 1.0, f64::NEG_INFINITY),
            },
            TestNbr {
                n: 1,
                result: (1, 2.718281828459045, 0.0),
            },
            TestNbr {
                n: 2,
                result: (2, 2.0_f64.exp(), 2.0_f64.ln()),
            },
            TestNbr {
                n: -1,
                result: (-1, (-1.0_f64).exp(), 1.0_f64.ln()),
            },
            TestNbr {
                n: -100,
                result: (-100, (-100.0_f64).exp(), 100.0_f64.ln()),
            },
        ];
        for nbr_case in nbr_cases {
            assert_eq!(nbr_function(nbr_case.n), nbr_case.result);
        }
    }
    #[test]
    fn test_copy_str_function() {
        struct TestStr {
            s: String,
            res: (String, String),
        }

        let str_cases = vec![
            TestStr {
                s: "1 2 3 4 5".to_string(),
                res: ("1 2 3 4 5".to_string(), "2.718281828459045 7.38905609893065 20.085536923187668 54.598150033144236 148.4131591025766".to_string()),
            },
            TestStr {
                s: "1 0 4".to_string(),
                res: ("1 0 4".to_string(), "2.718281828459045 1 54.598150033144236".to_string()),
            },
        ];

        for str_case in str_cases {
            assert_eq!(str_function(str_case.s), str_case.res);
        }
    }

    #[test]
    fn test_copy_vec_function() {
        fn create_test_case(v: Vec<i32>, res: (Vec<i32>, Vec<f64>)) -> TestVec {
            TestVec {
                // Test instance
                v,   //input
                res, //expected output
            }
        }
        struct TestVec {
            v: Vec<i32>,
            res: (Vec<i32>, Vec<f64>),
        }

        let vec_cases = vec![
            create_test_case(
                [0, 1].to_vec(),
                ([0, 1].to_vec(), [f64::NEG_INFINITY, 1.0_f64.ln()].to_vec()),
            ),
            create_test_case(
                [2, 4].to_vec(),
                ([2, 4].to_vec(), [2.0_f64.ln(), 4.0_f64.ln()].to_vec()),
            ),
            create_test_case(
                [5, 7].to_vec(),
                ([5, 7].to_vec(), [5.0_f64.ln(), 7.0_f64.ln()].to_vec()),
            ),
            create_test_case(
                [-1, -2].to_vec(),
                ([-1, -2].to_vec(), [(1.0_f64).ln(), (2.0_f64.ln())].to_vec()),
            ),
        ];
        for vec_case in vec_cases {
            assert_eq!(vec_function(vec_case.v), vec_case.res);
        }
    }
}
