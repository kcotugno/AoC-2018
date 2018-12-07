use chrono::prelude::*;
use chrono::Duration;
use std::cmp::Ord;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Entry {
    pub id: i32,
    pub time: DateTime<Utc>,
    pub kind: EntryKind,
}

#[derive(Eq, PartialEq, Debug, Hash)]
enum EntryKind {
    Begin,
    Sleep,
    Wake,
}

impl FromStr for Entry {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Entry, Self::Err> {
        let mut id = 0;
        let mut date = String::new();
        let mut time = String::new();
        let mut kind = EntryKind::Wake;

        for (i, p) in s.split(' ').enumerate() {
            if i == 0 {
                date = p[1..].to_string();
            } else if i == 1 {
                time = format!("{}:00", &p[..p.len() - 1]);
            } else if i == 2 {
                kind = if p == "Guard" {
                    EntryKind::Begin
                } else if p == "falls" {
                    EntryKind::Sleep
                } else {
                    EntryKind::Wake
                };
            } else if i == 3 && kind == EntryKind::Begin {
                id = p[1..].parse().unwrap();
            }
        }

        Ok(Self {
            id,
            time: format!("{}T{}Z", date, time).parse().unwrap(),
            kind,
        })
    }
}

pub fn run(input: &[&str], part: u8) -> String {
    let mut parsed: Vec<Entry> = input.iter().map(|i| i.parse().unwrap()).collect();
    parsed.sort_unstable_by(|a, b| a.time.cmp(&b.time));
    match part {
        1 => c_04_1(parsed.as_slice()).to_string(),
        2 => c_04_2(parsed.as_slice()).to_string(),
        _ => panic!("unknown part"),
    }
}

fn c_04_1(input: &[Entry]) -> i32 {
    let mut log: HashMap<i32, i64> = HashMap::new();
    let mut times: HashMap<i32, HashMap<NaiveTime, i32>> = HashMap::new();
    let mut active = 0;

    let mut start = Utc.ymd(1000, 1, 1).and_hms(0, 0, 0);

    for e in input {
        match e.kind {
            EntryKind::Begin => active = e.id,
            EntryKind::Sleep => start = e.time,
            EntryKind::Wake => {
                if active == 0 {
                    continue;
                }
                let mut current = 0;
                if let Some(v) = log.get(&active) {
                    current = *v;
                }

                log.insert(active, current + (e.time - start).num_minutes());

                let mut minute = start;
                while minute < e.time {
                    let slept = times.entry(active).or_insert_with(HashMap::new);
                    let c = slept.entry(minute.time()).or_insert(0);
                    *c += 1;
                    minute = minute + Duration::minutes(1);
                }
            }
        };
    }

    let mut laziest_id = 0;
    let mut largest = 0;
    for (id, val) in log {
        if val > largest {
            laziest_id = id;
            largest = val;
        }
    }

    let mut laziest_count = 0;
    let mut laziest_time = 0;
    for (t, n) in &times[&laziest_id] {
        if *n > laziest_count {
            laziest_count = *n;
            laziest_time = t.format("%M").to_string().parse().unwrap();
        }
    }

    laziest_id * laziest_time
}

fn c_04_2(input: &[Entry]) -> i32 {
    let mut log: HashMap<i32, i64> = HashMap::new();
    let mut times: HashMap<i32, HashMap<NaiveTime, i32>> = HashMap::new();
    let mut active = 0;

    let mut start = Utc.ymd(1000, 1, 1).and_hms(0, 0, 0);

    for e in input {
        match e.kind {
            EntryKind::Begin => active = e.id,
            EntryKind::Sleep => start = e.time,
            EntryKind::Wake => {
                if active == 0 {
                    continue;
                }
                let mut current = 0;
                if let Some(v) = log.get(&active) {
                    current = *v;
                }

                log.insert(active, current + (e.time - start).num_minutes());

                let mut minute = start;
                while minute < e.time {
                    let slept = times.entry(active).or_insert_with(HashMap::new);
                    let c = slept.entry(minute.time()).or_insert(0);
                    *c += 1;
                    minute = minute + Duration::minutes(1);
                }
            }
        };
    }

    let mut laziest_count = 0;
    let mut laziest_time = 0;
    let mut laziest_id = 0;
    for (id, ts) in times {
        for (t, n) in ts {
            if n > laziest_count {
                laziest_id = id;
                laziest_count = n;
                laziest_time = t.format("%M").to_string().parse().unwrap();
            }
        }
    }

    laziest_id * laziest_time
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn c_04_1_examples() -> Result<(), String> {
        let mut examples: HashMap<&[&str], &str> = HashMap::new();
        examples.insert(
            &[
                "[1518-11-01 00:00] Guard #10 begins shift",
                "[1518-11-01 00:05] falls asleep",
                "[1518-11-01 00:25] wakes up",
                "[1518-11-01 00:30] falls asleep",
                "[1518-11-01 00:55] wakes up",
                "[1518-11-01 23:58] Guard #99 begins shift",
                "[1518-11-02 00:40] falls asleep",
                "[1518-11-02 00:50] wakes up",
                "[1518-11-03 00:05] Guard #10 begins shift",
                "[1518-11-03 00:24] falls asleep",
                "[1518-11-03 00:29] wakes up",
                "[1518-11-04 00:02] Guard #99 begins shift",
                "[1518-11-04 00:36] falls asleep",
                "[1518-11-04 00:46] wakes up",
                "[1518-11-05 00:03] Guard #99 begins shift",
                "[1518-11-05 00:45] falls asleep",
                "[1518-11-05 00:55] wakes up",
            ][..],
            "240",
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
    fn c_04_2_examples() -> Result<(), String> {
        let mut examples: HashMap<&[&str], &str> = HashMap::new();
        examples.insert(
            &[
                "[1518-11-01 00:00] Guard #10 begins shift",
                "[1518-11-01 00:05] falls asleep",
                "[1518-11-01 00:25] wakes up",
                "[1518-11-01 00:30] falls asleep",
                "[1518-11-01 00:55] wakes up",
                "[1518-11-01 23:58] Guard #99 begins shift",
                "[1518-11-02 00:40] falls asleep",
                "[1518-11-02 00:50] wakes up",
                "[1518-11-03 00:05] Guard #10 begins shift",
                "[1518-11-03 00:24] falls asleep",
                "[1518-11-03 00:29] wakes up",
                "[1518-11-04 00:02] Guard #99 begins shift",
                "[1518-11-04 00:36] falls asleep",
                "[1518-11-04 00:46] wakes up",
                "[1518-11-05 00:03] Guard #99 begins shift",
                "[1518-11-05 00:45] falls asleep",
                "[1518-11-05 00:55] wakes up",
            ][..],
            "4455",
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
