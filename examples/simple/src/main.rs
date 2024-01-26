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
        .a_string("xxx".to_string())
        .build().unwrap();

    print!("t1: {:#?}", t1);

    let t2 = TestType::builder()
        .a_unsigned(24)
        .a_opt_unsigned(Some(13))
        .a_string("xxx".to_string())
        .build().unwrap();

    print!("t1: {:#?}", t2);
}
