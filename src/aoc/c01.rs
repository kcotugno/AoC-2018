use std::collections::HashSet;

pub fn run(input: &[&str], part: u8) -> String {
    let encoded: Vec<_> = input.iter().map(|i| i.parse::<i32>().unwrap()).collect();
    match part {
        1 => c_01_1(&encoded),
        2 => c_01_2(&encoded),
        _ => panic!("unknown part"),
    }.to_string()
}

fn c_01_1(input: &[i32]) -> i32 {
    let mut n = 0;
    for x in input {
        n += x;
    }
    n
}

fn c_01_2(input: &[i32]) -> i32 {
    let mut freq = HashSet::new();
    freq.insert(0);
    let mut n = 0;
    loop {
        for x in input {
            n += x;
            if freq.contains(&n) {
                return n;
            } else {
                freq.insert(n);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn c_01_1_examples() -> Result<(), String> {
        let mut examples: HashMap<&[i32], i32> = HashMap::new();
        examples.insert(&[1, 1, 1], 3);
        examples.insert(&[1, 1, -2], 0);
        examples.insert(&[-1, -2, -3], -6);

        for (input, out) in examples.iter() {
            let actual = c_01_1(input);
            if &actual != out {
                return Err(format!("Got: {}, Want: {}", actual, out));
            }
        }

        Ok(())
    }

    #[test]
    fn c_01_2_examples() -> Result<(), String> {
        let mut examples: HashMap<&[i32], i32> = HashMap::new();
        examples.insert(&[1, -1], 0);
        examples.insert(&[3, 3, 4, -2, -4], 10);
        examples.insert(&[-6, 3, 8, 5, -6], 5);
        examples.insert(&[7, 7, -2, -7, -4], 14);

        for (input, out) in examples.iter() {
            let actual = c_01_2(input);
            if &actual != out {
                return Err(format!("Got: {}, Want: {}", actual, out));
            }
        }

        Ok(())
    }
}
