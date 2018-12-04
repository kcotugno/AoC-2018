mod c01;

pub fn run(input: &[String], challenge: u8, part: u8) -> String {
    match challenge {
        1 => c01::run(input, part),
        _ => panic!("challenge not implemented"),
    }
}
