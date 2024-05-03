use builder_m4cro::Builder;

#[derive(Builder, Debug)]
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
        .build().unwrap();

    print!("t1: {:#?}", t1);

    let t2 = TestType::builder()
        .a_unsigned(24)
        .a_opt_unsigned(13)
        .a_string("xxx")
        .build().unwrap();

    print!("t1: {:#?}", t2);
}

#[cfg(test)]
mod tests {
    use builder_m4cro::Builder;


    #[test]
    fn test_05() {
        #[derive(PartialEq, Debug, Clone)]
        pub struct Dummy {
            a: String,
            u: u32,
        }

        #[derive(Builder, PartialEq, Debug)]
        pub struct TestType {
            pub o_s: Option<Dummy>,
            pub s: Dummy,
        }

        let t1_1 = TestType::builder()
        .s(Dummy{ a: "x".to_string(), u: 44})
        .build().unwrap();

        let t1_2 = TestType {
            o_s: None,
            s: Dummy{ a: "x".to_string(), u: 44},
        };

        assert_eq!(t1_1, t1_2);

        let t2_1 = TestType::builder()
        .o_s(Dummy{ a: "yyy".to_string(), u: 49})
        .s(Dummy{ a: "x".to_string(), u: 44})
        .build().unwrap();

        let t2_2 = TestType {
            o_s: Some(Dummy{ a: "yyy".to_string(), u: 49}),
            s: Dummy{ a: "x".to_string(), u: 44},
        };
        assert_eq!(t2_1, t2_2);

        let t1_3 = TestType::builder()
        .build();

        assert!(t1_3.is_err());

    }


    #[test]
    fn test_04() {
        #[derive(Builder, PartialEq, Debug)]
        pub struct TestType {
            pub o_v: Option<Vec<u32>>,
            pub v: Vec<u32>,
        }

        let t1_1 = TestType::builder()
        .o_v(vec![1, 2, 3])
        .v(vec![7, 6, 8])
        .build().unwrap();

        let t1_2 = TestType {
            o_v: Some(vec![1, 2, 3]),
            v: vec![7, 6, 8],
        };

        assert_eq!(t1_1, t1_2);

        let t2_1 = TestType::builder()
        .v(vec![7, 6, 8])
        .build().unwrap();

        let t2_2 = TestType {
            o_v: None,
            v: vec![7, 6, 8],
        };

        assert_eq!(t2_1, t2_2);
    }


    #[test]
    fn test_03() {
        #[derive(Builder, PartialEq, Debug)]
        pub struct TestType {
            pub a_unsigned: Option<u32>,
            pub a_string: Option<String>,
        }

        let t1_1 = TestType::builder()
        .build().unwrap();

        let t1_2 = TestType {
            a_unsigned: None,
            a_string: None,
        };

        assert_eq!(t1_1, t1_2);

        let t2_1 = TestType::builder()
        .a_string("xxx".to_string())
        .a_unsigned(5)
        .build().unwrap();

        let t2_2 = TestType {
            a_unsigned: Some(5),
            a_string: Some("xxx".to_string()),
        };

        assert_eq!(t2_1, t2_2);
    }

    #[test]
    fn test_02() {
        #[derive(Builder, PartialEq, Debug)]
        pub struct TestType {
            pub a_unsigned: u32,
            pub a_string: String,
        }

        let t1_1 = TestType::builder()
        .a_unsigned(23)
        .a_string("xxx")
        .build().unwrap();

        let t1_2 = TestType {
            a_unsigned: 23,
            a_string: "xxx".to_string(),
        };

        assert_eq!(t1_1, t1_2);
    }

    #[test]
    fn test_01() {

        #[derive(Builder, PartialEq, Debug)]
        pub struct TestType {
            pub a_unsigned: u32,
            pub a_opt_unsigned: Option<u32>,
            pub a_string: String,
            pub a_opt_string: Option<String>,
        }

        let t1_1 = TestType::builder()
        .a_unsigned(23)
        .a_string("xxx")
        .build().unwrap();

        let t1_2 = TestType {
            a_unsigned: 23,
            a_opt_unsigned: None,
            a_string: "xxx".to_string(),
            a_opt_string: None,
        };

        assert_eq!(t1_1, t1_2);

        let t2_1 = TestType::builder()
        .a_unsigned(3)
        .a_opt_string("yyy".to_string())
        .a_string("xxx")
        .build().unwrap();

        let t2_2 = TestType {
            a_unsigned: 3,
            a_opt_unsigned: None,
            a_string: "xxx".to_string(),
            a_opt_string: Some("yyy".to_string()),
        };
        assert_eq!(t2_1, t2_2);


        let t3_1 = TestType::builder()
        .a_unsigned(113)
        .a_opt_unsigned(12)
        .a_string("xxx")
        .build().unwrap();

        let t3_2 = TestType {
            a_unsigned: 113,
            a_opt_unsigned: Some(12),
            a_string: "xxx".to_string(),
            a_opt_string: None,
        };
        assert_eq!(t3_1, t3_2);

    }

}
