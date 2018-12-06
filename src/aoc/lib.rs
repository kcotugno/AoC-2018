mod c01;
mod c02;
mod c03;

pub fn run(input: &[&str], challenge: u8, part: u8) -> String {
    match challenge {
        1 => c01::run(input, part),
        2 => c02::run(input, part),
        3 => c03::run(input, part),
        _ => panic!("challenge not implemented"),
    }
}
