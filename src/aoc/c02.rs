use std::collections::HashMap;

pub fn run(input: &[&str], part: u8) -> String {
    match part {
        1 => c_02_1(input).to_string(),
        2 => c_02_2(input),
        _ => panic!("unknown part"),
    }
}

fn c_02_1(input: &[&str]) -> i32 {
    let mut three = 0;
    let mut two = 0;
    for val in input {
        let mut m = HashMap::new();

        for c in val.chars() {
            let new_val =match m.get(&c) {
                Some(v) => *v+1,
                None => 1
            };

            m.insert(c.clone(), new_val);
        }

        three += match m.iter().filter(|(_, v)| **v == 3).next() {
            Some(_) => 1,
            None => 0
        };
        two += match m.iter().filter(|(_, v)| **v == 2).next() {
            Some(_) => 1,
            None => 0
        };
    }

    three * two
}

fn c_02_2(input: &[&str]) -> String {
    for val1 in input {
        for val2 in input {
            let mut diff = 0;
            let mut val2char = val2.chars();
            for val1char in val1.chars() {
                match val2char.next() {
                    Some(v) => {
                        if v != val1char {
                            diff += 1;
                            if diff > 1 {
                                break;
                            }
                        }
                    },
                    None => ()
                }
            }

            if diff == 1 {
                let mut s = String::new();
                for (i, c) in val1.chars().enumerate() {
                    if c == val2.chars().nth(i).unwrap() {
                        s.push(c);
                    }
                }

                return s;
            }
        }
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn c_02_1_examples() -> Result<(), String> {
        let mut examples: HashMap<&[&str], i32> = HashMap::new();
        examples.insert(
            &[
                "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
            ],
            12,
        );

        for (input, out) in examples.iter() {
            let actual = c_02_1(input);
            if &actual != out {
                return Err(format!("Got: {}, Want: {}", actual, out));
            }
        }

        Ok(())
    }

    #[test]
    fn c_02_2_examples() -> Result<(), String> {
        let mut examples: HashMap<&[&str], &str> = HashMap::new();
        examples.insert(
            &[
                "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
            ],
            "fgij",
        );

        for (input, out) in examples.iter() {
            let actual = c_02_2(input);
            if &actual != out {
                return Err(format!("Got: {}, Want: {}", actual, out));
            }
        }

        Ok(())
    }
}
