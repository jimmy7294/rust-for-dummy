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
    #[test]
    fn test_copy_str_function_v2() {
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
            assert_eq!(str_function_v2(str_case.s), str_case.res);
        }
    }
    #[test]
    fn test_copy_str_function_v3() {
        struct TestStr {
            s: String,
            res: (String, String),
        }

        fn create_test_case(s: String, res: (String, String)) -> TestStr {
            TestStr { s, res }
        }

        let str_cases = vec![
            create_test_case("1 2 3 4 5".to_string(), ("1 2 3 4 5".to_string(),"2.718281828459045 7.38905609893065 20.085536923187668 54.598150033144236 148.4131591025766".to_string())),
            create_test_case("1 0 4".to_string(), ("1 0 4".to_string(), "2.718281828459045 1 54.598150033144236".to_string() )),
        ];

        for case in str_cases {
            assert_eq!(str_function_v3(case.s), case.res);
        }
    }

    // borrow_me_the_reference.rs
    use crate::borrow_me_the_reference::*;
    #[test]
    fn test_borrow_delete_and_backspace() {
        struct Test {
            s: String,
            res: String,
        }

        let cases = vec![
            Test {
                s: String::from("bpp--o+er+++sskroi-++lcw"),
                res: String::from("borrow"),
            },
            Test {
                s: String::from("hs-+deasdasd------l+++dsdp"),
                res: String::from("help"),
            },
            Test {
                s: String::from("pad-rtic+eulqw--+rar"),
                res: String::from("particular"),
            },
            Test {
                s: String::from("--++++"),
                res: String::from(""),
            },
        ];

        for case in cases {
            let mut s = case.s.clone();
            delete_and_backspace(&mut s);
            assert_eq!(s, case.res);
        }
    }

    #[test]
    fn test_borrow_do_operations() {
        struct Test {
            s: Vec<String>,
            res: Vec<String>,
        }

        let cases = vec![Test {
            s: vec![
                "2+33".to_string(),
                "3+22".to_string(),
                "10-44".to_string(),
                "5+55".to_string(),
                "0+00".to_string(),
            ],
            res: vec![
                "35".to_string(),
                "25".to_string(),
                "-34".to_string(),
                "60".to_string(),
                "0".to_string(),
            ],
        }];

        for case in cases {
            let mut s = case.s.clone();
            do_operations(&mut s);
            assert_eq!(s, case.res);
        }
    }

    // tic_tac_toe.rs
    use crate::tic_tac_toe::*;
    pub struct Test<'a> {
        player: &'a str,
        table: Vec<Vec<&'a str>>,
        result: &'a str,
    }

    impl Test<'_> {
        fn init_horizontal() -> Vec<Test<'static>> {
            vec![
                Test {
                    player: "O",
                    table: vec![
                        vec!["O", "O", "O"],
                        vec!["X", "X", "O"],
                        vec!["O", "#", "X"],
                    ],
                    result: "player O won",
                },
                Test {
                    player: "O",
                    table: vec![
                        vec!["X", "X", "O"],
                        vec!["O", "O", "O"],
                        vec!["O", "#", "X"],
                    ],
                    result: "player O won",
                },
                Test {
                    player: "X",
                    table: vec![
                        vec!["O", "X", "O"],
                        vec!["O", "#", "O"],
                        vec!["X", "X", "X"],
                    ],
                    result: "player X won",
                },
                Test {
                    player: "X",
                    table: vec![
                        vec!["O", "X", "O", "O"],
                        vec!["X", "X", "X", "X"],
                        vec!["X", "#", "O", "X"],
                        vec!["X", "X", "O", "O"],
                    ],
                    result: "player X won",
                },
            ]
        }

        fn init_tie() -> Vec<Test<'static>> {
            vec![
                Test {
                    player: "none",
                    table: vec![
                        vec!["O", "X", "O"],
                        vec!["O", "X", "O"],
                        vec!["X", "#", "X"],
                    ],
                    result: "tie",
                },
                Test {
                    player: "none",
                    table: vec![
                        vec!["O", "X", "O"],
                        vec!["X", "X", "O"],
                        vec!["X", "#", "X"],
                    ],
                    result: "tie",
                },
                Test {
                    player: "none",
                    table: vec![
                        vec!["O", "X", "O", "O"],
                        vec!["X", "O", "X", "X"],
                        vec!["X", "#", "X", "X"],
                        vec!["X", "X", "O", "O"],
                    ],
                    result: "tie",
                },
            ]
        }
        fn init_vertical() -> Vec<Test<'static>> {
            vec![
                Test {
                    player: "O",
                    table: vec![
                        vec!["O", "X", "O"],
                        vec!["O", "X", "O"],
                        vec!["O", "#", "X"],
                    ],
                    result: "player O won",
                },
                Test {
                    player: "O",
                    table: vec![
                        vec!["X", "O", "O"],
                        vec!["X", "O", "O"],
                        vec!["#", "O", "X"],
                    ],
                    result: "player O won",
                },
                Test {
                    player: "X",
                    table: vec![
                        vec!["O", "X", "X"],
                        vec!["O", "X", "X"],
                        vec!["X", "#", "X"],
                    ],
                    result: "player X won",
                },
            ]
        }
        fn init_diagonals() -> Vec<Test<'static>> {
            vec![
                Test {
                    player: "X",
                    table: vec![
                        vec!["O", "O", "X"],
                        vec!["O", "X", "O"],
                        vec!["X", "#", "X"],
                    ],
                    result: "player X won",
                },
                Test {
                    player: "O",
                    table: vec![
                        vec!["O", "X", "O"],
                        vec!["X", "O", "O"],
                        vec!["X", "#", "O"],
                    ],
                    result: "player O won",
                },
                Test {
                    player: "O",
                    table: vec![
                        vec!["O", "X", "O", "O"],
                        vec!["X", "O", "X", "O"],
                        vec!["X", "#", "O", "X"],
                        vec!["X", "X", "O", "O"],
                    ],
                    result: "player O won",
                },
            ]
        }
    }

    #[test]
    fn test_diagonals() {
        assert_eq!(
            diagonals(
                "O",
                &vec![
                    vec!["O", "X", "O"],
                    vec!["O", "O", "X"],
                    vec!["X", "#", "O"],
                ]
            ),
            true
        );
        let new_tests = Test::init_diagonals();
        for v in new_tests {
            assert_eq!(diagonals(v.player, &v.table), true)
        }
    }
    #[test]
    fn test_horizontal() {
        let new_tests = Test::init_horizontal();
        for v in new_tests {
            assert_eq!(horizontal(v.player, &v.table), true)
        }

        assert_eq!(
            horizontal(
                "O",
                &vec![
                    vec!["O", "X", "X"],
                    vec!["O", "O", "X"],
                    vec!["X", "#", "O"]
                ]
            ),
            false
        );
    }

    #[test]
    fn test_vertical() {
        let new_tests = Test::init_vertical();
        for v in new_tests {
            assert_eq!(vertical(v.player, &v.table), true)
        }

        assert_eq!(
            vertical(
                "O",
                &vec![
                    vec!["O", "X", "X"],
                    vec!["O", "O", "X"],
                    vec!["X", "#", "O"]
                ]
            ),
            false
        );
    }

    #[test]
    fn test_tic_tac_toe() {
        let mut new_tests = Test::init_diagonals();
        new_tests.append(&mut Test::init_horizontal());
        new_tests.append(&mut Test::init_vertical());
        new_tests.append(&mut Test::init_tie());

        for v in new_tests {
            assert_eq!(tic_tac_toe(v.table), v.result.to_string());
        }
    }

    // arrange_it.rs
    use crate::arrange_it::*;
    #[test]
    fn test_arrange_phrase() {
        struct Test<'a> {
            phrase: &'a str,
            result: &'a str,
        }

        let cases = vec![Test {
            phrase: "is2 Thi1s T4est 3a",
            result: "This is a Test",
        },
        Test {
            phrase: "My1 formula2 for3 greatness4 in5 a6 human7 being8 is9 that10 one11 wants12 nothing14 be15 diff16erent, no17t forwa18rd, no19t backw20ard, no21t i22n a23ll eterni24ty.",
            result: "My formula for greatness in a human being is that one wants nothing be different, not forward, not backward, not in all eternity.",
        }];

        for case in cases {
            assert_eq!(arrange_phrase(case.phrase), case.result.to_string())
        }
    }
}
