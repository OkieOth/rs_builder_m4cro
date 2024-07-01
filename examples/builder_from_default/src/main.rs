use builder_m4cro::BuilderFromDefault;


#[derive(BuilderFromDefault, Debug, Default)]
pub struct TestType {
    pub a_unsigned: u32,
    pub a_opt_unsigned: Option<u32>,
    pub a_string: String,
    pub a_opt_string: Option<String>,
}

impl TestType {
    pub fn dummy() {
        print!("dummy");
    }
}

pub fn main() {
    let t1 = TestType::builder()
        .a_unsigned(23)
        .a_string("xxx")
        .build();

    print!("t1: {:#?}", t1);

    let t2 = TestType::builder()
        .a_unsigned(24)
        .a_opt_unsigned(13)
        .a_string("xxx")
        .build();

    print!("t2: {:#?}", t2);

    let t3 = TestType::builder()
        .a_unsigned(24)
        .a_opt_unsigned(13)
        .a_opt_string("xxx")
        .build();

    print!("t3: {:#?}", t3);
}

#[cfg(test)]
mod tests {
    use builder_m4cro::BuilderFromDefault;


    #[test]
    fn test_05_1() {
        #[derive(PartialEq, Debug, Clone, Default)]
        pub struct Dummy {
            a: String,
            u: u32,
        }

        #[derive(BuilderFromDefault, PartialEq, Debug, Default)]
        pub struct TestType {
            pub o_s: Option<Dummy>,
            pub s: Dummy,
        }

        let t1_1 = TestType::builder()
        .s(Dummy{ a: "x".to_string(), u: 44})
        .build();

        let t1_2 = TestType {
            o_s: None,
            s: Dummy{ a: "x".to_string(), u: 44},
        };

        assert_eq!(t1_1, t1_2);

        let t2_1 = TestType::builder()
        .build();

        let t2_2 = TestType {
            o_s: None,
            s: Dummy{ a: "".to_string(), u: 0},
        };
        assert_eq!(t2_1, t2_2);
    }

    #[test]
    fn test_05_2() {
        #[derive(PartialEq, Debug, Clone)]
        pub struct Dummy {
            a: String,
            u: u32,
        }

        impl Default for Dummy {
            fn default() -> Self {
                Dummy{ a: "my default string".to_string(), u: 0}
            }
        }

        #[derive(BuilderFromDefault, PartialEq, Debug)]
        pub struct TestType {
            pub o_s: Option<Dummy>,
            pub s: Dummy,
            pub i: u32,
        }

        impl Default for TestType {
            fn default() -> Self {
                TestType {
                    o_s: Some(Dummy{ a: "xxx".to_string(), u: 10}),
                    s: Dummy::default(),
                    i: 0,
                }
            }
        }

        let t1_1 = TestType::builder()
        .i(100)
        .build();

        let t1_2 = TestType {
            o_s: Some(Dummy {a: "xxx".to_string(), u: 10}),
            s: Dummy{ a: "my default string".to_string(), u: 0},
            i: 100,
        };

        assert_eq!(t1_1, t1_2);
    }

    #[test]
    fn test_04() {
        #[derive(BuilderFromDefault, PartialEq, Debug, Default)]
        pub struct TestType {
            pub o_v: Option<Vec<u32>>,
            pub v: Vec<u32>,
        }

        let t1_1 = TestType::builder()
        .o_v(vec![1, 2, 3])
        .v(vec![7, 6, 8])
        .build();

        let t1_2 = TestType {
            o_v: Some(vec![1, 2, 3]),
            v: vec![7, 6, 8],
        };

        assert_eq!(t1_1, t1_2);

        let t2_1 = TestType::builder()
        .build();

        let t2_2 = TestType {
            o_v: None,
            v: vec![],
        };

        assert_eq!(t2_1, t2_2);
    }

}
