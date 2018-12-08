pub fn run(input: &[&str], part: u8) -> String {
    match part {
        1 => c_05_1(input[0]).to_string(),
        2 => c_05_2(input[0]).to_string(),
        _ => panic!("unknown part"),
    }
}

fn c_05_1(input: &str) -> i32 {
    let mut stack: Vec<u8> = Vec::with_capacity(input.len());

    for c in input.chars() {
        let b = c as u8;
        match stack.pop() {
            Some(v) => {
                if v != b + 0x20 && v != b - 0x20 {
                    stack.push(v);
                    stack.push(b);
                }
            }
            None => stack.push(b),
        };
    }

    stack.len() as i32
}

fn c_05_2(input: &str) -> i32 {
    let mut stack: Vec<u8> = Vec::with_capacity(input.len());
    let mut shortest = input.len();

    for l in 0x41..0x5B as u8 {
        for c in input.chars() {
            let b = c as u8;
            if b == l || b == l + 0x20 {
                continue;
            }

            match stack.pop() {
                Some(v) => {
                    if v != b + 0x20 && v != b - 0x20 {
                        stack.push(v);
                        stack.push(b);
                    }
                }
                None => stack.push(b),
            };
        }

        if stack.len() < shortest {
            shortest = stack.len();
        }
        stack.clear();
    }

    shortest as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn c_05_1_examples() -> Result<(), String> {
        let mut examples: HashMap<&[&str], &str> = HashMap::new();
        examples.insert(&["dabAcCaCBAcCcaDA"][..], "10");

        for (input, out) in examples.iter() {
            let actual = run(input, 1);
            if &actual != out {
                return Err(format!("Got: {}, Want: {}", actual, out));
            }
        }

        Ok(())
    }

    #[test]
    fn c_05_2_examples() -> Result<(), String> {
        let mut examples: HashMap<&[&str], &str> = HashMap::new();
        examples.insert(&["dabAcCaCBAcCcaDA"][..], "4");

        for (input, out) in examples.iter() {
            let actual = run(input, 2);
            if &actual != out {
                return Err(format!("Got: {}, Want: {}", actual, out));
            }
        }

        Ok(())
    }
}
