use std::str::FromStr;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Claim {
    pub id: i32,
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Claim {
    fn union(&self, b: &Claim) -> Option<Claim> {
        if (self.x2 > b.x1 && self.x1 < b.x2) && (self.y2 > b.y1 && self.y1 < b.y2) {
            let x1 = if self.x1 > b.x1 { self.x1 } else { b.x1 };
            let x2 = if self.x2 < b.x2 { self.x2 } else { b.x2 };
            let y1 = if self.y1 > b.y1 { self.y1 } else { b.y1 };
            let y2 = if self.y2 < b.y2 { self.y2 } else { b.y2 };
            Some(Claim {
                id: 0,
                x1,
                x2,
                y1,
                y2,
            })
        } else {
            None
        }
    }
}

impl FromStr for Claim {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Claim, Self::Err> {
        let mut id = 0;
        let mut x1 = 0;
        let mut x2 = 0;
        let mut y1 = 0;
        let mut y2 = 0;

        for (i, p) in s.split(' ').enumerate() {
            if i == 0 {
                let mut s = &p[1..];
                id = s.parse().unwrap();
            } else if i == 2 {
                let mut s = p[..p.len() - 1].split(',');
                x1 = s.next().unwrap().parse().unwrap();
                y1 = s.next().unwrap().parse().unwrap();
            } else if i == 3 {
                let mut s = p.split('x');
                x2 = s.next().unwrap().parse().unwrap();
                y2 = s.next().unwrap().parse().unwrap();
                x2 += x1;
                y2 += y1;
            }
        }

        Ok(Self { id, x1, x2, y1, y2 })
    }
}

pub fn run(input: &[&str], part: u8) -> String {
    let parsed: Vec<Claim> = input.iter().map(|i| i.parse().unwrap()).collect();
    match part {
        1 => c_03_1(parsed.as_slice()).to_string(),
        2 => c_03_2(parsed.as_slice()).to_string(),
        _ => panic!("unknown part"),
    }
}

fn c_03_1(input: &[Claim]) -> i32 {
    let mut grid: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];
    let mut total = 0;

    for claim in input.iter() {
        for row in grid
            .iter_mut()
            .take(claim.x2 as usize)
            .skip(claim.x1 as usize)
        {
            for cell in row
                .iter_mut()
                .take(claim.y2 as usize)
                .skip(claim.y1 as usize)
            {
                *cell += 1;
            }
        }
    }

    for row in grid {
        for cell in row {
            if cell > 1 {
                total += 1
            }
        }
    }
    total
}

fn c_03_2(input: &[Claim]) -> i32 {
    for claim in input {
        let mut found = false;

        for claim2 in input {
            if claim == claim2 {
                continue;
            }

            if claim.union(claim2).is_some() {
                found = true;
                break;
            }
        }

        if !found {
            return claim.id;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn c_03_1_examples() -> Result<(), String> {
        let mut examples = HashMap::new();
        examples.insert(
            &["#1 @ 1,3: 4x4", "#2 $ 3,1: 4x4", "#3 @ 5,5: 2x2"][..],
            "4",
        );

        for (input, out) in examples.iter() {
            let actual = run(input, 1);
            if &actual != out {
                return Err(format!("Got: {}, Want: {}", actual, out));
            }
        }

        Ok(())
    }

    #[test]
    fn c_03_2_examples() -> Result<(), String> {
        let mut examples = HashMap::new();
        examples.insert(
            &["#1 @ 1,3: 4x4", "#2 $ 3,1: 4x4", "#3 @ 5,5: 2x2"][..],
            "3",
        );

        for (input, out) in examples.iter() {
            let actual = run(input, 2);
            if &actual != out {
                return Err(format!("Got: {}, Want: {}", actual, out));
            }
        }

        Ok(())
    }
}
